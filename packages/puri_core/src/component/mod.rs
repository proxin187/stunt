pub mod state;
pub mod tree;

use tree::{Tree, Props, Attributes};
use state::Identity;

use std::collections::HashMap;
use std::sync::Arc;
use std::any::Any;


pub trait Component {
    type Message: 'static;
    type Properties: Properties;

    fn create() -> Self where Self: Sized;

    fn callback(&mut self, callback: &Self::Message);

    fn view(&self, ctx: Context) -> Tree;
}

pub trait BaseComponent {
    fn base_callback(&mut self, callback: &Arc<dyn Any + Send + Sync>);

    fn base_view(&self, ctx: Context) -> Tree;
}

impl<T: Component> BaseComponent for T {
    fn base_callback(&mut self, callback: &Arc<dyn Any + Send + Sync>) { T::callback(self, callback.downcast_ref().expect("invalid callback type")) }

    fn base_view(&self, ctx: Context) -> Tree { T::view(self, ctx) }
}

pub trait Properties {
    fn create(attr: HashMap<String, Arc<dyn Any>>) -> Self where Self: Sized;
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


