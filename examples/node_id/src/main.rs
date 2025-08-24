use stunt::prelude::*;


pub enum Message {
    Submit,
}

pub struct App {
    input: NodeId,
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create() -> App {
        App {
            input: NodeId::new(),
        }
    }

    fn callback(&mut self, message: &Message) {
        match message {
            Message::Submit => {
                if let Some(element) = self.input.cast::<web_sys::HtmlInputElement>() {
                    let _ = element.focus();
                }
            },
        }
    }

    fn view(&self, _: ()) -> Html {
        html! {
            <div>
                <input id={ self.input } placeholder={ "text" } />
                <button onclick={ Message::Submit } >
                    { "focus" }
                </button>
            </div>
        }
    }
}

fn main() {
    Renderer::new::<App>().render();
}


