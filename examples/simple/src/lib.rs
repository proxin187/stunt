use saar::component::{Component, ComponentRef, Context, Callback};
use saar::render::Renderer;
use saar::html::Html;

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

    fn view<'a>(&self, ctx: Context<'a>) -> Html<'a> {
        Html::new(
            ComponentRef::Block(|| "<h1>Welcome to Saar Web Framework</h1>".to_string()),
            &[],
            ctx.props,
        )
    }
}

// TODO: the same issue where the webpage is frozen also occurs when not using any bundler, i think
// we can conclude that there is something wrong with the code

#[wasm_bindgen(start)]
fn main() {
    Renderer::<App>::new().render();
}


