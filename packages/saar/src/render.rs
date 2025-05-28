use crate::dom::component::{Component, Context};
use crate::html::{Props, Attributes};
use crate::dom::tree::Tree;
use crate::scheduler;

use web_sys::HtmlElement;

use wasm_bindgen::JsValue;


pub struct Renderer<T: Component> {
    component: T,
    body: HtmlElement,
}

impl<T: Component> Renderer<T> {
    pub fn new() -> Renderer<T> {
        let window = web_sys::window().expect("no global window exists");
        let document = window.document().expect("should have a document on window");

        Renderer {
            component: T::create(),
            body: document.body().expect("document should have a body"),
        }
    }

    fn render(&mut self) {
        let props = Props::new(Vec::new());
        let attributes = Attributes::new(Vec::new());

        let raw = self.component.view(Context::new(&props, &attributes)).render();

        web_sys::console::log_1(&format!("raw: {:?}", raw).into());

        self.body.set_inner_html(&raw);
    }

    pub fn init(mut self) -> Result<(), JsValue> {
        // TODO: render system where it only updates what hasnt already been updated

        // TODO: we can trigger a render at the start by generating a callback for the base element

        let tree = Tree::new(self.component);

        loop {
            match scheduler::with(|scheduler| scheduler.recv()) {
                Ok(callback) => {
                    // TODO: we have to have some way to represent the job, eg. say whether its a
                    // callback or whatever it is
                    //
                    // maybe we can only support callbacks for now

                    self.render();
                },
                Err(err) => {
                    web_sys::console::log_1(&format!("scheduler error: {:?}", err).into());
                },
            }
        }
    }
}


