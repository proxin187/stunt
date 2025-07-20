use puri::prelude::*;


pub struct RouteProperties {
}

impl Properties for RouteProperties {
    fn create(attributes: AttrMap) -> RouteProperties {
        RouteProperties {
        }
    }
}

pub struct Route {
}

impl Component for Route {
    type Message = ();
    type Properties = RouteProperties;

    fn create() -> Route {
        Route {}
    }

    fn callback(&mut self, _message: &()) {}

    fn view(&self, ctx: Context, properties: RouteProperties) -> Tree {
        let window = web_sys::window()
            .expect("no window found")
            .location()
            .pathname()
            .expect("failed to get pathname");

        html! {
            <h1></h1>
        }
    }
}


