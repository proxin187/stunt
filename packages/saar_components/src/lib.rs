use saar_core::dom::component::{Component, Context};
use saar_core::dom::tree::Node;

use saar_macro::html;

use std::sync::Arc;
use std::any::Any;


macro_rules! create_component {
    ($name:ident, $tag:expr) => {
        #[allow(non_camel_case_types)]
        pub struct $name;

        impl Component for $name {
            fn create() -> $name { $name }

            fn callback(&mut self, _callback: &Arc<dyn Any + Send + Sync>) {}

            fn view(&self, ctx: Context) -> Node {
                let tag = $tag;

                html! {
                    <template { format!("<{} id=\"{}\" {}>{}</{}>", tag, ctx.identity.render(), ctx.attributes.render(), ctx.props.render(), tag) } />
                }
            }
        }
    }
}

create_component!(h1, "h1");
create_component!(div, "div");
create_component!(button, "button");


