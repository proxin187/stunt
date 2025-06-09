use crate::html::{Html, ComponentRef};

use std::any::Any;


pub trait Component {
    fn create() -> Self where Self: Sized;

    fn callback(&mut self, callback: Box<dyn Any>);

    fn extract(&self, extract: Box<dyn Any>) -> String;

    fn view(&self) -> Html;

    fn name(&self) -> String;
}

pub struct Base;

impl Component for Base {
    fn create() -> Base { Base }

    fn callback(&mut self, _callback: Box<dyn Any>) {}

    fn extract(&self, _extract: Box<dyn Any>) -> String { String::default() }

    fn view(&self) -> Html {
        Html::new(
            ComponentRef::Block(|ctx| { ctx.props.render() }),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
    }

    fn name(&self) -> String { String::from("base") }
}


