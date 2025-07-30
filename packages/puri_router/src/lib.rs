pub use puri_router_macro::Routable;

use puri_core::component::tree::AttrValue;

use puri::prelude::*;

use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;


pub trait Routable {
    fn route(map: HashMap<String, String>) -> Option<HashMap<String, Rc<dyn AttrValue>>>;
}

impl Routable for () {
    fn route(_: HashMap<String, String>) -> Option<HashMap<String, Rc<dyn AttrValue>>> {
        Some(HashMap::new())
    }
}

#[derive(Properties)]
pub struct RouteProperties {
    children: Children,
}

pub struct Router<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> Component for Router<T> {
    type Message = ();
    type Properties = RouteProperties;

    fn create() -> Router<T> { Router { _marker: std::marker::PhantomData } }

    fn callback(&mut self, _message: &()) {}

    fn view(&self, ctx: Context, properties: RouteProperties) -> Tree {
        html! {
            <? { properties.children.children() } ?>
        }
    }
}

#[derive(Properties)]
pub struct SwitchProperties {
    path: &'static str,
    children: Children,
}

pub struct Switch<T: Routable> {
    _marker: PhantomData<T>,
}

impl<T: Routable> Component for Switch<T> {
    type Message = ();
    type Properties = SwitchProperties;

    fn create() -> Switch<T> {
        Switch {
            _marker: PhantomData,
        }
    }

    fn callback(&mut self, _message: &()) {}

    fn view(&self, ctx: Context, properties: SwitchProperties) -> Tree {
        let pathname = web_sys::window()
            .expect("no window found")
            .location()
            .pathname()
            .expect("failed to get pathname");

        web_sys::console::log_1(&format!("pathname: {:?}", pathname).into());
        web_sys::console::log_1(&format!("path: {:?}", properties.path).into());

        html! {
            <? { pathname.eq(properties.path).then(|| properties.children.children()).unwrap_or_default() } ?>
        }
    }
}


