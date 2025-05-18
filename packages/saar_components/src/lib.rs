use saar::component::{Component, ComponentRef, Callback, Context};
use saar::html::Html;

// TODO: create a macro that automatically generates a template for all the default html components

pub struct Message;

impl Callback for Message {}

macro_rules! create_component {
    ($name:ident, $tag:expr) => {
        pub struct $name;

        impl Component for $name {
            type Callback = Message;

            fn create() -> $name { $name }

            fn callback(&mut self, _message: Message) {}

            fn view(&self, ctx: Context) -> Html {
                Html::new(
                    ComponentRef::Block(|ctx| { format!("<{}>{}</{}>", $tag, ctx.props.render(), $tag) }),
                    &[],
                    ctx.props,
                )
            }
        }
    }
}

create_component!(H1, "h1");
create_component!(Div, "div");
create_component!(P, "p");


