use crate::Route;

use stunt::prelude::*;
use stunt::frontend::html::node_id::NodeId;
use stunt::backend::Service;

use serde::{Serialize, Deserialize};


#[derive(Clone, Serialize, Deserialize)]
pub struct Response {
    user_id: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RegisterApi {
    username: String,
    password: String,
}

impl Service for RegisterApi {
    const PATH: &'static str = "/api/register";

    type Output = Response;

    #[cfg(not(target_arch = "wasm32"))]
    fn handle(self) -> Response {
        Response {
            user_id: self.username.bytes().fold(1, |acc, byte| acc * byte as usize) ^ self.password.bytes().fold(1, |acc, byte| acc * byte as usize),
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
                stunt_router::redirect(Route::Registered { user_id: response.user_id });
            },
            Message::Register => {
                let register = RegisterApi {
                    username: self.username.cast::<web_sys::HtmlInputElement>().expect("failed to cast").value(),
                    password: self.password.cast::<web_sys::HtmlInputElement>().expect("failed to cast").value(),
                };

                register.call(move |response| link.callback::<Register>(Message::Response(response)));
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


