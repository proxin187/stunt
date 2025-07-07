use crate::component::state::{self, Identity};
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
                hook_callback(&identity, &callback);

                render::render();
            });

            if let Err(_) = element.add_event_listener_with_callback(&self.event, closure.as_ref().unchecked_ref()) {
                web_sys::console::log_1(&format!("failed to set callback on id: {}", self.identity.render()).into());
            }

            closure.forget();
        }
    }
}

fn hook_callback(identity: &Identity, callback: &Arc<dyn Any + Send + Sync>) {
    let component = state::get(&identity.outer());

    component.lock().callback(callback);
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


