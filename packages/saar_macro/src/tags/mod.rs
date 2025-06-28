use syn::parse::{Parse, ParseStream, Result};
use syn::{Token, Ident, ExprBlock};


mod keyword {
    syn::custom_keyword!(template);
    syn::custom_keyword!(event);
}

pub struct Attribute {
    pub event: bool,
    pub name: Ident,
    pub value: ExprBlock,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Attribute> {
        let event = input.peek(keyword::event);

        if event {
            input.parse::<keyword::event>()?;
            input.parse::<Token![:]>()?;
        }

        let name: Ident = input.parse()?;

        input.parse::<Token![=]>()?;

        let value: ExprBlock = input.parse()?;

        Ok(Attribute {
            event,
            name,
            value,
        })
    }
}

pub struct OpenTag {
    pub name: Ident,
    pub attributes: Vec<Attribute>,
}

impl Parse for OpenTag {
    fn parse(input: ParseStream) -> Result<OpenTag> {
        let name: Ident = input.parse()?;

        let mut attributes: Vec<Attribute> = Vec::new();

        while !input.peek(Token![>]) {
            attributes.push(input.parse::<Attribute>()?);
        }

        input.parse::<Token![>]>()?;

        Ok(OpenTag {
            name,
            attributes,
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

struct Template {
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


