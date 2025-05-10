use crate::component::{Component, BaseComponent, ComponentRef, Callback};

use std::collections::HashMap;
use std::any::Any;


pub struct Html {
    pub(crate) component: ComponentRef,
    pub(crate) attributes: HashMap<String, fn() -> Box<dyn Any>>,
    pub(crate) children: Vec<Html>,
}

impl Html {
    pub fn new(
        component: ComponentRef,
        attributes: &[(String, fn() -> Box<dyn Any>)],
        children: Vec<Html>,
    ) -> Html {
        Html {
            attributes: attributes.into_iter().cloned().collect(),
            component,
            children,
        }
    }

    pub fn render(&self) {
    }
}


