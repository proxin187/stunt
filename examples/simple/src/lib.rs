use puri::prelude::*;


pub enum Message {
    Add,
}

pub struct App {
    count: usize,
}

impl Component for App {
    type Message = Message;

    fn create() -> App {
        App {
            count: 0,
        }
    }

    fn callback(&mut self, message: &Message) {
        match message {
            Message::Add => {
                self.count += 2;
            },
        }
    }

    fn view(&self, ctx: Context) -> Tree {
        html! {
            <div>
                <h1 style={ "" }>
                    <template { format!("count: {}", self.count) } />
                </h1>
                <button class={ "btn" } event: mousedown={ Message::Add }>
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


