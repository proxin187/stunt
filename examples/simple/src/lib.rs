use saar::html::{Html, Attribute, ComponentRef};
use saar::dom::component::Component;
use saar::dom::tree::Context;
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

    // TODO: this function is never called
    fn extract(&self, extract: Box<dyn Any>) -> String {
        web_sys::console::log_1(&"extract".into());

        match extract.downcast_ref::<Extract>() {
            Some(Extract::Count) => format!("{}", self.count),
            None => unreachable!(),
        }
    }

    fn view(&self) -> Html {
        Html::new(
            ComponentRef::Component(Arc::new(Div::create())),
            Vec::new(),
            Vec::new(),
            vec![
                Html::new(
                    ComponentRef::Component(Arc::new(H1::create())),
                    vec![Attribute::new(String::from("style"), || { String::from("background-color: yellow;") })],
                    Vec::new(),
                    vec![
                        Html::new(
                            ComponentRef::Block(|ctx| { format!("Welcome to saar web framework demo: {}", ctx.extract(Extract::Count)) }),
                            Vec::new(),
                            Vec::new(),
                            Vec::new(),
                        ),
                    ],
                ),
                Html::new(
                    ComponentRef::Component(Arc::new(Button::create())),
                    Vec::new(),
                    Vec::new(),
                    vec![
                        Html::new(
                            ComponentRef::Block(|_| { String::from("increment") }),
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


