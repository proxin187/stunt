use crate::html::{Html, Props};

use std::any::Any;


pub trait Component {
    fn create() -> Self where Self: Sized;

    fn callback(&mut self, callback: Box<dyn Any>);

    fn view(&self, ctx: Context) -> Html;
}

// we could maybe use the any::TypeId struct
// the issue is that we have to find a better way to represent callback maybe
// TODO: figure out how to represent the component here
//
// the issue is only really the callback
// maybe we could represent the callback with any?

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


