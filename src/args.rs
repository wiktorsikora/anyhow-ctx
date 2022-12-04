use proc_macro2::TokenStream;
use quote::quote;
use std::marker::PhantomData;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, LitStr, Token};

#[derive(Default, Debug)]
pub(crate) struct ContextArgs {
    context: Option<Expr>,
    fmt: Option<LitStr>,
}

impl ContextArgs {
    pub fn get_context(self) -> TokenStream {
        let Self { context, fmt } = self;

        if let Some(ctx) = context {
            return quote!(#ctx);
        }
        if let Some(fmt) = fmt {
            return quote!(format!(#fmt));
        }
        panic!("expected `context` or `fmt` arguments")
    }
}

struct CtxArg<Keyword: Parse, Value: Parse> {
    value: Value,
    _p: PhantomData<Keyword>,
}

impl<Keyword: Parse, Value: Parse> Parse for CtxArg<Keyword, Value> {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let _ = input.parse::<Keyword>()?;
        let _ = input.parse::<Token![=]>()?;
        let value = input.parse()?;
        Ok(Self {
            value,
            _p: PhantomData,
        })
    }
}

impl Parse for ContextArgs {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut args = Self::default();
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(keyword::context) {
                if args.context.is_some() {
                    return Err(input.error("expected only a single `context` argument"));
                }
                let context = input.parse::<CtxArg<keyword::context, Expr>>()?.value;
                args.context = Some(context);
            } else if lookahead.peek(keyword::fmt) {
                if args.fmt.is_some() {
                    return Err(input.error("expected only a single `fmt` argument"));
                }
                let fmt = input.parse::<CtxArg<keyword::fmt, LitStr>>()?.value;
                args.fmt = Some(fmt);
            } else {
                return Err(input.error("unexpected arguments"));
            }
        }
        Ok(args)
    }
}

mod keyword {
    syn::custom_keyword!(context);
    syn::custom_keyword!(fmt);
}
