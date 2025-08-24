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

mod path;

pub use stunt_router_macro::Routable;

use stunt::component::html::{AttrValue, Children};

use stunt::prelude::*;

use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;


/// The Routable trait allows a type to be taken in as properties from a Route.
pub trait Routable {
    /// The route function ensures that the correct attributes where passed and casts each one into a [`AttrValue`].
    ///
    /// ## Warning
    /// This function is not supposed to be called outside the framework.
    fn route(map: HashMap<String, String>) -> Option<Vec<(String, Rc<dyn AttrValue>)>>;
}

/// The properties of a [`Router`].
///
/// ## Warning
/// This type is not supposed to be used outside of the framework.
#[derive(Properties)]
pub struct RouteProperties {
    children: Children,
}

/// The Router routes its children.
pub struct Router;

impl Component for Router {
    type Message = ();
    type Properties = RouteProperties;

    fn create() -> Router { Router }

    fn view(&self, properties: RouteProperties) -> Html {
        html! {
            <span>
                { properties.children }
            </span>
        }
    }
}

/// The properties of a [`Switch`].
///
/// ## Warning
/// This type is not supposed to be used outside of the framework.
#[derive(Properties)]
pub struct SwitchProperties {
    path: &'static str,
}

/// The Switch allows you to route a [`Component`] to a path.
/// The [`Component`] must be specified through the generic.
/// You can specify path segments using the ':somename' operator.
pub struct Switch<T: Component> where T::Properties: Routable {
    _marker: PhantomData<T>,
}

impl<T: Component> Component for Switch<T> where T::Properties: Routable {
    type Message = ();
    type Properties = SwitchProperties;

    fn create() -> Switch<T> {
        Switch {
            _marker: PhantomData,
        }
    }

    fn view(&self, properties: SwitchProperties) -> Html {
        let pathname = web_sys::window()
            .expect("no window found")
            .location()
            .pathname()
            .expect("failed to get pathname");

        let attributes = path::parse(&pathname, properties.path).and_then(|path| T::Properties::route(path));

        html! {
            { attributes.map(|attributes| html! { <T ?{ attributes } /> }).unwrap_or_default() }
        }
    }
}


