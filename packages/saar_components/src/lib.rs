use saar::dom::component::{Component, Context};
use saar::dom::tree::{Node, ComponentRef};
use saar::dom::state::Identity;

use std::any::Any;


macro_rules! create_component {
    ($name:ident, $tag:expr, $id:expr) => {
        pub struct $name;

        impl Component for $name {
            fn create() -> $name { $name }

            fn callback(&mut self, _callback: Box<dyn Any>) {}

            fn view(&self, ctx: Context) -> Node {
                Node::new(
                    $id,
                    ComponentRef::Block(Box::new(|| { format!("<{} {}>{}</{}>", $tag, ctx.attributes.render(), ctx.props.render(), $tag) })),
                    Vec::new(),
                    Vec::new(),
                )
            }
        }
    }
}

create_component!(H1, "h1", Identity::new(1));
create_component!(Div, "div", Identity::new(2));
create_component!(Button, "button", Identity::new(3));


