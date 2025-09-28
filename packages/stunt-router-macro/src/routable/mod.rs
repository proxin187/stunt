//! This entire module is kind of clusterfucked and unreadable, atleast it works :)

use syn::parse::{Parse, ParseStream, Error, Result};
use syn::{DeriveInput, Data, LitStr, Ident, Attribute, Fields, Field};
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

    fn validate(&self, fields: &Fields) -> bool {
        match self {
            PathNode::Static(_) => true,
            PathNode::Segment(segment) => fields.iter().any(|field| field.ident.clone().map(|ident| ident.to_string() == segment.to_string()).unwrap_or_default()),
        }
    }

    fn pattern(&self) -> proc_macro2::TokenStream {
        match self {
            PathNode::Static(string) => quote! { #string },
            PathNode::Segment(ident) => quote! { #ident },
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            PathNode::Static(string) => string.is_empty(),
            PathNode::Segment(_) => false,
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

    fn validate(&self, fields: &Fields) -> bool {
        self.path.iter()
            .all(|path| path.validate(fields))
    }

    fn pattern(&self) -> proc_macro2::TokenStream {
        let paths = self.path.iter()
            .map(|path| path.pattern());

        quote! {
            #[deny(unused_variables)]
            [#(#paths),*]
        }
    }

    fn path(&self) -> proc_macro2::TokenStream {
        if self.path.iter().all(|path| path.is_empty()) {
            quote! { [String::from("/")] }
        } else {
            let paths = self.path.iter()
                .map(|path| {
                    let pattern = path.pattern();

                    quote! { format!("{}/", #pattern) }
                });

            quote! {
                [#(#paths),*]
            }
        }
    }
}

enum AttributeKind {
    At {
        path: Path,
        fields: Fields,
    },
    NotFound,
}

impl AttributeKind {
    fn new(attr: &Attribute, fields: Fields) -> Result<AttributeKind> {
        if attr.path().is_ident("at") {
            let literal: LitStr = attr.parse_args::<LitStr>()?;
            let path = Path::new(literal.value())?;

            if path.validate(&fields) {
                Ok(AttributeKind::At {
                    path,
                    fields,
                })
            } else {
                Err(Error::new(attr.path().span(), "Fields and path dont match"))
            }
        } else if attr.path().is_ident("not_found") {
            Ok(AttributeKind::NotFound)
        } else {
            Err(Error::new(attr.path().span(), "An `at` or `not_found` attribute must be present"))
        }
    }

    fn map_path(&self, f: impl Fn(&Path) -> proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        match self {
            AttributeKind::At { path, .. } => f(path),
            AttributeKind::NotFound => quote! { _ },
        }
    }

    fn condition(&self) -> Option<proc_macro2::TokenStream> {
        match self {
            AttributeKind::At { fields, .. } if !fields.is_empty() => {
                let tokens = fields.iter()
                    .map(|field| {
                        let ty = &field.ty;
                        let ident = &field.ident;

                        quote! { <#ty as ::std::str::FromStr>::from_str(#ident).is_ok() }
                    });

                Some(quote! {
                    if #(#tokens)&&*
                })
            },
            _ => None,
        }
    }

    fn map_fields(&self, f: impl Fn(&Field) -> proc_macro2::TokenStream) -> Option<proc_macro2::TokenStream> {
        match self {
            AttributeKind::At { fields, .. } => {
                let tokens = fields.iter().map(f);

                Some(quote! {
                    #(#tokens),*
                })
            },
            AttributeKind::NotFound => None,
        }
    }
}

struct Variant {
    ident: Ident,
    attributes: Vec<AttributeKind>,
}

impl Variant {
    pub fn new(ident: Ident, attributes: Vec<AttributeKind>) -> Variant {
        Variant {
            ident,
            attributes,
        }
    }

    fn route_tokens(&self) -> proc_macro2::TokenStream {
        let ident = &self.ident;

        self.attributes.iter()
            .map(|attribute| {
                let pattern = attribute.map_path(|path| path.pattern());
                let condition = attribute.condition();
                let fields = attribute.map_fields(|field| {
                    let ident = &field.ident;

                    quote! { #ident: std::str::FromStr::from_str(#ident).expect("internal error") }
                });

                quote! { #pattern #condition => Self::#ident { #fields }, }
            })
            .collect()
    }

    fn path_tokens(&self) -> proc_macro2::TokenStream {
        let ident = &self.ident;

        let attributes = self.attributes.iter()
            .filter(|attribute| !matches!(attribute, AttributeKind::NotFound))
            .map(|attribute| {
                let pattern = attribute.map_path(|path| path.path());
                let fields = attribute.map_fields(|field| {
                    let ident = &field.ident;

                    quote! { #ident }
                });

                quote! { Self::#ident { #fields } => #pattern.concat(), }
            });

        quote! { #(#attributes),*}
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
                    let attributes = variant.attrs.iter()
                        .filter(|attr| attr.path().is_ident("at") || attr.path().is_ident("not_found"))
                        .map(|attr| AttributeKind::new(attr, variant.fields.clone()))
                        .collect::<Result<Vec<AttributeKind>>>()?;

                    if !attributes.is_empty() {
                        variants.push(Variant::new(variant.ident.clone(), attributes));
                    } else {
                        return Err(Error::new(variant.ident.span(), "An `at` or `not_found` attribute must be present"));
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

        let route_variants = self.variants.iter()
            .map(|variant| variant.route_tokens());

        let path_variants = self.variants.iter()
            .map(|variant| variant.path_tokens());

        quote! {
            impl ::stunt_router::Routable for #ident {
                fn route(__path: &[&str]) -> Self {
                    match __path {
                        #(#route_variants)*
                    }
                }

                fn path(self) -> String {
                    match self {
                        #(#path_variants)*
                    }
                }
            }
        }
    }
}


