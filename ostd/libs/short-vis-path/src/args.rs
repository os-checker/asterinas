use proc_macro2::{Group, Span, TokenStream, TokenTree};
use quote::ToTokens;
use std::collections::BTreeMap;
use syn::{parse::Parse, punctuated::Punctuated, *};

pub enum Argument {
    Single(Ident),
    Override(Ident, Path),
}

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

pub struct AddArguments {
    pub args: BTreeMap<Ident, Path>,
}

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
                                 Please choose a corrent short module name.",
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

impl AddArguments {
    /// Replace `pub(in subsystem)` by `pub(in crate::to::subsystem)`.
    pub fn replace_restricted_vis_path(&self, vis: &mut Visibility) {
        if let Visibility::Restricted(vis) = vis
            && let Some(input) = vis.path.get_ident()
            && let Some(path) = self.args.get(input)
        {
            vis.path = Box::clone_from_ref(path);
        }
    }

    /// Parse `pub(in ident) ...`.
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

/// Module path to be replaced with, at best effort of guess basis on source file layout.
struct ExpandedPath {
    /// Starting from `crate`.
    segment: Vec<String>,
    /// Callsite span.
    callsite_span: Span,
}

impl ExpandedPath {
    /// Get the module path to be used in `pub(in ...)`, based on directory structure.
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

    /// Generate the Path tokens, starting from `crate` to `end` (both included).
    /// Returns None when the module path doesn't contain `end`.
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
