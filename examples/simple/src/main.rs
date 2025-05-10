use saar::component::Component;
use saar::html::Html;


pub enum Message {
}

pub struct App {
}

impl Component<Message> for App {
    fn update(&self, msg: Message) {
        match msg {
        }
    }

    fn view(&self) -> Html {
        Html {
        }
    }
}

fn main() {
    println!("Hello, world!");
}

