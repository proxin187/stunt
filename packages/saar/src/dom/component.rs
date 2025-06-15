use crate::dom::tree::{Attributes, Props};
use crate::html::{Html, ComponentRef};

use std::sync::Arc;
use std::any::Any;


pub trait Component {
    fn create() -> Self where Self: Sized;

    fn callback(&mut self, callback: Box<dyn Any>);

    fn extract(&self, extract: Box<dyn Any>) -> String;

    fn view(&self, ctx: Context) -> Html;
}

pub struct Base;

impl Component for Base {
    fn create() -> Base { Base }

    fn callback(&mut self, _callback: Box<dyn Any>) {}

    fn extract(&self, _extract: Box<dyn Any>) -> String { String::default() }

    fn view(&self, ctx: Context) -> Html {
        Html::new(
            ComponentRef::Block(|| { ctx.props.render() }),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
    }
}

#[derive(Clone)]
pub struct Context {
    component: Arc<dyn Component>,
    pub props: Props,
    pub attributes: Attributes,
}

impl Context {
    pub fn new(component: Arc<dyn Component>, props: Props, attributes: Attributes) -> Context {
        Context {
            component,
            props,
            attributes,
        }
    }

    pub fn extract<T: Any>(&self, extract: T) -> String {
        self.component.extract(Box::new(extract))
    }
}


