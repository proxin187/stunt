use crate::dom::component::{Component, Context};
use crate::dom::tree::{Props, Attributes};
use crate::dom::state::{self, Identity};
use crate::dom::callback;

use wasm_bindgen::JsValue;
use spin::Mutex;

use std::marker::PhantomData;
use std::sync::Arc;


pub struct Renderer<T: Component + Send + Sync + 'static> {
    _marker: PhantomData<T>,
}

impl<T: Component + Send + Sync + 'static> Renderer<T> {
    pub fn new() -> Renderer<T> {
        Renderer {
            _marker: PhantomData,
        }
    }

    pub fn init(self) -> Result<(), JsValue> {
        state::get_or_insert(&Identity::new(0), || Arc::new(Mutex::new(T::create())));

        render();

        Ok(())
    }
}

// TODO: make the render function only update the part of the dom that is different instead of
// updating everything like we do here

#[inline]
pub fn render() {
    let identity = Identity::new(0);

    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let root = state::get(&identity);
    let lock = root.lock();

    let render = lock.view(Context::new(Props::new(Vec::new()), Attributes::new(Vec::new()), identity)).render();

    web_sys::console::log_1(&format!("render: {:?}", render).into());

    body.set_inner_html(&render);

    callback::flush();
}


