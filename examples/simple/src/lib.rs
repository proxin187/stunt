use saar::dom::component::{Component, Context};
use saar::dom::tree::{Node, ComponentRef};
use saar::dom::state::Identity;

use saar::render::Renderer;

use saar_components::*;

use wasm_bindgen::prelude::*;

use std::sync::Arc;
use std::any::Any;


pub enum Message {
    Add,
}

pub struct App {
    count: usize,
}

impl Component for App {
    fn create() -> App {
        App {
            count: 0,
        }
    }

    fn callback(&mut self, callback: &Arc<dyn Any + Send + Sync>) {
        web_sys::console::log_1(&format!("count: {}", self.count).into());

        match callback.downcast_ref::<Message>() {
            Some(Message::Add) => {
                self.count += 1;
            },
            None => unreachable!(),
        }
    }

    fn view(&self, ctx: Context) -> Node {
        Node::new(
            ctx.identity.intersect(Identity::new(4)),
            ComponentRef::Component(|| Arc::new(saar::Mutex::new(Div::create()))),
            Vec::new(),
            Vec::new(),
            vec![
                Node::new(
                    ctx.identity.intersect(Identity::new(5)),
                    ComponentRef::Component(|| Arc::new(saar::Mutex::new(H1::create()))),
                    Vec::new(),
                    vec![(String::from("style"), String::from("background-color: yellow;"))],
                    vec![
                        Node::new(
                            ctx.identity.intersect(Identity::new(6)),
                            ComponentRef::Template(format!("Welcome to saar web framework demo: {}", self.count)),
                            Vec::new(),
                            Vec::new(),
                            Vec::new(),
                        ),
                    ],
                ),
                Node::new(
                    ctx.identity.intersect(Identity::new(7)),
                    ComponentRef::Component(|| Arc::new(saar::Mutex::new(Button::create()))),
                    vec![(String::from("mousedown"), Arc::new(Message::Add))],
                    Vec::new(),
                    vec![
                        Node::new(
                            ctx.identity.intersect(Identity::new(8)),
                            ComponentRef::Template(String::from("increment")),
                            Vec::new(),
                            Vec::new(),
                            Vec::new(),
                        ),
                    ],
                ),
            ],
        )
    }
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // NOTE: for some magic reason it only works when you put a console log at the start wtf, thats so
    // weird

    web_sys::console::log_1(&"loading wasm".into());

    Renderer::<App>::new().init()
}


