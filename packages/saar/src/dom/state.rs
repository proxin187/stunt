use crate::dom::tree::{Tree, Context};
use crate::dom::component::Component;
use crate::html::Html;

use std::sync::{Arc, LazyLock};
use std::collections::HashMap;

use spin::Mutex;

static STATES: LazyLock<Arc<Mutex<HashMap<Identity, State>>>> = LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

static IDENTITY: LazyLock<Arc<Mutex<Identity>>> = LazyLock::new(|| Arc::new(Mutex::new(Identity::new())));


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Identity {
    id: usize,
}

impl Identity {
    pub fn new() -> Identity {
        Identity {
            id: 0,
        }
    }

    pub fn alloc(&mut self) -> Identity {
        self.id += 1;

        Identity {
            id: self.id - 1,
        }
    }
}

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

pub fn get(identity: Identity) -> State {
    STATES.lock()[&identity].clone()
}

pub fn push(component: Arc<dyn Component + Send + Sync>, view: Html) -> Identity {
    let identity = IDENTITY.lock().alloc();

    let new = State::new(component, Arc::new(Tree::new(view, identity)));

    STATES.lock().insert(identity, new);

    identity
}


