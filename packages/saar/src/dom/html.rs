use crate::dom::component::{ComponentRef, Context};
use crate::dom::tree;

use std::any::Any;
use std::sync::Arc;


pub struct Props {
    props: Vec<Arc<Html>>,
}

impl Props {
    pub fn new(props: Vec<Arc<Html>>) -> Props {
        for prop in &props {
            tree::insert(prop.id, prop.clone());
        }

        Props {
            props,
        }
    }

    pub fn render(&self) -> String {
        self.props.iter()
            .map(|html| html.render())
            .collect()
    }
}

pub struct Attributes {
    attributes: Vec<(String, fn() -> String)>,
}

impl Attributes {
    pub fn new(attributes: Vec<(String, fn() -> String)>) -> Attributes {
        Attributes {
            attributes,
        }
    }

    pub fn render(&self) -> String {
        self.attributes.iter()
            .map(|(key, value)| format!("{}=\"{}\" ", key, value()))
            .collect()
    }
}

pub struct Html {
    component: ComponentRef,
    attributes: Attributes,
    callback: Vec<(String, fn() -> Box<dyn Any>)>,
    props: Props,
    id: usize,
}

impl Html {
    pub fn new(
        component: ComponentRef,
        attributes: Attributes,
        callback: Vec<(String, fn() -> Box<dyn Any>)>,
        props: Props,
    ) -> Html {
        let id = tree::alloc_id();

        Html {
            component,
            attributes,
            callback,
            props,
            id,
        }
    }

    pub fn render(&self) -> String {
        // TODO: here we will have to add the event listener

        self.component.render(Context::new(&self.props, &self.attributes))
    }
}


