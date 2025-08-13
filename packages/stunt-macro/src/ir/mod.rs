use crate::tags::{Event, Attribute, Tag, OpenTag};

use syn::{Ident, Type, ExprBlock};

use proc_macro2::Span;

use std::iter::Peekable;


pub struct Template {
    expr: ExprBlock,
}

impl Template {
    pub fn new(expr: ExprBlock) -> Template {
        Template {
            expr,
        }
    }
}

pub struct Node {
    name: Ident,
    events: Vec<Event>,
    attributes: Vec<Attribute>,
    generics: Vec<Type>,
    children: Vec<Kind>,
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

                    children.extend(ir.nodes)
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
    nodes: Vec<Kind>,
}

impl Ir {
    fn new<'a>(tags: &mut Peekable<impl Iterator<Item = &'a Tag>>) -> Ir {
        let mut nodes: Vec<Kind> = Vec::new();

        while let Some(tag) = tags.next() {
            match tag {
                Tag::OpenTag(open) => {
                    nodes.push(Kind::Node(Node::new(tags, open.clone())));
                },
                Tag::Template(template) => {
                    nodes.push(Kind::Template(Template::new(template.value.clone())));
                },
                _ => {
                    tag.span()
                        .error("expected an open tag or template")
                        .emit();
                },
            }
        }

        Ir {
            nodes,
        }
    }
}


