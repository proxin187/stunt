use crate::dom::component::Component;
use crate::dom::tree::Context;

use std::any::Any;


pub enum ComponentRef {
    Component(Box<dyn Component + Send + Sync>),
    Block(fn(Context) -> String),
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
}

pub struct Html {
    pub(crate) component: ComponentRef,
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) callback: Vec<(String, fn() -> Box<dyn Any>)>,
    pub(crate) props: Vec<Html>,
}

impl Html {
    pub fn new(
        component: ComponentRef,
        attributes: Vec<Attribute>,
        callback: Vec<(String, fn() -> Box<dyn Any>)>,
        props: Vec<Html>,
    ) -> Html {
        Html {
            component,
            attributes,
            callback,
            props,
        }
    }
}


