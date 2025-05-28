use crate::html::{Html, Props, Attributes, ComponentRef};

use std::any::Any;


pub trait Component {
    fn create() -> Self where Self: Sized;

    fn callback(&mut self, callback: Box<dyn Any>);

    fn view(&self) -> Html;
}

pub struct Base;

impl Component for Base {
    fn create() -> Base { Base }

    fn callback(&mut self, _callback: Box<dyn Any>) {}

    fn view(&self) -> Html {
        Html::new(
            ComponentRef::Block(|ctx| { format!("{}", ctx.props.render()) }),
            Attributes::new(Vec::new()),
            Vec::new(),
            Props::new(Vec::new()),
        )
    }
}


// TODO: we need to add so that context also has a reference to self

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


