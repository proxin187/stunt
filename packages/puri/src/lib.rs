pub use puri_core;
pub use puri_macro;


pub mod prelude {
    pub use puri_core;

    pub use puri_core::component::{Component, Context, Properties};
    pub use puri_core::component::tree::{Tree, AttrMap};
    pub use puri_core::render::Renderer;
    pub use puri_core::global;

    pub use puri_core::wasm_bindgen::prelude::*;

    pub use puri_core::wasm_bindgen;
    pub use puri_core::web_sys;

    pub use puri_macro::html;
}


