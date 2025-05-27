pub mod component;
pub mod html;

pub(crate) mod listener;
pub(crate) mod tree;

use component::Component;

use std::sync::{Arc, Mutex};


pub fn use_state<R>(f: impl Fn(&mut Vec<Box<dyn Component>>) -> R) -> R {
    thread_local! {
        static COMPONENTS: Arc<Mutex<Vec<Box<dyn Component>>>> = Arc::new(Mutex::new(Vec::new()));
    }

    COMPONENTS.with(|components| {
        f(&mut components.lock().expect("failed to lock"))
    })
}


