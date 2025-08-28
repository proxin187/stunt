//! This module contains everything related to components.

pub mod node_id;
pub mod path;
pub mod html;

use html::Html;

use std::sync::Arc;
use std::any::Any;
use std::rc::Rc;


/// A component is one of the basic building blocks within stunt. A component can pass messages
/// to its callback and receive properties from the parent.
///
/// You can implement component on virtually any type.
pub trait Component: Send + Sync + Sized + 'static {
    /// The message type will be passed to the [`callback`](Component::callback).
    type Message: 'static;

    /// The [`Properties`] will be passed down from the parent to the [`view`](Component::view).
    type Properties: Properties;

    /// Create your component.
    fn create() -> Self
    where Self: Sized;

    /// Recieve a callback. Callbacks can safely mutate the state of the component.
    #[allow(unused_variables)]
    fn callback(&mut self, callback: &Self::Message) {}

    /// The view describes the layout of how your component is to be rendered in the DOM.
    fn view(&self, properties: Self::Properties) -> Html;
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
    fn base_view(&self, properties: Rc<dyn Any>) -> Html;
}

impl<T: Component> BaseComponent for T {
    fn base_callback(&mut self, callback: &Arc<dyn Any + Send + Sync>) {
        T::callback(self, callback.downcast_ref().expect("invalid callback type"))
    }

    fn base_view(&self, properties: Rc<dyn Any>) -> Html { T::view(self, T::Properties::into_properties(properties)) }
}

/// The Properties trait can be implemented on any Struct you wish to recieve as
/// properties in the [`view`](Component::view).
pub trait Properties: Clone {
    /// Cast into properties.
    fn into_properties(any: Rc<dyn Any>) -> Self
    where Self:
        Sized + 'static
    {
        any.downcast_ref::<Self>().expect("internal error").clone()
    }
}

impl Properties for () {}

/// Buildable is a marker trait for buildable properties.
pub trait Buildable {
    /// The builder type.
    type Builder;

    /// Create the builder.
    fn builder() -> Self::Builder;
}

impl Buildable for () {
    type Builder = EmptyBuilder;

    fn builder() -> EmptyBuilder { EmptyBuilder }
}

/// Build a ().
pub struct EmptyBuilder;

impl EmptyBuilder {
    pub fn build(self) -> () { () }
}


