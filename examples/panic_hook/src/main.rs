mod panic_hook;

use stunt::prelude::*;


pub enum Message {
    Panic,
}

pub struct App;

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create() -> App { App }

    fn callback(&mut self, message: &Message) {
        match message {
            Message::Panic => {
                panic!("you clicked the panic button");
            },
        }
    }

    fn view(&self, _: ()) -> Html {
        html! {
            <div>
                <button onclick={ Message::Panic }>
                    { "panic" }
                </button>
            </div>
        }
    }
}

fn main() {
    panic_hook::init();

    Renderer::new::<App>().render();
}


