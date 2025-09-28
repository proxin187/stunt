#![feature(trim_prefix_suffix)]
#![warn(missing_docs)]

//! # Stunt Router Documentation
//!
//! The Stunt Router is the router implementation for stunt.
//!
//! ## Features
//!
//! - Route a component to a Path.
//! - Extract information with Path Segments.
//!
//! ## Example
//! ```rust,no_run
//! use stunt::prelude::*;
//! use stunt_router::Routable;
//!
//!
//! #[derive(Properties, Clone)]
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
//!                 { format!("{}-{}", properties.id, properties.name) }
//!             </h1>
//!         }
//!     }
//! }
//!
//! #[derive(Routable)]
//! pub enum Route {
//!     #[at("/account/:id/:name")]
//!     Account {
//!         id: usize,
//!         name: String,
//!     },
//!     #[not_found]
//!     NotFound,
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
//!         match stunt_router::route::<Route>() {
//!             Route::Account { id, name } => html! { <Account id={ id } name={ name } /> },
//!             Route::NotFound => {
//!                 html! {
//!                     <h1>
//!                         { "404: Not Found" }
//!                     </h1>
//!                 }
//!             },
//!         }
//!     }
//! }
//!
//! fn main() {
//!     Renderer::new::<App>().render();
//! }
//! ```

pub use stunt_router_macro::Routable;

use wasm_bindgen::prelude::*;
use web_sys::CustomEvent;


/// The Routable trait allows an enum to be routed.
pub trait Routable {
    /// Returns the appropriate route based on the path.
    fn route(path: &[&str]) -> Self where Self: Sized;

    /// Reconstructs the path of the route.
    fn path(self) -> String;
}

/// Get the current route.
#[inline]
pub fn route<T: Routable>() -> T {
    let pathname = web_sys::window()
        .expect("no window found")
        .location()
        .pathname()
        .expect("failed to get pathname");

    let path = pathname.split('/')
        .collect::<Vec<&str>>();

    T::route(&path.trim_suffix(&["/"]))
}

/// Register a callback for updates on the router.
#[inline]
pub fn register_callback(f: impl Fn() + 'static) {
    let window = web_sys::window().expect("no window found");

    let closure = Closure::<dyn Fn()>::new(f);

    window.add_event_listener_with_callback("RouterUpdate", closure.as_ref().unchecked_ref()).expect("failed to add event listener");

    closure.forget();
}

/// Update the url and dispatch a router callback.
#[inline]
pub fn redirect<T: Routable>(route: T) {
    let window = web_sys::window().expect("no window found");
    let history = window.history().expect("no history found");

    history.replace_state_with_url(&JsValue::null(), "", Some(route.path().as_str())).expect("failed to replace url");

    let event = CustomEvent::new("RouterUpdate").expect("failed to create event");

    window.dispatch_event(&event).expect("failed to dispatch");
}


