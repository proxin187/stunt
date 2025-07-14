use puri_core::component::{Component, Context};
use puri_core::component::tree::{Tree, Element, ComponentRef};
use puri_core::component::state::Identity;

use puri_macro::next_id;

use std::sync::Arc;
use std::any::Any;


macro_rules! create_component {
    ($name:ident, $tag:expr) => {
        #[allow(non_camel_case_types)]
        pub struct $name;

        impl Component for $name {
            fn create() -> $name { $name }

            fn callback(&mut self, _callback: &Arc<dyn Any + Send + Sync>) {}

            fn view(&self, ctx: Context) -> Tree {
                Tree::new(
                    ctx.identity.intersect(Identity::new(next_id!())),
                    ComponentRef::Element(Element::new(String::from($tag), ctx.attributes.clone(), ctx.props.clone())),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                )
            }
        }
    }
}

create_component!(h1, "h1");
create_component!(div, "div");
create_component!(button, "button");


