//!  Set the error [`context`] using doc comments.
//!
//! This is useful because instead of writing manual error messages to provide context to an error, it
//! automatically derives it from doc comments. This works especially well for async contexts, where
//! stack traces may not be persisted past yield points and thread boundaries. But contexts do.
//!
//! [`context`]: https://docs.rs/failure/0.1.5/failure/trait.ResultExt.html#tymethod.context
//!
//! ## Examples
//!
//! ```rust
//! use anyhow_ctx::with_context;
//! use failure::{ensure, ResultExt};
//!
//! /// Square a number if it's less than 10.
//! #[context]
//! fn square(num: usize) -> Result<usize, failure::Error> {
//!     ensure!(num < 10, "Number was too large");
//!     Ok(num * num)
//! }
//!
//! fn main() -> Result<(), failure::Error> {
//!     let args = std::env::args();
//!     ensure!(args.len() == 2, "usage: square <num>");
//!     let input = args.skip(1).next().unwrap().parse()?;
//!
//!     println!("result is {}", square(input)?);
//!
//!     Ok(())
//! }
//! ```
//!
//! ```sh
//! $ cargo run --example square 12
//! Error: ErrorMessage { msg: "Number was too large" }
//! Square a number if it's less than 10.
//! ```

#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(missing_docs, missing_doc_code_examples)]
#![cfg_attr(test, deny(warnings))]
#![recursion_limit = "512"]

mod args;

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote};
use syn::{ItemFn, Signature};
use crate::args::ContextArgs;

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
        output: return_type,
        inputs: params,
        unsafety,
        asyncness,
        constness,
        abi,
        ident,
        generics:
        syn::Generics {
            params: gen_params,
            where_clause,
            ..
        },
        ..
    } = sig;

    let args: Vec<_> = params.pairs().filter_map(|pair| {
        match pair.into_value() {
            syn::FnArg::Typed(arg) => Some(arg.pat.clone()),
            _ => return None,
        }
    }).collect();

    let res = quote!(
        #(#attrs) *
        #vis #constness #unsafety #asyncness #abi fn #ident<#gen_params>(#params) #return_type
        #where_clause
        {
            #vis #constness #unsafety #asyncness #abi fn #ident<#gen_params>(#params) #return_type {
                #block
            }
            use anyhow::Context;
            Ok(#ident(#(#args)*).with_context(|| #context)?)
        }
    );
    Ok(res)
}
