use crate::component::tree::{Props, Attributes};
use crate::component::state::{self, Identity};
use crate::component::{Component, Context};
use crate::component::callback;
use crate::vdom;

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

#[inline]
pub fn render() {
    let identity = Identity::new(0);

    let root = state::get(&identity);
    let lock = root.lock();

    let render = lock.view(Context::new(Props::new(Vec::new()), Attributes::new(Vec::new()), identity)).render();

    vdom::reconcile(render);

    callback::flush();
}


