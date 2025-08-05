//! This module contains everything related to components.

pub mod state;
pub mod tree;

use tree::{Tree, AttrMap};
use state::Identity;

use std::sync::Arc;
use std::any::Any;


/// A component is one of the basic building blocks within stunt. A component can pass messages
/// to its callback and receive properties from the parent.
///
/// You can implement component on virtually any type.
pub trait Component: Send + Sync + 'static {
    /// The message type will be passed to the [`callback`](Component::callback).
    type Message: 'static;

    /// The [`Properties`] will be passed down from the parent to the [`view`](Component::view).
    type Properties: Properties;

    /// Create your component.
    fn create() -> Self where Self: Sized;

    /// Recieve a callback. Callbacks can safely mutate the state of the component.
    fn callback(&mut self, callback: &Self::Message);

    /// The view describes the layout of how your component is to be rendered in the DOM.
    fn view(&self, ctx: Context, properties: Self::Properties) -> Tree;
}

/// The underlying low-level representation of a component within stunt.
///
/// Every type that implements [`Component`] has a blanket implementation of [`BaseComponent`].
///
/// ## Warning
/// This trait is not meant to be used outside the framework.
pub trait BaseComponent {
    /// Low-level implementation of a callback.
    fn base_callback(&mut self, callback: &Arc<dyn Any + Send + Sync>);

    /// Low-level implementation of a view.
    fn base_view(&self, ctx: Context, attributes: AttrMap) -> Tree;
}

impl<T: Component> BaseComponent for T {
    fn base_callback(&mut self, callback: &Arc<dyn Any + Send + Sync>) { T::callback(self, callback.downcast_ref().expect("invalid callback type")) }

    fn base_view(&self, ctx: Context, attributes: AttrMap) -> Tree { T::view(self, ctx, T::Properties::create(attributes)) }
}

/// The Properties trait can be implemented on any Struct you wish to recieve as
/// properties in the [`view`](Component::view).
pub trait Properties {
    /// Create properties from a AttrMap.
    ///
    /// ## Warning
    /// This function should not be manually implemented, use the derive macro instead.
    fn create(attributes: AttrMap) -> Self where Self: Sized;
}

impl Properties for () {
    fn create(_: AttrMap) -> () { () }
}

/// The context stores the identity of the component. The context must be in scope when using the [`html`](crate::stunt_macro::html) macro.
pub struct Context {
    /// The identity of the component.
    pub identity: Identity,
}

impl Context {
    /// Create a new context.
    ///
    /// ## Warning
    /// This function should not be used outside the framework.
    pub fn new(identity: Identity) -> Context {
        Context {
            identity,
        }
    }
}


