#![warn(missing_docs)]

//! # Stunt Web Framework Documentation
//!
//! stunt is a frontend web framework for developing reactive user interfaces with Rust.
//!
//! ## Features
//!
//! - Macro for writing html with rust expressions, similar to that of JSX.
//! - Highly extensible [components](#component).
//! - Use any build tool you like eg. [trunk](https://trunkrs.dev/).
//! - Multiple ways to manage the state of your application.
//!
//! ## Example
//! ```rust,no_run
//! use stunt::prelude::*;
//!
//! pub enum Message {
//!    Add,
//! }
//!
//! pub struct App {
//!     count: usize,
//! }
//!
//! impl Component for App {
//!     type Message = Message;
//!     type Properties = ();
//!
//!     fn create() -> App {
//!         App {
//!             count: 0,
//!         }
//!     }
//!
//!     fn callback(&mut self, message: &Message) {
//!         match message {
//!             Message::Add => {
//!                 self.count += 1;
//!             },
//!         }
//!     }
//!
//!     fn view(&self, _: ()) -> Html {
//!         html! {
//!             <div>
//!                 <button onclick={ Message::Add } >
//!                     { "increment" }
//!                 </button>
//!                 <h1>
//!                     { self.count }
//!                 </h1>
//!             </div>
//!         }
//!     }
//! }
//!
//! fn main() {
//!     Renderer::new::<App>().render();
//! }
//! ```

pub mod component;
pub mod render;
pub mod global;
mod virtual_dom;

pub(crate) use stunt_macro;

pub mod prelude {
    //! Re-export of common types.

    pub use crate::component::{Component, Properties};
    pub use crate::component::html::Html;
    pub use crate::component::node_id::NodeId;

    pub use crate::render::Renderer;

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
    pub use crate::stunt_macro::html;

    /// This macro will implement the [`Properties`] trait for a
    /// named Struct.
    pub use crate::stunt_macro::Properties;
}


