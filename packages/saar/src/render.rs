use crate::dom::component::{Component, Context};
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
    fn render(&self, identity: Identity) {
        let root = state::get(&identity);

        let render = root.view(Context::new(Props::new(Vec::new()), Attributes::new(Vec::new()), identity)).render();

        // TODO: we will have to get the view here instead

        web_sys::console::log_1(&format!("render: {:?}", render).into());

        self.body.set_inner_html(&render);
    }

    pub fn init(self) -> Result<(), JsValue> {
        state::get_or_insert(Identity::new(0), Box::new(|| Arc::new(T::create())));

        self.render(Identity::new(0));

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


