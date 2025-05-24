use crate::dom::html::Html;
use crate::error::Error;

use std::sync::{LazyLock, Mutex, Arc};
use std::collections::BTreeMap;


static TREE: LazyLock<Mutex<>> = LazyLock::new(|| Mutex::new(BTreeMap::new()));

struct Tree {
    inner: BTreeMap<usize, Arc<Html>>,
    id: usize,
}

impl Tree {
    pub fn insert(id: usize, inner: Arc<Html>) {
        let mut lock = TREE.lock().expect("tree failed");

        lock.inner.insert(id, inner);
    }

    pub fn get(id: usize) -> Result<Arc<Html>, Error> {
        let lock = TREE.lock().expect("tree failed");

        lock.inner.get(&id)
        .ok_or(Error::InvalidId)
        .map(|html| html.clone())
}
}

pub fn alloc_id() -> usize {
}


