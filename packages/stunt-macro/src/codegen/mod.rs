use crate::ir::{Ir, Kind};

use proc_macro2::TokenStream;
use quote::quote;


pub struct HtmlBuilder {
    nodes: Vec<TokenStream>,
}

impl HtmlBuilder {
    pub fn new() -> HtmlBuilder {
        HtmlBuilder {
            nodes: Vec::new(),
        }
    }

    fn build_nodes(&mut self, nodes: &[Kind]) {
        for kind in nodes.iter() {
            match kind {
                Kind::Node(node) => {
                    let name = &node.name;
                    let str_name = name.to_string();
                    let generics = &node.generics;

                    let events = node.events.iter()
                        .map(|event| event.tokens())
                        .collect::<TokenStream>();

                    let attributes = node.attributes.iter()
                        .map(|attribute| attribute.tokens())
                        .collect::<TokenStream>();

                    if name.to_string().chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) {
                        self.nodes.push(quote! {
                            ::stunt::component::html::HtmlNode::new(
                                ::stunt::component::html::HtmlKind::Element(::stunt::component::html::HtmlElement::new(#str_name.to_string(), std::vec![#attributes])),
                                std::sync::Arc::new(std::vec![#events]),
                                ::stunt::component::html::AttrMap::default(),
                            )
                        });
                    } else {
                        self.nodes.push(quote! {
                            ::stunt::component::html::HtmlNode::new(
                                ::stunt::component::html::HtmlKind::create_component::<#name<#(#generics),*>>(String::from(#str_name)),
                                std::sync::Arc::new(std::vec![#events]),
                                ::stunt::component::html::AttrMap::from(std::vec![#attributes].into_iter()),
                            )
                        });
                    }

                    self.build_nodes(&node.children);
                },
                Kind::Template(template) => {
                    let block = &template.expr;

                    self.nodes.push(quote! {
                        ::stunt::component::html::HtmlNode::new(
                            #[allow(unused_braces)]
                            ::stunt::component::html::HtmlKind::Template(std::sync::Arc::new(#block)),
                            std::sync::Arc::new(std::vec::Vec::new()),
                            ::stunt::component::html::AttrMap::default(),
                        )
                    });
                },
            }
        }
    }

    pub fn build(&mut self, ir: Ir) -> TokenStream {
        let layout = self.build_nodes(&ir.nodes);
        let nodes = &self.nodes;

        quote! {
            ::stunt::component::html::Html::new(
                std::vec![#(#nodes),*],
                std::vec![],
            )
        }
    }
}


