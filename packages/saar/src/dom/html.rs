use crate::dom::component::{ComponentRef, Context};
use crate::dom::tree;

use std::any::Any;
use std::sync::Arc;

// TODO: we should maybe move this and the component module into a seperate parent module called
// html as this has nothing to do with the virtual dom but is rather just a html representation


pub struct Props {
    props: Vec<Arc<Html>>,
}

impl Props {
    pub fn new(props: Vec<Arc<Html>>) -> Props {
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

    // TODO: move the render function into the virtual dom
    pub fn render(&self) -> String {
        // TODO: here we will have to add the event listener

        self.component.render(Context::new(&self.props, &self.attributes))
    }
}


