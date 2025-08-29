mod account;

use account::Account;

use stunt::prelude::*;
use stunt_router::*;


pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create() -> App { App }

    fn view(&self, _: ()) -> Html {
        html! {
            <Router>
                <Account id={ 123 } name={ String::from("test hello") } />
            </Router>
        }
    }
}

fn main() {
    Renderer::new::<App>().render();
}


