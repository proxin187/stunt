use saar::html::{Html, ComponentRef};
use saar::dom::component::Component;

use std::any::Any;


macro_rules! create_component {
    ($name:ident, $tag:expr) => {
        pub struct $name;

        impl Component for $name {
            fn create() -> $name { $name }

            fn callback(&mut self, _callback: Box<dyn Any>) {}

            fn extract(&self, _extract: Box<dyn Any>) -> String { String::new() }

            fn view(&self) -> Html {
                Html::new(
                    ComponentRef::Block(|ctx| { format!("<{} {}>{}</{}>", $tag, ctx.attributes.render(), ctx.props.render(), $tag) }),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                )
            }
        }
    }
}

create_component!(H1, "h1");
create_component!(Div, "div");
create_component!(Button, "button");


