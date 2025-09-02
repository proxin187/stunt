use syn::parse::{Parse, ParseStream, Error, Result};
use syn::{DeriveInput, Data, LitStr, Ident, Attribute};
use syn::spanned::Spanned;

use proc_macro2::Span;

use quote::quote;


enum PathNode {
    Static(String),
    Segment(Ident),
}

impl PathNode {
    pub fn new(mut node: String) -> PathNode {
        node.starts_with(':')
            .then(|| PathNode::Segment(Ident::new(&node.split_off(1), Span::call_site())))
            .unwrap_or_else(|| PathNode::Static(node.to_string()))
    }

    pub fn pattern(&self) -> proc_macro2::TokenStream {
        match self {
            PathNode::Static(string) => quote! { #string },
            PathNode::Segment(ident) => {
                quote! { #ident }
            },
        }
    }
}

struct Path {
    path: Vec<PathNode>,
}

impl Path {
    pub fn new(literal: String) -> Result<Path> {
        let path = literal.split('/')
            .map(|node| PathNode::new(node.to_string()))
            .collect::<Vec<PathNode>>();

        Ok(Path {
            path,
        })
    }

    pub fn pattern(&self) -> proc_macro2::TokenStream {
        let paths = self.path.iter()
            .map(|path| path.pattern());

        quote! {
            [#(#paths),*]
        }
    }
}

enum VariantKind {
    At(Path),
    NotFound,
}

impl VariantKind {
    pub fn new(attr: &Attribute) -> Result<VariantKind> {
        if attr.path().is_ident("at") {
            let literal: LitStr = attr.parse_args::<LitStr>()?;
            let path = Path::new(literal.value())?;

            Ok(VariantKind::At(path))
        } else if attr.path().is_ident("not_found") {
            Ok(VariantKind::NotFound)
        } else {
            Err(Error::new(attr.path().span(), "An `at` or `not_found` attribute must be present"))
        }
    }

    pub fn pattern(&self) -> proc_macro2::TokenStream {
        match self {
            VariantKind::At(path) => path.pattern(),
            VariantKind::NotFound => quote! { _ },
        }
    }
}

struct Variant {
    ident: Ident,
    kind: VariantKind,
}

impl Variant {
    pub fn new(ident: Ident, kind: VariantKind) -> Variant {
        Variant {
            ident,
            kind,
        }
    }

    pub fn tokens(&self) -> proc_macro2::TokenStream {
        let pattern = self.kind.pattern();
        let ident = &self.ident;

        quote! {
            #pattern => #ident {}
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
                    match variant.attrs.iter().filter(|attr| attr.path().is_ident("at") || attr.path().is_ident("not_found")).next() {
                        Some(attr) => {
                            variants.push(Variant::new(variant.ident.clone(), VariantKind::new(attr)?));
                        },
                        None => {
                            return Err(Error::new(variant.ident.span(), "An `at` or `not_found` attribute must be present"));
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
        let ident = &self.ident;

        let variants = self.variants.iter()
            .map(|variant| variant.tokens());

        quote! {
            impl ::stunt_router::Routable for #ident {
                fn route(__path: &[&str]) -> #ident {
                    match __path {
                        #(#variants),*

                        ["api", "account", id, name] if <usize as std::str::FromStr>::from_str(id).is_ok() && <String as std::str::FromStr>::from_str(name).is_ok() => Route::Account {
                            id: std::str::FromStr::from_str(id).expect("internal error"),
                            name: std::str::FromStr::from_str(name).expect("internal error"),
                        },
                        _ => Route::NotFound,
                    }
                }
            }
        }
    }
}


