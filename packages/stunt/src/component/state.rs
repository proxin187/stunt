//! The state of each component is stored globally with each its own [`Identity`].

use crate::component::BaseComponent;

use std::sync::{Arc, LazyLock};
use std::collections::HashMap;

use spin::Mutex;

static STATES: LazyLock<Arc<Mutex<HashMap<Path, Arc<Mutex<dyn BaseComponent + Send + Sync>>>>>> = LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct PathNode {
    index: usize,
    name: String,
}

impl PathNode {
    pub(crate) fn new(index: usize, name: String) -> PathNode {
        PathNode {
            index,
            name,
        }
    }
}

/// Describes a path from root to an element.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq)]
pub struct Path {
    nodes: Vec<PathNode>,
}

impl Path {
    pub(crate) fn new() -> Path {
        Path {
            nodes: Vec::new(),
        }
    }

    pub(crate) fn concat(mut self, node: PathNode) -> Path {
        self.nodes.push(node);

        Path {
            nodes: self.nodes,
        }
    }

    pub(crate) fn selector(&self) -> String {
        self.nodes.iter()
            .map(|node| format!(":nth-child({}) > ", node.index))
            .collect::<String>()
    }
}

#[inline]
pub(crate) fn get(path: &Path) -> Arc<Mutex<dyn BaseComponent + Send + Sync>> {
    STATES.lock()[path].clone()
}

#[inline]
pub(crate) fn get_or_insert(
    path: &Path,
    f: impl Fn() -> Arc<Mutex<dyn BaseComponent + Send + Sync>>
) -> Arc<Mutex<dyn BaseComponent + Send + Sync>> {
    let mut states = STATES.lock();

    match states.get(path) {
        Some(component) => component.clone(),
        None => {
            states.insert(path.clone(), (f)());

            states[&path].clone()
        },
    }
}


