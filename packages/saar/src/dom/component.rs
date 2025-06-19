use crate::dom::tree::{Node, Attribute};
use crate::dom::state::Identity;
use crate::html::Html;

use std::any::Any;


pub trait Component {
    fn create() -> Self where Self: Sized;

    fn callback(&mut self, callback: Box<dyn Any>);

    fn view(&self, ctx: Context) -> Html;
}

pub struct Context {
    pub props: Vec<Node>,
    pub attributes: Vec<Attribute>,
    pub identity: Identity,
}

impl Context {
    pub fn new(props: Vec<Node>, attributes: Vec<Attribute>, identity: Identity) -> Context {
        Context {
            props,
            attributes,
            identity,
        }
    }
}


