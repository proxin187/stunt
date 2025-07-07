pub mod callback;
pub mod state;
pub mod tree;

use tree::{Tree, Props, Attributes};
use state::Identity;

use std::sync::Arc;
use std::any::Any;


pub trait Component {
    fn create() -> Self where Self: Sized;

    fn callback(&mut self, callback: &Arc<dyn Any + Send + Sync>);

    fn view(&self, ctx: Context) -> Tree;
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


