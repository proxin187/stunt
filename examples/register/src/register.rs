use stunt::prelude::*;
use stunt::frontend::html::node_id::NodeId;
use stunt::backend::Service;

use serde::{Serialize, Deserialize};


#[derive(Clone, Serialize, Deserialize)]
pub struct Response {
    user_id: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Api {
    username: String,
    password: String,
}

impl Api {
    pub fn new(username: String, password: String) -> Api {
        Api {
            username,
            password,
        }
    }
}

impl Service for Api {
    const PATH: &'static str = "/api/register";

    type Output = Response;

    fn handle(self) -> Response {
        Response {
            user_id: self.username.bytes().fold(0, |acc, byte| acc * byte as usize) ^ self.password.bytes().fold(0, |acc, byte| acc * byte as usize),
        }
    }
}

pub enum Message {
    Response(Response),
    Register,
}

pub struct Register {
    username: NodeId,
    password: NodeId,
}

impl Component for Register {
    type Message = Message;
    type Properties = ();

    fn create() -> Register {
        Register {
            username: NodeId::new(),
            password: NodeId::new(),
        }
    }

    fn callback(&mut self, message: &Message, link: Link) {
        match message {
            Message::Response(response) => {
                web_sys::console::log_1(&format!("user_id: {}", response.user_id).into());

                // TODO: here we have to redirect
            },
            Message::Register => {
                let username = self.username.cast::<web_sys::HtmlInputElement>().expect("failed to cast").value();
                let password = self.password.cast::<web_sys::HtmlInputElement>().expect("failed to cast").value();

                Api::new(username, password)
                    .call(move |response| link.callback::<Register>(Message::Response(response)));
            },
        }
    }

    fn view(&self, _: ()) -> Html {
        html! {
            <div>
                <input id={ self.username } placeholder={ "username" } />
                <input id={ self.password } placeholder={ "password" } />
                <button onclick={ Message::Register }>
                    { "register" }
                </button>
            </div>
        }
    }
}


