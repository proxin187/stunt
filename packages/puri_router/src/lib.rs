use puri::prelude::*;


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

    fn view(&self, ctx: Context, properties: RouteProperties) -> Tree {
        html! {
            <template { properties.children.children() } />
        }
    }
}

#[derive(Properties)]
pub struct SwitchProperties {
    path: &'static str,
    children: Children,
}

pub struct Switch;

impl Component for Switch {
    type Message = ();
    type Properties = SwitchProperties;

    fn create() -> Switch { Switch }

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
            <template {
                if pathname == properties.path {
                    properties.children.children()
                } else {
                    Vec::new()
                }
            }/>
        }
    }
}


