use syn::parse::{Parse, ParseStream, Error, Result};
use syn::{DeriveInput, Data, LitStr, Ident};

use quote::quote;


enum PathNode {
    Static(String),
    Segment,
}

struct Path {
    path: Vec<PathNode>,
}

impl Path {
    pub fn new(literal: String) -> Result<Path> {
        todo!()
    }
}

struct Variant {
    ident: Ident,
    path: Path,
}

impl Variant {
    pub fn new(ident: Ident, path: Path) -> Variant {
        Variant {
            ident,
            path,
        }
    }
}

pub struct Routable {
    ident: Ident,
    variants: Vec<Variant>,
}

impl Parse for Routable {
    fn parse(stream: ParseStream) -> Result<Routable> {
        let input: DeriveInput = stream.parse()?;

        match input.data {
            Data::Enum(data) => {
                let mut variants: Vec<Variant> = Vec::new();

                for variant in data.variants {
                    match variant.attrs.iter().filter(|attr| attr.path().is_ident("at")).next() {
                        Some(attr) => {
                            let literal: LitStr = attr.parse_args::<LitStr>()?;
                            let path = Path::new(literal.value())?;

                            variants.push(Variant::new(variant.ident.clone(), path));
                        },
                        None => {
                            return Err(Error::new(variant.ident.span(), "An `at` attribute must be present"));
                        },
                    }
                }

                Ok(Routable {
                    ident: input.ident,
                    variants,
                })
            },
            _ => Err(Error::new(input.ident.span(), "You can only derive Routable for an Enum")),
        }
    }
}

impl Routable {
    pub fn tokens(&self) -> proc_macro2::TokenStream {
        quote! {}
    }
}


