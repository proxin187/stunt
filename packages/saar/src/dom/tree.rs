use crate::error::Error;
use crate::html::Html;

use crate::dom::component::Component;

use std::sync::{LazyLock, Mutex};


// TODO: we can move the state into a seperate module

static STATE: LazyLock<Mutex<Vec<Box<dyn Component + Send>>>> = LazyLock::new(|| Mutex::new(Vec::new()));

struct State {
}


pub enum Inner {
    Component(usize),
    Block(fn() -> String),
}

impl Inner {
}

pub struct Attribute {
    key: String,
    value: fn() -> String,
}

pub struct Node {
    inner: Inner,
    attributes: Vec<Attribute>,
    props: Vec<Node>,
}

impl Node {
    pub fn new(html: Html) -> (Node, Vec<Box<dyn Component>>) {
        Node {
            inner: html.component
        }
    }

    pub fn render(&self) {
    }
}

pub struct Tree {
    node: Node,
    state: Vec<Box<dyn Component>>,
}

impl Tree {
    pub fn new<T: Component>(component: T) -> Tree {
        let html = component.view();

        Tree {
            node: Node::new(html),
        }
    }
}


/*
static TREE: LazyLock<Mutex<Tree>> = LazyLock::new(|| Mutex::new(Tree::new()));


// TODO: this implementation has horrible performance, make something better

pub struct Tree {
    inner: BTreeMap<usize, Arc<Html>>,
    id: usize,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            inner: BTreeMap::new(),
            id: 0,
        }
    }

    pub fn insert(&mut self, id: usize, inner: Arc<Html>) {
        self.inner.insert(id, inner);
    }

    pub fn get(&self, id: usize) -> Result<Arc<Html>, Error> {
        self.inner.get(&id)
            .ok_or(Error::InvalidId)
            .map(|html| html.clone())
    }

    pub fn alloc_id(&mut self) -> usize {
        self.id += 1;

        self.id
    }
}

pub fn with<R>(f: impl Fn(&mut Tree) -> R) -> R {
    let mut lock = TREE.lock().expect("tree failed");

    f(&mut lock)
}
*/


