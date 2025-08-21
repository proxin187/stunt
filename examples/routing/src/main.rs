mod account;

use account::Account;

use stunt::prelude::*;
use stunt_router::*;


pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create() -> App { App }

    fn callback(&mut self, _: &()) {}

    fn view(&self, _: ()) -> Html {
        html! {
            <Router>
                <Switch<Account> path={ "/account/:id/:name" } />
            </Router>
        }
    }
}

fn main() {
    Renderer::<App>::new().render();
}


