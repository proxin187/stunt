use crate::dom::component::Component;
use crate::dom::state::Identity;

use std::sync::Arc;
use std::any::Any;


pub enum ComponentRef {
    Component(Box<dyn Fn() -> Arc<dyn Component + Send + Sync>>),
    Block(Box<dyn Fn() -> String>),
}

pub struct Attribute {
    key: String,
    value: fn() -> String,
}

impl Attribute {
    pub fn new(key: String, value: fn() -> String) -> Attribute {
        Attribute {
            key,
            value,
        }
    }

    pub fn render(&self) -> String {
        format!("{}=\"{}\"", self.key, (self.value)())
    }
}

pub struct Html {
    pub(crate) identity: Identity,
    pub(crate) component: ComponentRef,
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) callback: Vec<(String, fn() -> Box<dyn Any>)>,
    pub(crate) props: Vec<Html>,
}

impl Html {
    pub fn new(
        identity: Identity,
        component: ComponentRef,
        attributes: Vec<Attribute>,
        callback: Vec<(String, fn() -> Box<dyn Any>)>,
        props: Vec<Html>,
    ) -> Html {
        Html {
            identity,
            component,
            attributes,
            callback,
            props,
        }
    }
}


