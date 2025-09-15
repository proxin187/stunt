#![warn(missing_docs)]

// TODO: we can make this into a full stack framework. We can have advanced signal system where the
// frontend can send signals to the backend with a convenient api
//
// TODO: We can have the project lsp target_arch be native (x86_64) and then do the conditional code
// inside the library.

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

pub mod frontend;
pub mod backend;
pub mod prelude;


