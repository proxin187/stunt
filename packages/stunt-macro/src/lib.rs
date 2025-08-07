#![feature(proc_macro_diagnostic)]

mod tags;

use proc_macro2::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data, Fields};
use quote::quote;

use tags::{Intermediate, Tag};

use std::sync::{LazyLock, Arc, Mutex};
use std::iter::Peekable;


// TODO: this function is ugly beyond all imagination, refactor it so that it looks acceptable at least
fn generate<'a>(tags: &mut Peekable<impl Iterator<Item = &'a Tag>>, is_root: bool) -> TokenStream {
    match tags.next() {
        Some(Tag::OpenTag(open)) => {
            let mut inner: Vec<TokenStream> = Vec::new();

            while let Some(tag) = tags.peek() {
                match tag {
                    Tag::CloseTag(close) if close.name == open.name => {
                        tags.next();

                        break;
                    },
                    Tag::CloseTag(_) => {
                        tag.span()
                            .error(format!("mismatched closing tag, expected </{}>", open.name))
                            .emit();

                        tags.next();

                        return TokenStream::new();
                    },
                    _ => inner.push(generate(tags, false)),
                }
            }

            let name = open.name.clone();
            let str_name = name.to_string();
            let nodes: TokenStream = inner.into_iter().collect();

            let events = open.events.iter()
                .map(|event| event.tokens())
                .collect::<TokenStream>();

            let attributes = open.attributes.iter()
                .map(|attribute| attribute.tokens())
                .collect::<TokenStream>();

            let generics = open.generics.clone();

            let mut tokens = name.to_string().chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
                .then(|| quote! {
                    ::stunt::component::tree::Tree::new(
                        ::stunt::component::tree::ComponentRef::Element(::stunt::component::tree::Element::new(String::from(#str_name), vec![#attributes], vec![#nodes])),
                        vec![#events],
                        Vec::new(),
                        Vec::new(),
                    )
                })
                .unwrap_or_else(|| quote! {
                    ::stunt::component::tree::Tree::new(
                        ::stunt::component::tree::ComponentRef::create_component::<#name<#(#generics),*>>(),
                        vec![#events],
                        vec![#attributes],
                        vec![#nodes],
                    )
                });

            if !is_root {
                tokens.extend(quote! {,});
            }

            TokenStream::from(tokens)
        },
        Some(Tag::Template(template)) => {
            let block = template.value.clone();

            let mut tokens = quote! {
                ::stunt::component::tree::Tree::new(
                    #[allow(unused_braces)]
                    ::stunt::component::tree::ComponentRef::Template(std::sync::Arc::new(#block)),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                )
            };

            if !is_root {
                tokens.extend(quote! {,});
            }

            TokenStream::from(tokens)
        },
        Some(tag) => {
            tag.span()
                .error("expected a open tag")
                .emit();

            TokenStream::new()
        },
        None => TokenStream::new(),
    }
}

#[proc_macro]
pub fn html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let intermediate: Intermediate = parse_macro_input!(input as Intermediate);

    proc_macro::TokenStream::from(generate(&mut intermediate.tags.iter().peekable(), true))
}

#[proc_macro_derive(Properties)]
pub fn properties_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            let fields = fields.named.iter().map(|field| match &field.ident {
                Some(ident) => {
                    let name = &ident;
                    let key = ident.to_string();
                    let ty = &field.ty;

                    quote! {
                        #name: attributes.get::<#ty>(#key).unwrap_or_default()
                    }
                },
                None => quote! {},
            });

            let name = input.ident;

            return proc_macro::TokenStream::from(quote! {
                impl ::stunt::component::Properties for #name {
                    fn create(attributes: AttrMap) -> Self {
                        #name {
                            #(#fields),*
                        }
                    }
                }
            });
        }
    }

    proc_macro::TokenStream::from(syn::Error::new(input.ident.span(), "You can only derive Properties for Structs with Named fields").to_compile_error())
}


