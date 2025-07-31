mod account;
mod theme;

use account::{Account, AccountProperties};
use theme::{Theme, ThemeState};

use puri::prelude::*;
use puri_router::{Router, Switch};


pub enum Message {
    Add,
}

pub struct App {
    count: usize,
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create() -> App {
        App {
            count: 0,
        }
    }

    fn callback(&mut self, message: &Message) {
        match message {
            Message::Add => {
                self.count += 2;

                global::update_global::<Theme>(|theme| {
                    match theme.state {
                        ThemeState::Light => Theme {
                            state: ThemeState::Dark,
                            background: String::from("#000000ff"),
                        },
                        ThemeState::Dark => Theme {
                            state: ThemeState::Light,
                            background: String::from("#ffffffff"),
                        },
                    }
                });
            },
        }
    }

    fn view(&self, ctx: Context, _properties: ()) -> Tree {
        let theme = global::use_global::<Theme>();

        html! {
            <Router>
                <Switch<Account> path={ "/settings/account/:id/:name" }></Switch>
            </Router>
        }
    }
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // NOTE: for some magic reason it only works when you put a console log at the start wtf, thats so
    // weird

    web_sys::console::log_1(&"loading wasm".into());

    Renderer::<App>::new().init()
}


