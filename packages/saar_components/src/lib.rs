use saar::html::{Html, ComponentRef};
use saar::dom::component::Component;
use saar::dom::tree::Context;

use std::any::Any;


macro_rules! create_component {
    ($name:ident, $tag:expr) => {
        pub struct $name;

        impl Component for $name {
            fn create() -> $name { $name }

            fn callback(&mut self, _callback: Box<dyn Any>) {}

            fn extract(&self, _extract: Box<dyn Any>) -> String { $tag.to_string() }

            // TODO: we pass the wrong context to the props render
            //
            // the issue is obviously that we pass the context of the h1 to its own props, which is
            // obviously wrong.

            fn view(&self, ctx: Context) -> Html {
                Html::new(
                    ComponentRef::Block(|| { format!("<{} {}>{}</{}>", $tag, ctx.attributes.render(), ctx.props.render(ctx.clone()), $tag) }),
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


