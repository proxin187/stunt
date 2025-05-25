use crate::dom::html::Html;
use crate::error::Error;

use std::sync::{LazyLock, Mutex, Arc};
use std::collections::BTreeMap;


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


