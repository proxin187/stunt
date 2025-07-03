use puri::prelude::*;

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
        match callback.downcast_ref::<Message>() {
            Some(Message::Add) => {
                self.count += 5;
            },
            None => unreachable!(),
        }
    }

    fn view(&self, ctx: Context) -> Node {
        html! {
            <div>
                <h1 style={ "background-color: blue;" }>
                    <template { format!("count: {}", self.count) } />
                </h1>
                <button event: mousedown={ Arc::new(Message::Add) }>
                    <template { format!("increment") } />
                </button>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // NOTE: for some magic reason it only works when you put a console log at the start wtf, thats so
    // weird

    web_sys::console::log_1(&"loading wasm".into());

    Renderer::<App>::new().init()
}


