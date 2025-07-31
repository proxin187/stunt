pub mod state;
pub mod tree;

use tree::{Tree, AttrMap};
use state::Identity;

use std::sync::Arc;
use std::any::Any;


pub trait Component: Send + Sync + 'static {
    type Message: 'static;
    type Properties: Properties;

    fn create() -> Self where Self: Sized;

    fn callback(&mut self, callback: &Self::Message);

    fn view(&self, ctx: Context, properties: Self::Properties) -> Tree;
}

pub trait BaseComponent {
    fn base_callback(&mut self, callback: &Arc<dyn Any + Send + Sync>);

    fn base_view(&self, ctx: Context, attributes: AttrMap) -> Tree;
}

impl<T: Component> BaseComponent for T {
    fn base_callback(&mut self, callback: &Arc<dyn Any + Send + Sync>) { T::callback(self, callback.downcast_ref().expect("invalid callback type")) }

    fn base_view(&self, ctx: Context, attributes: AttrMap) -> Tree { T::view(self, ctx, T::Properties::create(attributes)) }
}

pub trait Properties {
    fn create(attributes: AttrMap) -> Self where Self: Sized;
}

impl Properties for () {
    fn create(_: AttrMap) -> () { () }
}

pub struct Context {
    pub identity: Identity,
}

impl Context {
    pub fn new(identity: Identity) -> Context {
        Context {
            identity,
        }
    }
}


