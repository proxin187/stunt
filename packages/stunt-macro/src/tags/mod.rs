use syn::parse::{Parse, ParseStream, Result};
use syn::{Token, Ident, ExprBlock, Type};
use syn::spanned::Spanned;

use proc_macro2::TokenStream;
use proc_macro::Span;

use quote::quote;


mod keyword {
    syn::custom_keyword!(event);
}

pub struct Event {
    pub name: String,
    pub value: ExprBlock,
}

impl Parse for Event {
    fn parse(input: ParseStream) -> Result<Event> {
        let name: Ident = input.parse()?;

        input.parse::<Token![=]>()?;

        let value: ExprBlock = input.parse()?;

        Ok(Event {
            name: name.to_string().split_off(2),
            value,
        })
    }
}

impl Event {
    pub fn tokens(&self) -> TokenStream {
        let name = self.name.clone();
        let value = self.value.clone();

        quote! {
            #[allow(unused_braces)]
            (String::from(#name), std::sync::Arc::new(#value)),
        }
    }
}

pub enum Attribute {
    Multiple {
        expr: ExprBlock,
    },
    Single {
        name: Ident,
        value: ExprBlock,
    },
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Attribute> {
        if input.peek(Token![?]) {
            input.parse::<Token![?]>()?;

            let expr: ExprBlock = input.parse()?;

            Ok(Attribute::Multiple {
                expr,
            })
        } else {
            let name: Ident = input.parse()?;

            input.parse::<Token![=]>()?;

            let value: ExprBlock = input.parse()?;

            Ok(Attribute::Single {
                name,
                value,
            })
        }
    }
}

impl Attribute {
    pub fn tokens(&self) -> TokenStream {
        match self {
            Attribute::Multiple { expr } => {
                let expr = expr.clone();

                quote! { #expr }
            },
            Attribute::Single { name, value } => {
                let name = format!("{}", name);
                let value = value.clone();

                quote! {
                    vec![#[allow(unused_braces)](String::from(#name), std::rc::Rc::new(#value))],
                }
            },
        }
    }
}

pub struct OpenTag {
    pub name: Ident,
    pub generics: Vec<Type>,
    pub attributes: Vec<Attribute>,
    pub events: Vec<Event>,
}

impl Parse for OpenTag {
    fn parse(input: ParseStream) -> Result<OpenTag> {
        input.parse::<Token![<]>()?;

        let name: Ident = input.parse()?;

        let mut generics: Vec<Type> = Vec::new();

        if input.peek(Token![<]) {
            input.parse::<Token![<]>()?;

            while !input.peek(Token![>]) {
                generics.push(input.parse()?);

                if !input.peek(Token![>]) {
                    input.parse::<Token![,]>()?;
                }
            }

            input.parse::<Token![>]>()?;
        }

        let mut attributes: Vec<Attribute> = Vec::new();
        let mut events: Vec<Event> = Vec::new();

        while !input.peek(Token![>]) {
            if input.fork().parse::<Ident>().map(|ident| ident.to_string().starts_with("on")).unwrap_or_default() {
                events.push(input.parse::<Event>()?);
            } else {
                attributes.push(input.parse::<Attribute>()?);
            }
        }

        input.parse::<Token![>]>()?;

        Ok(OpenTag {
            name,
            generics,
            attributes,
            events,
        })
    }
}

pub struct CloseTag {
    pub name: Ident,
}

impl Parse for CloseTag {
    fn parse(input: ParseStream) -> Result<CloseTag> {
        input.parse::<Token![<]>()?;
        input.parse::<Token![/]>()?;

        let name: Ident = input.parse()?;

        input.parse::<Token![>]>()?;

        Ok(CloseTag {
            name,
        })
    }
}

pub struct Template {
    pub value: ExprBlock,
}

impl Parse for Template {
    fn parse(input: ParseStream) -> Result<Template> {
        Ok(Template {
            value: input.parse::<ExprBlock>()?,
        })
    }
}

pub enum Tag {
    OpenTag(OpenTag),
    CloseTag(CloseTag),
    Template(Template),
}

impl Parse for Tag {
    fn parse(input: ParseStream) -> Result<Tag> {
        if input.peek(syn::token::Brace) {
            Ok(Tag::Template(input.parse::<Template>()?))
        } else if input.peek(Token![<]) && input.peek2(Token![/]) {
            Ok(Tag::CloseTag(input.parse::<CloseTag>()?))
        } else {
            Ok(Tag::OpenTag(input.parse::<OpenTag>()?))
        }
    }
}

impl Tag {
    pub fn span(&self) -> Span {
        match self {
            Tag::OpenTag(tag) => tag.name.span().unwrap(),
            Tag::CloseTag(tag) => tag.name.span().unwrap(),
            Tag::Template(tag) => tag.value.span().unwrap(),
        }
    }
}

pub struct Intermediate {
    pub tags: Vec<Tag>,
}

impl Parse for Intermediate {
    fn parse(input: ParseStream) -> Result<Intermediate> {
        let mut tags: Vec<Tag> = Vec::new();

        while !input.is_empty() {
            tags.push(input.parse::<Tag>()?);
        }

        Ok(Intermediate {
            tags,
        })
    }
}


