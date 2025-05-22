use crate::dom::component::{Component, Context};
use crate::dom::html::{Props, Attributes};

use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use std::collections::HashMap;


pub struct Renderer<T: Component> {
    body: HtmlElement,
    component: T,
}

impl<T: Component> Renderer<T> {
    pub fn new() -> Renderer<T> {
        let window = web_sys::window().expect("no global window exists");
        let document = window.document().expect("should have a document on window");

        Renderer {
            body: document.body().expect("document should have a body"),
            component: T::create(),
        }
    }

    pub fn render(&mut self) -> Result<(), JsValue> {
        // TODO: render system where it only updates what hasnt already been updated

        let attributes = Attributes::new(Vec::new());
        let props = Props::new(Vec::new());

        let raw = self.component.view(Context::new(&props, &attributes)).render();

        web_sys::console::log_1(&format!("raw: {:?}", raw).into());

        self.body.set_inner_html(&raw);

        Ok(())
    }
}


