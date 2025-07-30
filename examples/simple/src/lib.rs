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
            <Router<String>>
                <Switch<AccountProperties> path={ "/settings/account/:id/:name" }>
                    <Account></Account>
                </Switch>
                <Switch<()> path={ "/settings/theme" }>
                    <div>
                        <h1 style={ format!("background-color: {};", theme.background) }>
                            <? { format!("this is the theme count: {}", self.count) } ?>
                        </h1>
                        <button class={ "btn" } event: mousedown={ Message::Add }>
                            <? { format!("increment") } ?>
                        </button>
                    </div>
                </Switch>
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


