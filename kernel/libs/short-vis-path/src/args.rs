// SPDX-License-Identifier: MPL-2.0

use std::collections::BTreeMap;

use proc_macro2::{Group, Span, TokenStream, TokenTree};
use quote::ToTokens;
use syn::{parse::Parse, punctuated::Punctuated, *};

/// Represents a single argument in the `#[add(...)]` attribute.
/// Either a simple identifier or an override with an explicit path.
pub enum Argument {
    Single(Ident),
    Override(Ident, Path),
}

/// Parses `Argument` from token stream.
/// Accepts either a single identifier or `ident = path` format.
impl Parse for Argument {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        Ok(if input.peek2(Token![=]) {
            let ident: Ident = input.parse()?;
            let _: Token![=] = input.parse()?;
            let path: Path = input.parse()?;
            Argument::Override(ident, path)
        } else {
            let ident: Ident = input.parse()?;
            Argument::Single(ident)
        })
    }
}

/// Holds the parsed arguments from `#[add(...)]`.
/// Maps each identifier to its corresponding path.
pub struct AddArguments {
    pub args: BTreeMap<Ident, Path>,
}

/// Parses the `#[add(...)]` attribute content.
/// Expects a comma-separated list of identifiers, optionally with path overrides.
impl Parse for AddArguments {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        // Parse multiple arguments.
        let args = Punctuated::<Argument, Token![,]>::parse_terminated(input)?;

        // Default module path inferred from file path.
        let path = ExpandedPath::new();

        Ok(AddArguments {
            args: args
                .into_iter()
                .map(|arg| match arg {
                    Argument::Single(ident) => {
                        let Some(tokens) = path.to_syn_path(&ident) else {
                            panic!(
                                "The path `{}` doesn't contain `{ident}`. \
                                 Please choose a correct short module name.",
                                path.segment.join("::")
                            )
                        };
                        (ident, tokens)
                    }
                    Argument::Override(ident, path) => (ident, path.clone()),
                })
                .collect(),
        })
    }
}

/// Implements VisitMut to transform visibility paths in AST nodes.
impl visit_mut::VisitMut for AddArguments {
    fn visit_visibility_mut(&mut self, vis: &mut Visibility) {
        self.replace_restricted_vis_path(vis);
    }

    fn visit_item_mut(&mut self, item: &mut Item) {
        if let Item::Verbatim(ts) = item {
            // Syn doesn't support parsing `pub(in path) macro` yet.
            self.replace_verbatim_vis_path(ts);
            return;
        }
        visit_mut::visit_item_mut(self, item);
    }
}

/// Provides methods for replacing short visibility paths with full paths.
impl AddArguments {
    /// Replaces `pub(in subsystem)` with `pub(in crate::to::subsystem)`.
    /// Only affects visibility restricted to identifiers registered in `self.args`.
    fn replace_restricted_vis_path(&self, vis: &mut Visibility) {
        if let Visibility::Restricted(vis) = vis
            && let Some(input) = vis.path.get_ident()
            && let Some(path) = self.args.get(input)
        {
            vis.path = Box::clone_from_ref(path);
        }
    }

    /// Parses and replaces visibility paths in verbatim token streams.
    /// Handles `pub(in ident)` syntax that syn cannot parse normally.
    fn replace_verbatim_vis_path(&self, ts: &mut TokenStream) {
        let mut v_tt: Vec<TokenTree> = ts.clone().into_iter().collect();
        let mut iter = v_tt.iter_mut();
        if let Some(TokenTree::Ident(ident)) = iter.next()
            && ident == "pub"
            && let Some(TokenTree::Group(group)) = iter.next()
        {
            let mut new_stream = TokenStream::new();
            let mut stream = group.stream().into_iter();
            if let Some(in_) = stream.next()
                && let TokenTree::Ident(ident) = &in_
                && ident == "in"
            {
                new_stream.extend([in_]);

                let path_stream = stream.collect::<TokenStream>();
                if let Ok(input) = parse2::<Ident>(path_stream)
                    && let Some(path) = self.args.get(&input)
                {
                    path.to_tokens(&mut new_stream);
                    *group = Group::new(group.delimiter(), new_stream);
                }
            }
            *ts = TokenStream::from_iter(v_tt);
        }
    }
}

/// Represents the full module path derived from the source file location.
/// Used to replace short visibility paths with properly qualified paths.
struct ExpandedPath {
    /// Module path segments starting from `crate`.
    segment: Vec<String>,
    /// Span for maintaining original source location in generated tokens.
    callsite_span: Span,
}

impl ExpandedPath {
    /// Constructs the full module path based on the source file location.
    /// The path starts from `crate` and follows the directory structure.
    /// For example, if the attribute is in `a/src/procfs.rs`, this function returns
    /// `crate::procfs`; if in `a/src/fs/procfs/mod.rs`, returns `crate::fs::procfs`.
    fn new() -> Self {
        let callsite_span = Span::call_site();
        let Some(local_path) = callsite_span.local_file() else {
            panic!("Unknown local file path to call site span {callsite_span:?}.");
        };
        let Ok(local_path) = local_path.canonicalize() else {
            panic!("Unable to canonicalize {local_path:?}.")
        };

        let manifest_dir =
            std::env::var("CARGO_MANIFEST_DIR").expect("Failed to get manifest dir.");

        let Ok(relative_path) = local_path.strip_prefix(&manifest_dir) else {
            panic!("{manifest_dir:?} must be a prefix of {local_path:?}.")
        };

        let Ok(module_path) = relative_path.strip_prefix("src") else {
            panic!("`src/` must be a prefix of {relative_path:?}.")
        };

        let module_str = module_path.to_str().unwrap();
        // Handle `xx/mod_name/mod.rs` module style.
        let module_str = module_str.strip_suffix("/mod.rs").unwrap_or(module_str);
        // Handle `xx/mod_name.rs` module style.
        let module_str = module_str.strip_suffix(".rs").unwrap_or(module_str);

        ExpandedPath {
            segment: std::iter::once("crate")
                .chain(
                    std::path::Path::new(module_str)
                        .iter()
                        .map(|m| m.to_str().unwrap()),
                )
                .map(String::from)
                .collect(),
            callsite_span,
        }
    }

    /// Generates a `Path` from `crate` up to and including the segment matching `end`.
    /// Returns `None` if `end` is not found in the module path.
    fn to_syn_path(&self, end: &Ident) -> Option<Path> {
        let pos = self.segment.iter().rposition(|seg| end == seg.as_str())?;
        Some(Path {
            leading_colon: None,
            segments: self.segment[..pos + 1]
                .iter()
                .map(|s| PathSegment::from(Ident::new(s, self.callsite_span)))
                .collect(),
        })
    }
}
