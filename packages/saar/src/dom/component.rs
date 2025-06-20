use crate::dom::tree::{Node, Props, Attributes};
use crate::dom::state::Identity;

use std::any::Any;


pub trait Component {
    fn create() -> Self where Self: Sized;

    fn callback(&mut self, callback: Box<dyn Any>);

    fn view(&self, ctx: Context) -> Node;
}

pub struct Context<'a> {
    pub props: &'a Props,
    pub attributes: &'a Attributes,
    pub identity: &'a Identity,
}

impl<'a> Context<'a> {
    pub fn new(props: &'a Props, attributes: &'a Attributes, identity: &'a Identity) -> Context<'a> {
        Context {
            props,
            attributes,
            identity,
        }
    }
}


