use crate::dom::component::Component;
use crate::dom::state;
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
    fn render(&self) {
        let raw = state::root().render();

        web_sys::console::log_1(&format!("raw: {:?}", raw).into());

        self.body.set_inner_html(&raw);

        web_sys::console::log_1(&format!("render done").into());
    }

    fn create(&self) {
        let component = T::create();

        let view = component.view();

        state::push(Arc::new(component), view);
    }

    pub fn init(self) -> Result<(), JsValue> {
        self.create();

        self.render();

        loop {
            let callback = scheduler::recv();

            // TODO: we have to have some way to represent the job, eg. say whether its a
            // callback or whatever it is
            //
            // maybe we can only support callbacks for now

            self.render();
        }
    }
}


