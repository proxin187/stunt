use crate::dom::component::Component;
use crate::dom::tree::Tree;
use crate::html::Html;

use std::sync::{Arc, LazyLock, Mutex};

static STATE: LazyLock<Mutex<Vec<State>>> = LazyLock::new(|| Mutex::new(Vec::new()));


#[derive(Clone)]
pub struct State {
    pub component: Arc<dyn Component + Send + Sync>,
    pub tree: Arc<Tree>,
}

impl State {
    pub fn new(component: Arc<dyn Component + Send + Sync>, tree: Arc<Tree>) -> State {
        State {
            component,
            tree,
        }
    }

    pub fn render(&self) -> String {
        self.tree.render()
    }
}

pub fn get(index: usize) -> State {
    STATE.lock().expect("failed to lock")[index].clone()
}

pub fn push(component: Arc<dyn Component + Send + Sync>, view: Html) -> usize {
    // TODO: there is a literal bug with mutexes on wasm, rust wasm literally does not support
    // mutexes and there is no way to fix the error
    //
    // TODO: we will have to make our own mutex implementation i guess lol

    let mut state = STATE.lock().expect("failed to lock");

    let len = state.len();

    state.push(State::new(component, Arc::new(Tree::new(view, len))));

    state.len() - 1
}


