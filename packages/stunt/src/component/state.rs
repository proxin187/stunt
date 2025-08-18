//! The state of each component is stored globally under its [`Path`].

use crate::component::BaseComponent;

use std::sync::{Arc, LazyLock};
use std::collections::HashMap;

use wasm_bindgen::prelude::*;
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

/// Describes a path from root to an element. This is used to build an XPath query during
/// reconciliation.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq)]
pub struct Path {
    nodes: Vec<PathNode>,
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let xpath = self.nodes.iter()
            .map(|node| format!("/*[{}]", node.index + 1))
            .collect::<String>();

        f.write_str(&xpath)
    }
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

    #[inline]
    pub(crate) fn get_element_by_path(&self, document: &web_sys::Document) -> Result<web_sys::HtmlElement, JsValue> {
        let node = document.evaluate(&format!("/html/body{}", self), &document.get_root_node())?
            .iterate_next()?
            .ok_or(JsValue::from_str("failed to get node"))?;

        node.dyn_ref::<web_sys::HtmlElement>()
            .map(|element| element.clone())
            .ok_or(JsValue::from_str("failed to cast"))
    }
}

#[inline]
pub(crate) fn get(path: &Path) -> Arc<Mutex<dyn BaseComponent + Send + Sync>> {
    STATES.lock()[path].clone()
}

#[inline]
pub(crate) fn get_or_insert(
    path: &Path,
    f: impl Fn() -> Arc<Mutex<dyn BaseComponent + Send + Sync>>,
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


