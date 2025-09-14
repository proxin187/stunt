use stunt::prelude::*;
use stunt::service::{Service, ServiceCaller, NullTransport};

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
#[service("/api/register")]
pub struct Register {
    username: String,
    id: usize,
}

impl ServiceCaller for Register {
    fn path() -> &'static str { "/api/register" }
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

    fn callback(&mut self, message: &Message) {
        match message {
            Message::Add => {
                self.count += 1;

                Register::new(String::from("user"), 123).call();
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

fn main() {
    if cfg!(target_arch = "wasm32") {
        Renderer::new::<App>().render();
    } else {
    }
}


