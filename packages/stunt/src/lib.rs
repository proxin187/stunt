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
//!     fn view(&self, _properties: ()) -> Html {
//!         html! {
//!             <div>
//!                 <button onclick={ Message::Add } >
//!                     { "increment" }
//!                 </button>
//!                 <h1>
//!                     { format!("count: {}", self.count) }
//!                 </h1>
//!             </div>
//!         }
//!     }
//! }
//!
//! fn main() {
//!     Renderer::<App>::new().render();
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
    pub use crate::component::html::{Html, AttrMap};
    pub use crate::render::Renderer;

    /// The html macro implements html templating for Rust. A context under the identifier "ctx" must
    /// be in scope of the macro, or else it will return a compile time error.
    ///
    /// This macro returns a [`html`](crate::component::html::Html) and should always be the
    /// prefered method for creating a [`html`](crate::component::html::Html).
    ///
    /// ## Syntax
    /// The syntax is similar to JSX.
    ///
    /// ### Attributes
    /// ```rust,no_run
    /// # use stunt::prelude::*;
    /// #
    /// # pub struct App;
    /// #
    /// # impl Component for App {
    /// #     type Message = ();
    /// #     type Properties = ();
    /// #
    /// #     fn create() -> App { App }
    /// #
    /// #     fn callback(&mut self, _: &Self::Message) {}
    /// #
    /// #     fn view(&self, ctx: Context, _properties: ()) -> Tree {
    /// html! {
    ///     <h1 foo={ "bar" } baz={ 44 }></h1>
    /// }
    /// #     }
    /// # }
    /// #
    /// # fn main() {
    /// #     Renderer::<App>::new().render();
    /// # }
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
    /// #
    /// # pub struct App;
    /// #
    /// # impl Component for App {
    /// #     type Message = ();
    /// #     type Properties = ();
    /// #
    /// #     fn create() -> App { App }
    /// #
    /// #     fn callback(&mut self, _: &Self::Message) {}
    /// #
    /// #     fn view(&self, ctx: Context, _properties: ()) -> Tree {
    /// html! {
    ///     <button onclick={ () }></button>
    /// }
    /// #     }
    /// # }
    /// #
    /// # fn main() {
    /// #     Renderer::<App>::new().render();
    /// # }
    /// ```
    ///
    /// ### Templates
    /// Templates will render as a Text Node into the DOM, or as html if you template a vector of trees.
    ///
    /// ```rust,no_run
    /// # use stunt::prelude::*;
    /// #
    /// # pub struct App;
    /// #
    /// # impl Component for App {
    /// #     type Message = ();
    /// #     type Properties = ();
    /// #
    /// #     fn create() -> App { App }
    /// #
    /// #     fn callback(&mut self, _: &Self::Message) {}
    /// #
    /// #     fn view(&self, ctx: Context, _properties: ()) -> Tree {
    /// html! {
    ///     { "this will be inserted as a template" }
    /// }
    /// #     }
    /// # }
    /// #
    /// # fn main() {
    /// #     Renderer::<App>::new().render();
    /// # }
    /// ```
    ///
    /// ```rust,no_run
    /// # use stunt::prelude::*;
    /// #
    /// # pub struct App;
    /// #
    /// # impl Component for App {
    /// #     type Message = ();
    /// #     type Properties = ();
    /// #
    /// #     fn create() -> App { App }
    /// #
    /// #     fn callback(&mut self, _: &Self::Message) {}
    /// #
    /// #     fn view(&self, ctx: Context, _properties: ()) -> Tree {
    /// html! {
    ///     {
    ///         Children::new(vec![
    ///             html! {
    ///                 { "you can also use Children as a template" }
    ///             }
    ///         ])
    ///     }
    /// }
    /// #     }
    /// # }
    /// #
    /// # fn main() {
    /// #     Renderer::<App>::new().render();
    /// # }
    /// ```
    pub use crate::stunt_macro::html;

    /// This macro will implement the [`Properties`] trait for a
    /// named Struct.
    pub use crate::stunt_macro::Properties;
}


