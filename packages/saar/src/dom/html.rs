use crate::dom::component::{ComponentRef, Context};

use std::any::Any;


pub struct Props {
    props: Vec<Html>,
}

impl Props {
    pub fn new(props: Vec<Html>) -> Props {
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
}

impl Html {
    pub fn new(
        component: ComponentRef,
        attributes: Attributes,
        callback: Vec<(String, fn() -> Box<dyn Any>)>,
        props: Props,
    ) -> Html {
        Html {
            component,
            attributes,
            callback,
            props,
        }
    }

    pub fn render(&self) -> String {
        self.component.render(Context::new(&self.props, &self.attributes))
    }
}


