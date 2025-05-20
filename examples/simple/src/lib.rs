use saar::component::{Component, ComponentRef, Context};
use saar::html::{Html, Props};
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
            ComponentRef::Component(Box::new(H1::create())),
            &[],
            Props::new(vec![
                Html::new(
                    ComponentRef::Block(Box::new(move || format!("Welcome to saar web framework demo: {}", count))),
                    &[],
                    Props::new(Vec::new()),
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


