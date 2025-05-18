use saar::component::{Component, ComponentRef, Context, Callback};
use saar::html::{Html, Props};
use saar::render::Renderer;

use saar_components::*;

use wasm_bindgen::prelude::*;


pub enum Message {
    Add,
}

impl Callback for Message {}

pub struct App {
    count: usize,
}

impl Component for App {
    type Callback = Message;

    fn create() -> App {
        App {
            count: 0,
        }
    }

    fn callback(&mut self, message: Message) {
        match message {
            Message::Add => {
                self.count += 1;
            },
        }
    }

    fn view(&self, ctx: Context) -> Html {
        Html::new(
            ComponentRef::Component(Box::new(H1::create())),
            &[],
            Props::new(Vec::new()),
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


