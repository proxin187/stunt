use syn::parse::{Parse, ParseStream, Result};
use syn::{Token, Ident, ExprBlock};
use syn::spanned::Spanned;

use proc_macro2::TokenStream;
use proc_macro::Span;

use quote::quote;


mod keyword {
    syn::custom_keyword!(template);
    syn::custom_keyword!(event);
}

pub struct Event {
    pub name: Ident,
    pub value: ExprBlock,
}

impl Parse for Event {
    fn parse(input: ParseStream) -> Result<Event> {
        input.parse::<keyword::event>()?;
        input.parse::<Token![:]>()?;

        let name: Ident = input.parse()?;

        input.parse::<Token![=]>()?;

        let value: ExprBlock = input.parse()?;

        Ok(Event {
            name,
            value,
        })
    }
}

impl Event {
    pub fn tokens(&self) -> TokenStream {
        let name = format!("{}", self.name);
        let value = self.value.clone();

        quote! {
            (String::from(#name), #value),
        }
    }
}

pub struct Attribute {
    pub name: Ident,
    pub value: ExprBlock,
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
    pub fn tokens(&self) -> TokenStream {
        let name = format!("{}", self.name);
        let value = self.value.clone();

        quote! {
            #[allow(unused_braces)]
            (String::from(#name), String::from(#value)),
        }
    }
}

pub struct OpenTag {
    pub name: Ident,
    pub attributes: Vec<Attribute>,
    pub events: Vec<Event>,
}

impl Parse for OpenTag {
    fn parse(input: ParseStream) -> Result<OpenTag> {
        let name: Ident = input.parse()?;

        let mut attributes: Vec<Attribute> = Vec::new();
        let mut events: Vec<Event> = Vec::new();

        while !input.peek(Token![>]) {
            if input.peek(keyword::event) {
                events.push(input.parse::<Event>()?);
            } else {
                attributes.push(input.parse::<Attribute>()?);
            }
        }

        input.parse::<Token![>]>()?;

        Ok(OpenTag {
            name,
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
        input.parse::<Token![/]>()?;

        let name: Ident = input.parse()?;

        input.parse::<Token![>]>()?;

        Ok(CloseTag {
            name,
        })
    }
}

impl CloseTag {
    pub fn new(name: Ident) -> CloseTag {
        CloseTag {
            name,
        }
    }
}

pub struct Template {
    pub value: ExprBlock,
}

impl Parse for Template {
    fn parse(input: ParseStream) -> Result<Template> {
        input.parse::<keyword::template>()?;

        let value: ExprBlock = input.parse::<ExprBlock>()?;

        input.parse::<Token![/]>()?;
        input.parse::<Token![>]>()?;

        Ok(Template {
            value,
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
        input.parse::<Token![<]>()?;

        if input.peek(keyword::template) {
            Ok(Tag::Template(input.parse::<Template>()?))
        } else if input.peek(Token![/]) {
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


