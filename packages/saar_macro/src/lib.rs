use proc_macro::TokenStream;

use syn::parse::{Parse, ParseStream, Result};
use syn::{Token, Ident, Expr};


struct Attribute {
    name: Ident,
    value: Expr,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Attribute> {
        Ok(Attribute {
            name: input.parse(),
            value: input.parse(),
        })
    }
}

struct Tag {
    name: Ident,
    attributes: Vec<Attribute>,
}

#[proc_macro]
pub fn html(tokens: TokenStream) {
}


