use syn::{Expr, Token};
use syn::parse::{Parse, ParseStream};

#[derive(Default, Debug)]
pub (crate) struct ContextArgs {
    context: Option<Expr>
}

impl ContextArgs {
    pub fn get_context(self) -> Expr {
        self.context.expect("missing context")
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
                let _ = input.parse::<keyword::context>()?;
                let _ = input.parse::<Token![=]>()?;
                let context = input.parse::<Expr>()?;
                args.context = Some(context);
            } else {
                panic!("unexpected arguments")
            }
        }
        Ok(args)
    }
}

mod keyword {
    syn::custom_keyword!(context);
}