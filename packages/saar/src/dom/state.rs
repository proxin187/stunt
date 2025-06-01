use crate::dom::component::Component;
use crate::dom::tree::Tree;

use std::sync::{Arc, LazyLock, Mutex};


pub struct State {
    pub component: Arc<dyn Component>,
    pub tree: Tree,
}

impl State {
    pub fn new(component: Arc<dyn Component>, tree: Tree) -> State {
        State {
            component,
            tree,
        }
    }

    pub fn render(&self) -> String {
        self.tree.render()
    }
}

// TODO: the issue is when we nest this function, aka call itself within the clousure that is
// passed
//
// this might be related to thread_local or something, it might be a fix to just define it as a
// normal static instead
pub fn with<R>(f: impl FnOnce(&mut Vec<State>) -> R) -> R {
    thread_local! {
        static STATE: LazyLock<Mutex<Vec<State>>> = LazyLock::new(|| Mutex::new(Vec::new()));
    }

    STATE.with(|state| f(&mut state.lock().expect("failed to lock")))
}


