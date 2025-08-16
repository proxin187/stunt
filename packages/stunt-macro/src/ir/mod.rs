use crate::tags::{Event, Attribute, Tag, OpenTag};

use syn::{Ident, Type, ExprBlock};

use proc_macro2::Span;

use std::iter::Peekable;


pub struct Template {
    pub expr: ExprBlock,
}

impl Template {
    pub fn new(expr: ExprBlock) -> Template {
        Template {
            expr,
        }
    }
}

pub struct Node {
    pub name: Ident,
    pub events: Vec<Event>,
    pub attributes: Vec<Attribute>,
    pub generics: Vec<Type>,
    pub children: Vec<Kind>,
}

impl Default for Node {
    fn default() -> Node {
        Node {
            name: Ident::new("", Span::call_site()),
            events: Vec::default(),
            attributes: Vec::default(),
            generics: Vec::default(),
            children: Vec::default(),
        }
    }
}

impl Node {
    fn new<'a>(tags: &mut Peekable<impl Iterator<Item = &'a Tag>>, open: OpenTag) -> Node {
        let mut children: Vec<Kind> = Vec::new();

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

                    return Node::default();
                },
                _ => {
                    let ir = Ir::new(tags);

                    children.extend(ir.nodes);
                },
            }
        }

        Node {
            name: open.name,
            events: open.events,
            attributes: open.attributes,
            generics: open.generics,
            children,
        }
    }
}

pub enum Kind {
    Node(Node),
    Template(Template),
}

pub struct Ir {
    pub nodes: Vec<Kind>,
}

impl Ir {
    pub fn new<'a>(tags: &mut Peekable<impl Iterator<Item = &'a Tag>>) -> Ir {
        let mut nodes: Vec<Kind> = Vec::new();

        match tags.next() {
            Some(Tag::OpenTag(open)) => {
                nodes.push(Kind::Node(Node::new(tags, open.clone())));
            },
            Some(Tag::Template(template)) => {
                nodes.push(Kind::Template(Template::new(template.value.clone())));
            },
            Some(tag) => {
                tag.span()
                    .error("expected an open tag or template")
                    .emit();
            },
            None => {},
        }

        Ir {
            nodes,
        }
    }
}


