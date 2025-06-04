use crate::dom::component::Component;
use crate::dom::tree::Tree;
use crate::html::Html;

use std::sync::{Arc, LazyLock, Mutex};

static STATE: LazyLock<Arc<Mutex<Vec<State>>>> = LazyLock::new(|| Arc::new(Mutex::new(Vec::new())));


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
    // TODO: we managed to enable the atomic flags, now the only thing left is that atomics dont
    // work in the main thread, so we will have to somehow spawn the renderer in a seperate thread

    let mut state = STATE.lock().expect("failed to lock");

    let len = state.len();

    state.push(State::new(component, Arc::new(Tree::new(view, len))));

    state.len() - 1
}


