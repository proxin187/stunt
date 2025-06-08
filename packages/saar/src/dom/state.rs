use crate::dom::component::Component;
use crate::dom::tree::Tree;
use crate::html::Html;

use std::sync::{Arc, LazyLock};

use spin::Mutex;

static STATES: LazyLock<Arc<Mutex<Vec<State>>>> = LazyLock::new(|| Arc::new(Mutex::new(Vec::new())));


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

pub fn root() -> State {
    let states = STATES.lock();

    states[states.len() - 1].clone()
}

pub fn get(index: usize) -> State {
    STATES.lock()[index].clone()
}

pub fn push(component: Arc<dyn Component + Send + Sync>, view: Html) -> usize {
    let len = STATES.lock().len();

    // TODO: the len that we set here is actually wrong, this is because the len will be updated by
    // all the inner elements before we actually push this one
    //
    // maybe we can have tree return the index?
    let new = State::new(component, Arc::new(Tree::new(view, len)));

    let mut states = STATES.lock();

    states.push(new);

    web_sys::console::log_1(&format!("len: {:?}", states.len() - 1).into());

    states.len() - 1
}


