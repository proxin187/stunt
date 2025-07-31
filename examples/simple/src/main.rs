mod account;
mod theme;

use theme::{Theme, ThemeState};
use account::Account;

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
                <? { "<script>alert(1);</script>" } ?>
            </Router>
        }
    }
}

fn main() {
    Renderer::<App>::new().render();
}


