#[allow(unused_imports)]
use web_sys;

use saar::component::{Component, BaseComponent, ComponentRef, Callback};
use saar::render::Renderer;
use saar::html::Html;


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

    fn view(&self) -> Html {
        Html::new(
            ComponentRef::Base(BaseComponent::new("h1", &[])),
            &[],
            Vec::new(),
        )
    }
}

fn main() {
    Renderer::<App>::new().render();
}


