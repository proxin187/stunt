use crate::dom::state::{self, Identity};
use crate::dom::tree::{Props, Attributes};
use crate::dom::component::Context;
use crate::render;

use std::sync::{Arc, LazyLock};
use std::any::Any;

use spin::Mutex;

use wasm_bindgen::prelude::*;

static CALLBACKS: LazyLock<Arc<Mutex<Vec<Callback>>>> = LazyLock::new(|| Arc::new(Mutex::new(Vec::new())));


pub struct Callback {
    identity: Identity,
    event: String,
    callback: Arc<dyn Any + Send + Sync>,
}

impl Callback {
    pub fn new(identity: Identity, event: String, callback: Arc<dyn Any + Send + Sync>) -> Callback {
        Callback {
            identity,
            event,
            callback,
        }
    }

    pub fn flush(&self, document: &web_sys::Document) {
        if let Some(element) = document.get_element_by_id(&self.identity.render()) {
            let identity = self.identity.clone();
            let callback = self.callback.clone();

            let closure = Closure::<dyn Fn()>::new(move || {
                web_sys::console::log_1(&"clicked".into());

                let component = state::get(&identity.outer());

                let mut lock = component.lock();

                lock.callback(&callback);

                // TODO: re-render the component
            });

            if let Err(_) = element.add_event_listener_with_callback(&self.event, closure.as_ref().unchecked_ref()) {
                web_sys::console::log_1(&format!("failed to set callback on id: {}", self.identity.render()).into());
            }

            closure.forget();
        }
    }
}

#[inline]
pub fn push(identity: Identity, event: String, callback: Arc<dyn Any + Send + Sync>) {
    let callback = Callback::new(identity, event, callback);

    CALLBACKS.lock().push(callback);
}

#[inline]
pub fn flush() {
    let window = web_sys::window().expect("no window found");
    let document = window.document().expect("no document found");

    for callback in CALLBACKS.lock().drain(..) {
        callback.flush(&document);
    }
}


