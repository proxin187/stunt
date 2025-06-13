use crate::dom::component::Component;
use crate::dom::state::{self, Identity};
use crate::dom::tree::{Props, Attributes};
use crate::scheduler;

use web_sys::HtmlElement;

use wasm_bindgen::JsValue;

use std::marker::PhantomData;
use std::sync::Arc;


pub struct Renderer<T: Component + Send + Sync + 'static> {
    body: HtmlElement,
    _marker: PhantomData<T>,
}

impl<T: Component + Send + Sync + 'static> Renderer<T> {
    pub fn new() -> Renderer<T> {
        let window = web_sys::window().expect("no global window exists");
        let document = window.document().expect("should have a document on window");

        Renderer {
            body: document.body().expect("document should have a body"),
            _marker: PhantomData,
        }
    }

    #[inline]
    fn render(&self, root: Identity) {
        let render = state::get(root).render(Props::new(Vec::new()), Attributes::new(Vec::new()));

        web_sys::console::log_1(&format!("render: {:?}", render).into());

        self.body.set_inner_html(&render);
    }

    fn create(&self) -> Identity {
        let component = T::create();

        let view = component.view();

        state::push(Arc::new(component), view)
    }

    pub fn init(self) -> Result<(), JsValue> {
        let root = self.create();

        self.render(root);

        /*
        loop {
            let callback = scheduler::recv();

            // TODO: we have to have some way to represent the job, eg. say whether its a
            // callback or whatever it is
            //
            // maybe we can only support callbacks for now

            self.render(root);
        }
        */

        Ok(())
    }
}


