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
//! ```rust
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
//!     fn view(&self, ctx: Context, _properties: ()) -> Tree {
//!         html! {
//!             <div>
//!                 <button event: mousedown={ Message::Add } >
//!                     <? { "increment" } ?>
//!                 </button>
//!                 <h1>
//!                     <? { format!("count: {}", self.count) } ?>
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
mod vdom;

pub(crate) use stunt_macro;

pub mod prelude {
    pub use crate::component::{Component, Context, Properties};
    pub use crate::component::tree::{Tree, Children, AttrMap};
    pub use crate::render::Renderer;

    pub use crate::stunt_macro::{html, Properties};
}


