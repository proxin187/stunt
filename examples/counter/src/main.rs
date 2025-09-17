#![no_main]

use stunt::prelude::*;
use stunt::backend::{Service, NullTransport};

use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;


#[derive(Serialize, Deserialize)]
pub struct Register {
    username: String,
    id: usize,
}

impl Register {
    pub fn new(username: String, id: usize) -> Register {
        Register {
            username,
            id,
        }
    }
}

impl Service for Register {
    const PATH: &'static str = "/api/register";

    type Output = NullTransport;

    fn handle(self) -> Result<NullTransport, Box<dyn std::error::Error>> {
        Ok(NullTransport)
    }
}

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

    async fn callback(&mut self, message: &Message) {
        match message {
            Message::Add => {
                self.count += 1;

                if let Ok(register) = Register::new(String::from("user"), 123).call().await {
                }
            },
        }
    }

    fn view(&self, _: ()) -> Html {
        html! {
            <div>
                <button onclick={ Message::Add } >
                    { "increment" }
                </button>
                <h1>
                    { self.count }
                </h1>
            </div>
        }
    }
}

#[wasm_bindgen]
pub async fn main() {
    if cfg!(target_arch = "wasm32") {
        Renderer::new::<App>().render();
    } else {
    }
}


