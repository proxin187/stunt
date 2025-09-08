//! This module contains everything related to components.

pub mod render;
pub mod global;
pub mod html;
mod virtual_dom;

use html::{Html, Children};

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
    type Properties: Properties + Buildable;

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

impl<T: Buildable> Properties for T {}

/// The Buildable trait creates a builder, The Buildable trait provides a blanket implementation of [`Properties`].
pub trait Buildable: Clone {
    /// The builder type.
    type Builder: PreBuild;

    /// Create the builder.
    fn builder() -> Self::Builder;
}

/// The PreBuild trait represents properties that arent built yet.
pub trait PreBuild {
    /// Insert children into the properties.
    fn children(&mut self, _children: Children) {}

    /// Build the properties.
    fn build(&self) -> Rc<dyn Any>;
}

impl Buildable for () {
    type Builder = EmptyBuilder;

    fn builder() -> EmptyBuilder { EmptyBuilder }
}

/// A builder for ().
#[derive(Clone)]
pub struct EmptyBuilder;

impl PreBuild for EmptyBuilder {
    fn build(&self) -> Rc<dyn Any> { Rc::new(()) }
}

impl PreBuild for () {
    fn build(&self) -> Rc<dyn Any> { Rc::new(()) }
}


