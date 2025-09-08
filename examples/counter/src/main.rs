use stunt::prelude::*;


struct Register {
    username: String,
    id: usize,
}

mod services {
    use super::*;

    #[service("/api/register")]
    pub fn register(register: Register) {
    }
}

pub enum Message {
    Add,
}

pub struct App {
    count: usize,
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create() -> App {
        App {
            count: 0,
        }
    }

    fn callback(&mut self, message: &Message) {
        match message {
            Message::Add => {
                self.count += 1;

                services::register(Register {
                    username: String::from("user"),
                    id: 123,
                });
            },
        }
    }

    fn view(&self, _: ()) -> Html {
        html! {
            <div>
                <button onclick={ Message::Add } >
                    { "increment" }
                </button>
                <h1>
                    { self.count }
                </h1>
            </div>
        }
    }
}

#[stunt_main]
fn main() {
    Renderer::new::<App>().render();

    stunt::backend::Entry::new()
        .service(String::from("/api/register"), services::register);
}


