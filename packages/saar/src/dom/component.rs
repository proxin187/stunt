use crate::dom::html::{Html, Props};

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

pub struct Context {
    pub props: Props,
}

impl Context {
    pub fn new(props: Props) -> Context {
        Context {
            props,
        }
    }
}


