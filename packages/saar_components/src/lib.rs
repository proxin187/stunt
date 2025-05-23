use saar::dom::component::{Component, ComponentRef, Context};
use saar::dom::html::{Html, Props, Attributes};

use std::any::Any;


macro_rules! create_component {
    ($name:ident, $tag:expr) => {
        pub struct $name;

        impl Component for $name {
            fn create() -> $name { $name }

            fn callback(&mut self, _callback: Box<dyn Any>) {}

            fn view(&self, ctx: Context) -> Html {
                let inner = ctx.props.render();
                let attr = ctx.attributes.render();

                Html::new(
                    ComponentRef::Block(Box::new(move || { format!("<{} {}>{}</{}>", $tag, attr, inner, $tag) })),
                    Attributes::new(Vec::new()),
                    Vec::new(),
                    Props::new(Vec::new()),
                )
            }
        }
    }
}

create_component!(H1, "h1");
create_component!(Div, "div");
create_component!(Button, "button");


