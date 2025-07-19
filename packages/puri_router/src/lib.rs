use puri::prelude::*;


pub struct Properties {
}

pub struct Route {
}

impl Component for Route {
    type Message = ();

    fn create() -> Route {
        Route {}
    }

    fn callback(&mut self, _message: &()) {}

    fn view(&self, ctx: Context) -> Tree {
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


