// mod account;
mod theme;

use theme::{Theme, ThemeState};
// use account::Account;

use stunt::prelude::*;
use stunt::global;

// use stunt_router::{Router, Switch};


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
                self.count += 1;

                global::use_global(|theme: &mut Theme| {
                    match theme.state {
                        ThemeState::Light => {
                            theme.background = String::from("#000000ff");
                        },
                        ThemeState::Dark => {
                            theme.background = String::from("#ffffffff");
                        },
                    }
                });
            },
        }
    }

    fn view(&self, _properties: ()) -> Html {
        let theme = global::use_global(|theme: &mut Theme| theme.clone());

        html! {
            <div>
                { "<script>alert(1);</script>" }
                <button class={ "btn" } onclick={ Message::Add } >
                    { "increment" }
                </button>
                { format!("count: {}", self.count) }
            </div>
        }
    }
}

fn main() {
    Renderer::<App>::new().render();
}


