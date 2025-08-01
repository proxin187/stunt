pub mod component;
pub mod render;
pub mod global;
pub mod error;
mod vdom;

pub use spin::Mutex;

pub use stunt_macro;


pub mod prelude {
    pub use crate::component::{Component, Context, Properties};
    pub use crate::component::tree::{Tree, Children, AttrMap};
    pub use crate::render::Renderer;
    pub use crate::global;

    pub use crate::stunt_macro::{html, Properties};
}


