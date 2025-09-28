//! Re-export of common types.

pub use crate::frontend::{Component, Properties, Link};
pub use crate::frontend::html::Html;
pub use crate::frontend::render::Renderer;

#[cfg(not(target_arch = "wasm32"))]
pub use crate::backend::Service;

/// The html macro implements html templating for Rust.
///
/// This macro returns [`Html`] and should always be the
/// prefered method for creating [`Html`].
///
/// ### Attributes
/// ```rust,no_run
/// # use stunt::prelude::*;
/// # fn main() {
/// html! {
///     <h1 foo={ "bar" } baz={ 44 }></h1>
/// }
/// # ;}
/// ```
///
/// ### Event listeners
/// Event listeners will call the callback with any value. If the type of the event doesnt
/// match the [`Message`](crate::component::Component::Message) type of the [`Component`] you will encounter a runtime error.
///
/// Any attribute that starts with "on" will be treated as an event listener.
///
/// ```rust,no_run
/// # use stunt::prelude::*;
/// # fn main() {
/// html! {
///     <button onclick={ () }></button>
/// }
/// # ;}
/// ```
///
/// ### Templates
/// Templates will render as a Text Node into the DOM, or as html if you template a vector of html.
///
/// ```rust,no_run
/// # use stunt::prelude::*;
/// # fn main() {
/// html! {
///     { "this will be inserted as a template" }
/// }
/// # ;}
/// ```
///
/// ```rust,no_run
/// # use stunt::prelude::*;
/// # fn main() {
/// html! {
///     {
///         html! {
///             { "you can also use html as a template" }
///         }
///     }
/// }
/// # ;}
/// ```
pub use stunt_macro::html;

/// This macro will implement the [`Properties`] trait for a
/// named Struct.
pub use stunt_macro::Properties;


