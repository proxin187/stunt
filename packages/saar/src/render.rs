use crate::dom::html::Html;

use wasm_bindgen::prelude::*;

use std::sync::LazyLock;


pub static RENDERER: LazyLock<Option<Renderer>> = LazyLock::new(|| None);


pub struct Renderer {
    base: Html,
}

impl Renderer {
    pub fn new(base: Html) -> Renderer {
        Renderer {
            base,
        }
    }

    pub fn render(&mut self) -> Result<(), JsValue> {
        // TODO: render system where it only updates what hasnt already been updated

        let raw = self.base.render();

        web_sys::console::log_1(&format!("raw: {:?}", raw).into());

        let window = web_sys::window().expect("no global window exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        body.set_inner_html(&raw);

        Ok(())
    }
}


