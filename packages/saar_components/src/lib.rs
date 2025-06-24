use saar::dom::component::{Component, Context};
use saar::dom::tree::{Node, ComponentRef};
use saar::dom::state::Identity;

use std::sync::Arc;
use std::any::Any;


macro_rules! create_component {
    ($name:ident, $tag:expr, $id:expr) => {
        pub struct $name;

        impl Component for $name {
            fn create() -> $name { $name }

            fn callback(&mut self, _callback: &Arc<dyn Any + Send + Sync>) {}

            fn view(&self, ctx: Context) -> Node {
                Node::new(
                    ctx.identity.intersect($id),
                    ComponentRef::Template(format!("<{} id=\"{}\" {}>{}</{}>", $tag, ctx.identity.render(), ctx.attributes.render(), ctx.props.render(), $tag)),
                    Vec::new(),
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


