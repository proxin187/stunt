//! The basic building blocks for building Html. This module is mostly used by macros.

use crate::virtual_dom::{VirtualNode, VirtualKind, VirtualElement};

use crate::component::state::{self, Path, PathNode};
use crate::component::{Component, BaseComponent};

use std::collections::HashMap;
use std::sync::Arc;
use std::any::Any;
use std::rc::Rc;

use spin::Mutex;

macro_rules! impl_t {
    ($($t:ty),+) => {
        trait NonTreeTemplate {}

        $(impl NonTreeTemplate for $t {})*
    }
}

impl_t!(&str, String, usize, u64, u32, u16, u8, isize, i128, i64, i32, i16, i8, f64, f32);

/// The template trait allows a type to be used as a template.
///
/// ## Warning
/// For the time being this trait is not supposed to be implemented outside the framework.
pub trait Template {
    /// Render the template into the virtual dom.
    fn template(&self, path: Path, scope: Path) -> Vec<VirtualNode>;
}

impl<T: std::fmt::Display + NonTreeTemplate + Clone> Template for T {
    fn template(&self, _: Path, scope: Path) -> Vec<VirtualNode> {
        vec![VirtualNode::new(Arc::new(Vec::new()), VirtualKind::Template(format!("{}", self)), scope)]
    }
}

impl Template for Children {
    fn template(&self, path: Path, _: Path) -> Vec<VirtualNode> {
        self.clone().render(path)
    }
}

impl Template for Html {
    fn template(&self, path: Path, scope: Path) -> Vec<VirtualNode> {
        let children = Children::new(self.nodes.clone(), self.refs.clone(), scope);

        children.template(path, Path::new())
    }
}

/// The AttrValue trait represents a value in an attribute.
///
/// The trait provides a blanket implementation for types that implement Any + Display.
pub trait AttrValue: Any + std::fmt::Display {}

impl<T: Any + std::fmt::Display> AttrValue for T {}

/// Represents a map of attributes. Only a wrapper around HashMap.
#[derive(Clone, Default)]
pub struct AttrMap {
    attributes: HashMap<String, Rc<dyn AttrValue>>,
}

impl<T: Iterator<Item = Vec<(String, Rc<dyn AttrValue>)>>> From<T> for AttrMap {
    fn from(from: T) -> AttrMap {
        AttrMap {
            attributes: from.into_iter().flatten().collect(),
        }
    }
}

impl AttrMap {
    fn insert<T: AttrValue>(&mut self, key: String, value: T) {
        self.attributes.insert(key, Rc::new(value));
    }

    /// Get a value from a key. This function returns None if the key doesnt exist, or if the
    /// return type doesnt match the type of the value.
    pub fn get<'a, T: Any + Clone>(&'a self, key: &str) -> Option<T> {
        self.attributes.get(key)
            .and_then(|attr| (attr.as_ref() as &dyn Any).downcast_ref().cloned())
    }

    fn render(&self) -> String {
        self.attributes.iter()
            .map(|(key, value)| format!("{}=\"{}\"", key, value))
            .collect()
    }
}

/// Represents the children of a node.
#[derive(Clone, Default)]
pub struct Children {
    nodes: Rc<Vec<HtmlNode>>,
    refs: Rc<Vec<NodeRef>>,
    scope: Path,
}

impl std::fmt::Display for Children {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_str("children")
    }
}

impl Children {
    fn new(nodes: Rc<Vec<HtmlNode>>, refs: Rc<Vec<NodeRef>>, scope: Path) -> Children {
        Children {
            nodes,
            refs,
            scope,
        }
    }

    fn render(self, path: Path) -> Vec<VirtualNode> {
        self.refs.iter()
            .enumerate()
            .flat_map(|(child_index, node_ref)| self.nodes[node_ref.index].render(self.scope.clone(), path.clone(), self.nodes.clone(), node_ref.refs.clone(), child_index))
            .collect::<Vec<VirtualNode>>()
    }
}

/// Represents a html element.
#[derive(Clone)]
pub struct HtmlElement {
    name: String,
    attributes: AttrMap,
}

impl HtmlElement {
    /// Create a new html element.
    pub fn new(name: String, attributes: Vec<Vec<(String, Rc<dyn AttrValue>)>>) -> HtmlElement {
        HtmlElement {
            name,
            attributes: AttrMap::from(attributes.into_iter()),
        }
    }
}

/// Represents a component, template or element.
///
/// ## Warning
/// This enum is not supposed to be used outside of the framework.
#[derive(Clone)]
pub enum HtmlKind {
    #[allow(missing_docs)]
    Component {
        builder: fn() -> Arc<Mutex<dyn BaseComponent + Send + Sync>>,
        name: String,
    },

    #[allow(missing_docs)]
    Template(Arc<dyn Template>),

    #[allow(missing_docs)]
    Element(HtmlElement),
}

impl HtmlKind {
    /// Create a component of the generic type
    pub fn create_component<T: Component + Send + Sync>(name: String) -> HtmlKind {
        HtmlKind::Component {
            builder: || Arc::new(Mutex::new(T::create())),
            name,
        }
    }

    fn render(
        &self,
        path: Path,
        scope: Path,
        attributes: AttrMap,
        callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>,
        children: Children,
        child_index: usize,
    ) -> Vec<VirtualNode> {
        match self {
            HtmlKind::Component { builder, name } => {
                let path = path.concat(PathNode::new(child_index, name.clone()));

                state::get_or_insert(&path, builder)
                    .lock()
                    .base_view(attributes)
                    .render(path)
            },
            HtmlKind::Template(templates) => {
                let path = path.concat(PathNode::new(child_index, String::from("template")));

                templates.template(path.clone(), scope.clone())
            },
            HtmlKind::Element(element) => {
                let path = path.concat(PathNode::new(child_index, String::from("element")));

                vec![VirtualNode::new(
                    callbacks,
                    VirtualKind::Element(VirtualElement::new(element.name.clone(), element.attributes.render(), Arc::new(children.render(path.clone())))),
                    scope,
                )]
            },
        }
    }
}

/// Represents a node returned from a [`view`](Component::view).
#[derive(Clone)]
pub struct HtmlNode {
    kind: HtmlKind,
    callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>,
    attributes: AttrMap,
}

impl HtmlNode {
    /// Create a new [`HtmlNode`].
    pub fn new(
        kind: HtmlKind,
        callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>,
        attributes: AttrMap
    ) -> HtmlNode {
        HtmlNode {
            kind,
            callbacks,
            attributes,
        }
    }

    fn render(&self, scope: Path, path: Path, nodes: Rc<Vec<HtmlNode>>, refs: Rc<Vec<NodeRef>>, child_index: usize) -> Vec<VirtualNode> {
        let mut attributes = self.attributes.clone();

        attributes.insert(String::from("children"), Children::new(nodes.clone(), refs.clone(), scope.clone()));

        self.kind.render(path, scope.clone(), attributes, self.callbacks.clone(), Children::new(nodes, refs, scope), child_index)
    }
}

/// Reference to a [`HtmlNode`] and its children.
#[derive(Clone, Default)]
pub struct NodeRef {
    index: usize,
    refs: Rc<Vec<NodeRef>>,
}

impl NodeRef {
    /// Create a new [`NodeRef`].
    pub fn new(index: usize, refs: Rc<Vec<NodeRef>>) -> NodeRef {
        NodeRef {
            index,
            refs,
        }
    }
}

/// Html is returned from [`view`](Component::view). The Html struct stores
/// all the nodes and the references to the nodes that describe how they are layed out.
#[derive(Clone, Default)]
pub struct Html {
    nodes: Rc<Vec<HtmlNode>>,
    refs: Rc<Vec<NodeRef>>,
}

impl Html {
    /// Create a new Html tree
    pub fn new(nodes: Rc<Vec<HtmlNode>>, refs: Rc<Vec<NodeRef>>) -> Html {
        Html {
            nodes: nodes,
            refs,
        }
    }

    pub(crate) fn render(self, path: Path) -> Vec<VirtualNode> {
        self.refs.iter()
            .enumerate()
            .flat_map(|(child_index, node_ref)| self.nodes[node_ref.index].render(path.clone(), path.clone(), self.nodes.clone(), node_ref.refs.clone(), child_index))
            .collect::<Vec<VirtualNode>>()
    }
}


