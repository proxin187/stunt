use crate::dom::component::{Component, Context};
use crate::dom::state::{self, Identity};

use std::sync::Arc;
use std::any::Any;


pub enum ComponentRef {
    Component(Box<dyn Fn() -> Arc<dyn Component + Send + Sync>>),
    Block(Box<dyn Fn() -> String>),
}

impl ComponentRef {
    pub fn render(&self, identity: &Identity, context: Context) -> String {
        match self {
            ComponentRef::Component(component) => {
                state::get_or_insert(&identity, component)
                    .view(context)
                    .render()
            },
            ComponentRef::Block(f) => f(),
        }
    }
}

pub struct Props {
    props: Vec<Node>,
}

impl Props {
    pub fn new(props: Vec<Node>) -> Props {
        Props {
            props,
        }
    }

    pub fn render(&self) -> String {
        props.iter()
            .map(|prop| prop.render())
            .collect::<String>()
    }
}

pub struct Attributes {
    attributes: Vec<(String, String)>,
}

impl Attributes {
    pub fn new(attributes: Vec<(String, String)>) -> Attributes {
        Attributes {
            attributes,
        }
    }

    pub fn render(&self) -> String {
        self.attributes.iter()
            .map(|(key, value)| format!("{}=\"{}\"", key, value))
            .collect::<String>()
    }
}

pub struct Node {
    pub(crate) identity: Identity,
    pub(crate) component: ComponentRef,
    pub(crate) attributes: Attributes,
    pub(crate) props: Props,
}

impl Node {
    pub fn new(
        identity: Identity,
        component: ComponentRef,
        attributes: Vec<(String, String)>,
        props: Vec<Node>,
    ) -> Node {
        Node {
            identity,
            component,
            attributes: Attributes::new(attributes),
            props: Props::new(props),
        }
    }

    pub fn render(&self) -> String {
        let context = Context::new(&self.props, &self.attributes, &self.identity);

        self.component.render(&self.identity, context)
    }
}


