use crate::dom::html::{Html, Props, Attributes};

use std::any::Any;


pub trait Component {
    fn create() -> Self where Self: Sized;

    fn callback(&mut self, callback: Box<dyn Any>);

    fn view(&self, ctx: Context) -> Html;
}

pub enum ComponentRef {
    Component(Box<dyn Component>),
    Block(Box<dyn Fn() -> String>),
}

impl ComponentRef {
    pub fn render(&self, context: Context) -> String {
        match self {
            ComponentRef::Component(component) => component.view(context).render(),
            ComponentRef::Block(block) => block(),
        }
    }
}

pub struct Context<'a> {
    pub props: &'a Props,
    pub attributes: &'a Attributes,
}

impl<'a> Context<'a> {
    pub fn new(props: &'a Props, attributes: &'a Attributes) -> Context<'a> {
        Context {
            props,
            attributes,
        }
    }
}


