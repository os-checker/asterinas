#![feature(clone_from_ref)]

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{visit_mut::VisitMut, *};

mod args;

#[proc_macro_attribute]
pub fn add(attr: TokenStream, item: TokenStream) -> TokenStream {
    if attr.is_empty() {
        // Do nothing if argument hasn't been provided yet.
        return item;
    }

    let mut args = syn::parse_macro_input!(attr as args::AddArguments);
    let mut file = syn::parse_macro_input!(item as File);

    args.visit_file_mut(&mut file);

    file.into_token_stream().into()
}
