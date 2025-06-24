use crate::dom::component::{Component, Context};
use crate::dom::state::{self, Identity};
use crate::dom::callback;

use std::sync::Arc;
use std::any::Any;
use std::rc::Rc;

use spin::Mutex;


pub enum ComponentRef {
    Component(fn() -> Arc<Mutex<dyn Component + Send + Sync>>),
    Template(String),
}

impl ComponentRef {
    pub fn render(&self, identity: &Identity, context: Context) -> String {
        match self {
            ComponentRef::Component(component) => {
                state::get_or_insert(&identity, *component)
                    .lock()
                    .view(context)
                    .render()
            },
            ComponentRef::Template(template) => template.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Props {
    props: Rc<Vec<Node>>,
}

impl Props {
    pub fn new(props: Vec<Node>) -> Props {
        Props {
            props: Rc::new(props),
        }
    }

    pub fn render(&self) -> String {
        self.props.iter()
            .map(|prop| prop.render())
            .collect::<String>()
    }
}

#[derive(Clone)]
pub struct Attributes {
    attributes: Rc<Vec<(String, String)>>,
}

impl Attributes {
    pub fn new(attributes: Vec<(String, String)>) -> Attributes {
        Attributes {
            attributes: Rc::new(attributes),
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
    pub(crate) callbacks: Rc<Vec<(String, Arc<dyn Any + Send + Sync>)>>,
    pub(crate) attributes: Attributes,
    pub(crate) props: Props,
}

impl Node {
    pub fn new(
        identity: Identity,
        component: ComponentRef,
        callbacks: Vec<(String, Arc<dyn Any + Send + Sync>)>,
        attributes: Vec<(String, String)>,
        props: Vec<Node>,
    ) -> Node {
        Node {
            identity,
            component,
            callbacks: Rc::new(callbacks),
            attributes: Attributes::new(attributes),
            props: Props::new(props),
        }
    }

    pub fn render(&self) -> String {
        for (event, cb) in self.callbacks.iter() {
            callback::push(self.identity.clone(), event.clone(), cb.clone());
        }

        let context = Context::new(self.props.clone(), self.attributes.clone(), self.identity.clone());

        self.component.render(&self.identity, context)
    }
}


