//! A [`NodeId`] can be attached to an HTML element, you can then get the DOM element that the [`NodeId`] is attached to.

use wasm_bindgen::JsCast;

use std::sync::atomic::{Ordering, AtomicU64};

static ID: AtomicU64 = AtomicU64::new(0);

/// A NodeId allows you to access an element from the dom.
#[derive(Clone, Copy)]
pub struct NodeId {
    id: u64,
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_str(&self.id.to_string())
    }
}

impl NodeId {
    /// Create a new unique [`NodeId`].
    pub fn new() -> NodeId {
        NodeId {
            id: ID.fetch_add(1, Ordering::Relaxed),
        }
    }

    /// Cast the [`NodeId`] into a Element
    pub fn cast<T: JsCast>(&self) -> Option<T> {
        let window = web_sys::window().expect("no global window exists");
        let document = window.document().expect("should have a document on window");

        document.get_element_by_id(&self.id.to_string())
            .and_then(|element| element.dyn_into::<T>().ok())
    }
}


