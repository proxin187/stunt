use crate::dom::component::Component;
use crate::dom::tree::Tree;

use std::sync::{LazyLock, Mutex};


pub struct State {
    component: Box<dyn Component>,
    tree: Tree,
}

impl State {
    pub fn new(component: Box<dyn Component>, tree: Tree) -> State {
        State {
            component,
            tree,
        }
    }

    pub fn render(&self) -> String {
        self.tree.render()
    }
}

pub fn with<R>(f: impl FnOnce(&mut Vec<State>) -> R) -> R {
    thread_local! {
        static STATE: LazyLock<Mutex<Vec<State>>> = LazyLock::new(|| Mutex::new(Vec::new()));
    }

    STATE.with(|scheduler| f(&mut scheduler.lock().expect("failed to lock")))
}


