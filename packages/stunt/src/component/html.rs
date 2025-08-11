use crate::component::tree::{AttrMap, Tree, Element, Template};
use crate::component::BaseComponent;

use std::sync::Arc;
use std::any::Any;

use spin::Mutex;


/// Represents a component, template or element.
///
/// ## Warning
/// This enum is not supposed to be used outside of the framework.
#[derive(Clone)]
pub enum ChildKind {
    #[allow(missing_docs)]
    Component {
        builder: fn() -> Arc<Mutex<dyn BaseComponent + Send + Sync>>,
        name: String,
    },

    #[allow(missing_docs)]
    Template(Arc<dyn Template>),

    #[allow(missing_docs)]
    Element(Element),
}

/// Represents a child within a [`view`](Component::view).
pub struct ChildNode {
    kind: ChildKind,
    callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>,
    attributes: AttrMap,
}

impl ChildNode {
    pub fn new(
        kind: ChildKind,
        callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>,
        attributes: AttrMap
    ) -> ChildNode {
        ChildNode {
            kind,
            callbacks,
            attributes,
        }
    }
}

/// Reference to a [`ChildNode`]
pub struct ChildRef {
    index: usize,
    children: Vec<ChildRef>,
}

impl ChildRef {
    pub fn new(index: usize, children: Vec<ChildRef>) -> ChildRef {
        ChildRef {
            index,
            children,
        }
    }
}

/// Html is a representation of the layout of a [`view`](Component::view).
pub struct Html {
    children: Vec<Tree>,
    layout: ChildRef,
}

impl Html {
    /// Create a new Html tree
    pub fn new(children: Vec<Tree>, layout: ChildRef) -> Html {
        Html {
            children,
            layout,
        }
    }

    fn build_tree(&self) {
    }
}

