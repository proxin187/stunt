use puri::prelude::*;


#[derive(Properties)]
pub struct RouteProperties {
    path: String,
    children: Children,
}

pub struct Route;

impl Component for Route {
    type Message = ();
    type Properties = RouteProperties;

    fn create() -> Route { Route }

    fn callback(&mut self, _message: &()) {}

    fn view(&self, ctx: Context, properties: RouteProperties) -> Tree {
        let pathname = web_sys::window()
            .expect("no window found")
            .location()
            .pathname()
            .expect("failed to get pathname");

        html! {
            <template {
                if pathname == properties.path {
                    properties.children.children()
                } else {
                    vec![
                        html! { <template { "" } /> }
                    ]
                }
            }/>
        }
    }
}


