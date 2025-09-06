use syn::parse::{Parse, ParseStream, Result};
use syn::{Token, Ident, ExprBlock, Type};
use syn::spanned::Spanned;

use proc_macro2::TokenStream;
use proc_macro::Span;

use quote::quote;


#[derive(Clone)]
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

#[derive(Clone)]
pub struct Attribute {
    name: Ident,
    value: ExprBlock,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Attribute> {
        let name: Ident = input.parse()?;

        input.parse::<Token![=]>()?;

        let value: ExprBlock = input.parse()?;

        Ok(Attribute {
            name,
            value,
        })
    }
}

impl Attribute {
    pub fn element_tokens(&self) -> TokenStream {
        let name = format!("{}", self.name);
        let value = &self.value;

        quote! {
            #[allow(unused_braces)]
            (::std::string::String::from(#name), ::std::rc::Rc::new(#value) as ::std::rc::Rc<dyn ::std::fmt::Display>),
        }
    }

    pub fn component_tokens(&self) -> TokenStream {
        let name = &self.name;
        let value = &self.value;

        quote! {
            #[allow(unused_braces)]
            let __stunt_token = builder.#name(__stunt_token, #value);
        }
    }
}

#[derive(Clone)]
pub struct OpenTag {
    pub name: Ident,
    pub generics: Vec<Type>,
    pub attributes: Vec<Attribute>,
    pub events: Vec<Event>,
    pub closed: bool,
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

        while !input.peek(Token![>]) && !input.peek(Token![/]) {
            if input.fork().parse::<Ident>().map(|ident| ident.to_string().starts_with("on")).unwrap_or_default() {
                events.push(input.parse::<Event>()?);
            } else {
                attributes.push(input.parse::<Attribute>()?);
            }
        }

        let closed = input.peek(Token![/])
            .then(|| { let _ = input.parse::<Token![/]>(); true })
            .unwrap_or_default();

        input.parse::<Token![>]>()?;

        Ok(OpenTag {
            name,
            generics,
            attributes,
            events,
            closed,
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


