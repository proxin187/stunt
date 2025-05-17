use saar::component::{Component, ComponentRef, Callback, Context};
use saar::html::Html;

// TODO: create a macro that automatically generates a template for all the default html components


macro_rules! component {
    ($name:ident, $tag:expr) => {
    }
}

component!(H1, "h1");

pub struct Message;

impl Callback for Message {}

pub struct H1 {
    count: usize,
}

impl Component for App {
    type Callback = Message;

    fn create() -> App {
        App {
            count: 0,
        }
    }

    fn callback(&mut self, _message: Message) {}

    fn view<'a>(&self, ctx: Context<'a>) -> Html<'a> {
        Html::new(
            ComponentRef::Block(|| "<h1>Welcome to Saar Web Framework</h1>".to_string()),
            &[],
            ctx.props,
        )
    }
}


