use saar::html::{Html, ComponentRef};
use saar::dom::component::Component;
use saar::dom::state::Identity;
use saar::dom::tree::Context;

use std::any::Any;


macro_rules! create_component {
    ($name:ident, $tag:expr, $id:expr) => {
        pub struct $name;

        impl Component for $name {
            fn create() -> $name { $name }

            fn callback(&mut self, _callback: Box<dyn Any>) {}

            fn view(&self, ctx: Context) -> Html {
                Html::new(
                    $id,
                    ComponentRef::Block(|| { format!("<{} {}>{}</{}>", $tag, ctx.attributes.render(), ctx.props.render(ctx.clone()), $tag) }),
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


