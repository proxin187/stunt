use crate::dom::component::{Component, Context};
use crate::dom::tree::{Props, Attributes};
use crate::dom::state::{self, Identity};

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

        web_sys::console::log_1(&format!("render: {:?}", render).into());

        self.body.set_inner_html(&render);
    }

    pub fn init(self) -> Result<(), JsValue> {
        state::get_or_insert(&Identity::new(0), || Arc::new(T::create()));

        self.render(Identity::new(0));

        Ok(())
    }
}


