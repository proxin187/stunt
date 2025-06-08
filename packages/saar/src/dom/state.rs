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

pub fn get(index: usize) -> State {
    STATES.lock()[index].clone()
}

pub fn push(component: Arc<dyn Component + Send + Sync>, view: Html) -> usize {
    let len = STATES.lock().len();

    let new = State::new(component, Arc::new(Tree::new(view, len)));

    let mut states = STATES.lock();

    states.push(new);

    states.len() - 1
}


