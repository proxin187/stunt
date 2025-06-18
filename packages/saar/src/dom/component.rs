use crate::dom::tree::{Attributes, Props};
use crate::dom::state::Identity;
use crate::html::Html;

use std::sync::Arc;
use std::any::Any;


pub trait Component {
    fn create() -> Self where Self: Sized;

    fn callback(&mut self, callback: Box<dyn Any>);

    fn view(&self, ctx: Context) -> Html;
}

#[derive(Clone)]
pub struct Context {
    component: Arc<dyn Component>,
    pub props: Props,
    pub attributes: Attributes,
    pub identity: Identity,
}

impl Context {
    pub fn new(component: Arc<dyn Component>, props: Props, attributes: Attributes, identity: Identity) -> Context {
        Context {
            component,
            props,
            attributes,
            identity,
        }
    }
}


