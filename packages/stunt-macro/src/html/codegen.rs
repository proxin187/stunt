use crate::html::intermediate::{Ir, Kind};

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

    fn build_nodes(&mut self, nodes: &[Kind]) -> TokenStream {
        let mut layout: Vec<TokenStream> = Vec::new();

        for kind in nodes.iter() {
            match kind {
                Kind::Node(node) => {
                    let name = &node.name;
                    let str_name = name.to_string();
                    let generics = &node.generics;
                    let index = self.nodes.len();

                    let events = node.events.iter()
                        .map(|event| event.tokens())
                        .collect::<TokenStream>();

                    if name.to_string().chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) {
                        let attributes = node.attributes.iter()
                            .map(|attribute| attribute.element_tokens())
                            .collect::<TokenStream>();

                        self.nodes.push(quote! {
                            ::stunt::component::html::HtmlNode::new(
                                ::stunt::component::html::HtmlKind::Element(::stunt::component::html::HtmlElement::new(#str_name.to_string(), ::std::vec![#attributes])),
                                ::std::sync::Arc::new(::std::vec![#events]),
                                (),
                            )
                        });
                    } else {
                        let properties = node.attributes.iter()
                            .map(|attribute| attribute.component_tokens())
                            .collect::<TokenStream>();

                        self.nodes.push(quote! {{
                            let mut builder = <<#name<#(#generics),*> as ::stunt::component::Component>::Properties as ::stunt::component::Buildable>::builder();
                            let __stunt_token = ();
                            #properties
                            builder.typecheck(__stunt_token);
                            ::stunt::component::html::HtmlNode::new(
                                ::stunt::component::html::HtmlKind::create_component::<#name<#(#generics),*>>(String::from(#str_name)),
                                ::std::sync::Arc::new(::std::vec![#events]),
                                builder,
                            )
                        }});
                    }

                    let children = self.build_nodes(&node.children);

                    layout.push(quote! {
                        ::stunt::component::html::NodeRef::new(
                            #index,
                            ::std::rc::Rc::new(#children),
                        )
                    });
                },
                Kind::Template(template) => {
                    let block = &template.expr;
                    let index = self.nodes.len();

                    self.nodes.push(quote! {
                        ::stunt::component::html::HtmlNode::new(
                            #[allow(unused_braces)]
                            ::stunt::component::html::HtmlKind::Template(::std::sync::Arc::new(#block)),
                            ::std::sync::Arc::new(std::vec::Vec::new()),
                            (),
                        )
                    });

                    layout.push(quote! {
                        ::stunt::component::html::NodeRef::new(
                            #index,
                            ::std::rc::Rc::new(std::vec::Vec::new()),
                        )
                    });
                },
            }
        }

        quote! { std::vec![#(#layout),*] }
    }

    pub fn build(&mut self, ir: Ir) -> TokenStream {
        let layout = self.build_nodes(&ir.nodes);
        let nodes = &self.nodes;

        quote! {
            ::stunt::component::html::Html::new(
                ::std::rc::Rc::new(::std::vec![#(#nodes),*]),
                ::std::rc::Rc::new(#layout),
            )
        }
    }
}


