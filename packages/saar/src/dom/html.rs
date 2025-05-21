use crate::dom::component::{ComponentRef, Context};

use std::collections::HashMap;
use std::any::Any;


// TODO: make this into a slice again after we are done with the componentsref issue
pub struct Props {
    props: Vec<Html>,
}

impl Props {
    pub fn new(props: Vec<Html>) -> Props {
        Props {
            props,
        }
    }

    pub fn render(self) -> String {
        self.props.into_iter()
            .map(|html| html.render())
            .collect()
    }
}

pub struct Html {
    pub(crate) component: ComponentRef,
    pub(crate) attributes: HashMap<String, fn() -> Box<dyn Any>>,
    pub(crate) props: Props,
}

impl Html {
    pub fn new(
        component: ComponentRef,
        attributes: &[(String, fn() -> Box<dyn Any>)],
        props: Props,
    ) -> Html {
        Html {
            component,
            attributes: attributes.into_iter().cloned().collect(),
            props,
        }
    }

    pub fn render(self) -> String {
        self.component.render(Context::new(self.props))
    }
}


