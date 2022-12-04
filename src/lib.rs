#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(missing_docs, missing_doc_code_examples)]
#![cfg_attr(test, deny(warnings))]
#![recursion_limit = "512"]

mod args;

extern crate proc_macro;

use crate::args::ContextArgs;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, Signature};

#[proc_macro_attribute]
pub fn with_context(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(attr as ContextArgs);
    add_context(args, item).unwrap().into()
}

fn add_context(
    args: ContextArgs,
    item: TokenStream,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let context = args.get_context();

    let input = syn::parse::<ItemFn>(item)?;

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;

    let Signature {
        inputs: params,
        asyncness,
        ident,
        ..
    } = &sig;

    let fn_args: Vec<_> = params
        .pairs()
        .filter_map(|pair| match pair.into_value() {
            syn::FnArg::Typed(arg) => Some(arg.pat.clone()),
            _ => return None,
        })
        .collect();

    let fn_call = if asyncness.is_some() {
        quote!(#ident(#(#fn_args)*).await)
    } else {
        quote!(#ident(#(#fn_args)*))
    };

    let res = quote!(
        #(#attrs) *
        #vis #sig
        {
            #sig {
                #block
            }
            use anyhow::Context;
            Ok(#fn_call.with_context(|| #context)?)
        }
    );
    Ok(res)
}
