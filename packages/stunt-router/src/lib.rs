mod path;

pub use stunt_router_macro::Routable;

use stunt::component::html::{AttrValue, Children};

use stunt::prelude::*;

use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;


pub trait Routable {
    fn route(map: HashMap<String, String>) -> Option<Vec<(String, Rc<dyn AttrValue>)>>;
}

#[derive(Properties)]
pub struct RouteProperties {
    children: Children,
}

pub struct Router;

impl Component for Router {
    type Message = ();
    type Properties = RouteProperties;

    fn create() -> Router { Router }

    fn callback(&mut self, _message: &()) {}

    fn view(&self, properties: RouteProperties) -> Html {
        html! {
            <span>
                { properties.children }
            </span>
        }
    }
}

#[derive(Properties)]
pub struct SwitchProperties {
    path: &'static str,
}

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

    fn callback(&mut self, _message: &()) {}

    fn view(&self, properties: SwitchProperties) -> Html {
        let pathname = web_sys::window()
            .expect("no window found")
            .location()
            .pathname()
            .expect("failed to get pathname");

        let attributes = path::parse(&pathname, properties.path).and_then(|path| T::Properties::route(path));

        html! {
            { attributes.map(|attributes| html! { <T ?{ attributes }></T> }).unwrap_or_default() }
        }
    }
}


