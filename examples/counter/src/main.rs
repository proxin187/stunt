use stunt::prelude::*;
use stunt::backend::{Service, ServiceCaller, NullTransport};

use serde::{Serialize, Deserialize};


#[service("/api/register")]
pub struct Register {
    username: String,
    id: usize,
}

impl ServiceCaller for Register {
    fn path() -> &'static str { "/api/register" }

    fn call(&self) -> impl Future<Output = Result<Self::Output, Box<dyn std::error::Error>>> {
        let mut response = stunt::backend::__macro::ureq::post("127.0.0.1/api/register")
            .send_json(self)?;

        Ok(response.body_mut().read_json::<Self::Output>()?)
    }
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

async fn main() {
    if cfg!(target_arch = "wasm32") {
        Renderer::new::<App>().render();
    } else {
    }
}


