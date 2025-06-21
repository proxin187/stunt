use crate::dom::tree::{Node, Props, Attributes};
use crate::dom::state::Identity;

use std::any::Any;


pub trait Component {
    fn create() -> Self where Self: Sized;

    fn callback(&mut self, callback: Box<dyn Any>);

    fn view(&self, ctx: Context) -> Node;
}

pub struct Context {
    pub props: Props,
    pub attributes: Attributes,
    pub identity: Identity,
}

impl Context {
    pub fn new(props: Props, attributes: Attributes, identity: Identity) -> Context {
        Context {
            props,
            attributes,
            identity,
        }
    }
}


