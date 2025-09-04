mod account;

use account::Account;

use stunt::prelude::*;
use stunt_router::Routable;


#[derive(Routable)]
pub enum Route {
    #[at("/account/:id/:name")]
    Account {
        id: usize,
        name: String,
    },
    #[not_found]
    NotFound,
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


