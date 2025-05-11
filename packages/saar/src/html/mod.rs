use crate::component::{ComponentRef, Context};

use std::collections::HashMap;
use std::any::Any;


pub struct Html<'a> {
    pub(crate) component: ComponentRef,
    pub(crate) attributes: HashMap<String, fn() -> Box<dyn Any>>,
    pub(crate) children: &'a [Html<'a>],
}

impl<'a> Html<'a> {
    pub fn new(
        component: ComponentRef,
        attributes: &[(String, fn() -> Box<dyn Any>)],
        children: &'a [Html<'a>],
    ) -> Html<'a> {
        Html {
            attributes: attributes.into_iter().cloned().collect(),
            component,
            children,
        }
    }

    pub fn render(&self) -> String {
        self.component.render(Context::new(&self.children))
    }
}


