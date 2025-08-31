mod account;

use account::Account;

use stunt::prelude::*;
use stunt_router::Routable;


pub enum Route {
    Account {
        id: usize,
        name: String,
    },
    NotFound,
}

impl Routable for Route {
    fn route(path: &[&str]) -> Route {
        match path {
            ["api", "account", id, name] if <usize as std::str::FromStr>::from_str(id).is_ok() && <String as std::str::FromStr>::from_str(name).is_ok() => Route::Account {
                id: std::str::FromStr::from_str(id).expect("internal error"),
                name: std::str::FromStr::from_str(name).expect("internal error"),
            },
            _ => Route::NotFound,
        }
    }
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create() -> App { App }

    fn view(&self, _: ()) -> Html {
        match stunt_router::route::<Route>() {
            Route::Account { id, name } => html! { <Account id={ id } name={ name } /> },
            Route::NotFound => {
                html! {
                    <h1>
                        { "404: Not Found" }
                    </h1>
                }
            },
        }
    }
}

fn main() {
    Renderer::new::<App>().render();
}


