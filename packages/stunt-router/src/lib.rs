#![warn(missing_docs)]

//! # Stunt Router Documentation
//!
//! The Stunt Router is the standard router implementation for stunt.
//!
//! ## Features
//!
//! - Route a component to a Path.
//! - Extract information with Path Segments.
//!
//! ## Example
//! ```rust,no_run
//! use stunt::prelude::*;
//!
//! use stunt_router::{Switch, Router, Routable};
//!
//!
//! #[derive(Properties, Routable)]
//! pub struct AccountProperties {
//!     id: usize,
//!     name: String,
//! }
//!
//! pub struct Account;
//!
//! impl Component for Account {
//!     type Message = ();
//!     type Properties = AccountProperties;
//!
//!     fn create() -> Account { Account }
//!
//!     fn view(&self, properties: AccountProperties) -> Html {
//!         html! {
//!             <h1>
//!                 { format!("id: {}, name: {}", properties.id, properties.name) }
//!             </h1>
//!         }
//!     }
//! }
//!
//! pub struct App;
//!
//! impl Component for App {
//!     type Message = ();
//!     type Properties = ();
//!
//!     fn create() -> App { App }
//!
//!     fn view(&self, _: ()) -> Html {
//!         html! {
//!             <Router>
//!                 <Switch<Account> path={ "/settings/account/:id/:name" } />
//!             </Router>
//!         }
//!     }
//! }
//!
//! fn main() {
//!     Renderer::new::<App>().render();
//! }
//! ```

pub use stunt_router_macro::Routable;

/// The Routable trait allows an enum to be routed.
pub trait Routable {
    /// Returns the appropriate route based on the path.
    fn route(path: &[&str]) -> Self where Self: Sized;
}

/// Get the current route.
#[inline]
pub fn route<T: Routable>() -> T {
    let pathname = web_sys::window()
        .expect("no window found")
        .location()
        .pathname()
        .expect("failed to get pathname");

    let path = pathname.split('/').collect::<Vec<&str>>();

    T::route(&path)
}


