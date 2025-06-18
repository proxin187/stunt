use saar::dom::component::Component;
use saar::dom::state::Identity;
use saar::dom::tree::Context;

use saar::html::{Html, Attribute, ComponentRef};
use saar::render::Renderer;

use saar_components::*;

use wasm_bindgen::prelude::*;

use std::sync::Arc;
use std::any::Any;


pub enum Message {
    Add,
}

pub enum Extract {
    Count,
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

    fn callback(&mut self, callback: Box<dyn Any>) {
        match callback.downcast_ref::<Message>() {
            Some(Message::Add) => {
                self.count += 1;
            },
            None => unreachable!(),
        }
    }

    fn view(&self, ctx: Context) -> Html {
        let count = self.count.to_string();

        Html::new(
            Identity::new(4),
            ComponentRef::Component(Arc::new(Div::create())),
            Vec::new(),
            Vec::new(),
            vec![
                Html::new(
                    Identity::new(5),
                    ComponentRef::Component(Arc::new(H1::create())),
                    vec![Attribute::new(String::from("style"), || { String::from("background-color: yellow;") })],
                    Vec::new(),
                    vec![
                        Html::new(
                            Identity::new(6),
                            ComponentRef::Block(|| { format!("Welcome to saar web framework demo: {}", count) }),
                            Vec::new(),
                            Vec::new(),
                            Vec::new(),
                        ),
                    ],
                ),
                Html::new(
                    Identity::new(7),
                    ComponentRef::Component(Arc::new(Button::create())),
                    Vec::new(),
                    Vec::new(),
                    vec![
                        Html::new(
                            Identity::new(9),
                            ComponentRef::Block(|| { String::from("increment") }),
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


