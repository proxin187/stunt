use crate::component::{Component, Context};

use web_sys::HtmlElement;


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

    pub fn render(&mut self) {
        web_sys::console::log_1(&"starting rendering".into());

        let raw = self.component.view(Context::new(&[])).render();

        self.body.set_inner_html(&raw);

        loop {
        }
    }
}


