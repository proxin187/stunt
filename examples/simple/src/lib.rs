use saar::dom::component::{Component, ComponentRef, Context};
use saar::dom::html::{Html, Props, Attributes};
use saar::render::Renderer;

use saar_components::*;

use wasm_bindgen::prelude::*;

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

    fn callback(&mut self, callback: Box<dyn Any>) {
        match callback.downcast_ref::<Message>() {
            Some(Message::Add) => {
                self.count += 1;
            },
            None => unreachable!(),
        }
    }

    fn view(&self, ctx: Context) -> Html {
        let count = self.count;

        Html::new(
            ComponentRef::Component(Box::new(Div::create())),
            Attributes::new(Vec::new()),
            Vec::new(),
            Props::new(vec![
                Html::new(
                    ComponentRef::Component(Box::new(H1::create())),
                    Attributes::new(vec![(String::from("style"), || { String::from("background-color: yellow;") })]),
                    Vec::new(),
                    Props::new(vec![
                        Html::new(
                            ComponentRef::Block(Box::new(move || format!("Welcome to saar web framework demo: {}", count))),
                            Attributes::new(Vec::new()),
                            Vec::new(),
                            Props::new(Vec::new()),
                        ),
                    ]),
                ),
                Html::new(
                    ComponentRef::Component(Box::new(Button::create())),
                    Attributes::new(Vec::new()),
                    Vec::new(),
                    Props::new(vec![
                        Html::new(
                            ComponentRef::Block(Box::new(move || String::from("increment"))),
                            Attributes::new(Vec::new()),
                            Vec::new(),
                            Props::new(Vec::new()),
                        ),
                    ]),
                ),
            ]),
        )
    }
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // NOTE: for some magic reason it only works when you put a console log at the start wtf, thats so
    // weird

    web_sys::console::log_1(&"loading wasm".into());

    Renderer::<App>::new().render()
}


