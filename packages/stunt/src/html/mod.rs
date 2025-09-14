//! The basic building blocks for building Html. This module is mostly used by macros.

pub mod node_id;
pub mod path;

use crate::virtual_dom::{VirtualNode, VirtualKind, VirtualElement};
use crate::render::Renderer;

use crate::html::path::{Path, PathNode};
use crate::{Component, BaseComponent, PreBuild};

use std::cell::RefCell;
use std::sync::Arc;
use std::any::Any;
use std::rc::Rc;

use spin::Mutex;

macro_rules! impl_t {
    ($($t:ty),+) => {
        trait NonHtmlTemplate {}

        $(impl NonHtmlTemplate for $t {})*
    }
}

impl_t!(&str, String, usize, u64, u32, u16, u8, isize, i128, i64, i32, i16, i8, f64, f32);

/// The template trait allows a type to be used as a template.
///
/// ## Warning
/// For the time being this trait is not supposed to be implemented outside the framework.
pub trait Template {
    /// Render the template into the virtual dom.
    fn template(&self, renderer: Renderer, path: Path, scope: Path) -> Vec<VirtualNode>;
}

impl<T: std::fmt::Display + NonHtmlTemplate + Clone> Template for T {
    fn template(&self, _: Renderer, _: Path, scope: Path) -> Vec<VirtualNode> {
        vec![VirtualNode::new(Arc::new(Vec::new()), VirtualKind::Template(format!("{}", self)), scope)]
    }
}

impl Template for Children {
    fn template(&self, renderer: Renderer, path: Path, _: Path) -> Vec<VirtualNode> {
        self.clone().render(renderer, path)
    }
}

impl Template for Html {
    fn template(&self, renderer: Renderer, path: Path, scope: Path) -> Vec<VirtualNode> {
        let children = Children::new(self.nodes.clone(), self.refs.clone(), scope);

        children.template(renderer, path, Path::new())
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

    fn render(self, renderer: Renderer, path: Path) -> Vec<VirtualNode> {
        self.refs.iter()
            .enumerate()
            .flat_map(|(child_index, node_ref)| self.nodes[node_ref.index].render(renderer.clone(), self.scope.clone(), path.clone(), self.nodes.clone(), node_ref.refs.clone(), child_index))
            .collect::<Vec<VirtualNode>>()
    }
}

/// Represents a html element.
#[derive(Clone)]
pub struct HtmlElement {
    name: String,
    attributes: Vec<(String, Rc<dyn std::fmt::Display>)>,
}

impl HtmlElement {
    /// Create a new html element.
    pub fn new(name: String, attributes: Vec<(String, Rc<dyn std::fmt::Display>)>) -> HtmlElement {
        HtmlElement {
            name,
            attributes,
        }
    }

    fn attributes(&self) -> String {
        self.attributes.iter()
            .map(|(key, value)| format!("{}=\"{}\"", key, value))
            .collect()
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
        renderer: Renderer,
        path: Path,
        scope: Path,
        properties: Rc<RefCell<dyn PreBuild>>,
        callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>,
        children: Children,
        child_index: usize,
    ) -> Vec<VirtualNode> {
        match self {
            HtmlKind::Component { builder, name } => {
                let path = path.concat(PathNode::new(child_index, name.clone()));

                renderer.get_or_insert(&path, builder)
                    .lock()
                    .base_view(properties.borrow().build())
                    .render(renderer, path)
            },
            HtmlKind::Template(templates) => {
                let path = path.concat(PathNode::new(child_index, String::from("template")));

                templates.template(renderer, path.clone(), scope.clone())
            },
            HtmlKind::Element(element) => {
                let path = path.concat(PathNode::new(child_index, String::from("element")));

                vec![VirtualNode::new(
                    callbacks,
                    VirtualKind::Element(VirtualElement::new(element.name.clone(), element.attributes(), Arc::new(children.render(renderer, path.clone())))),
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
    properties: Rc<RefCell<dyn PreBuild>>,
}

impl HtmlNode {
    /// Create a new [`HtmlNode`].
    pub fn new<T: PreBuild + 'static>(
        kind: HtmlKind,
        callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>,
        properties: T,
    ) -> HtmlNode {
        HtmlNode {
            kind,
            callbacks,
            properties: Rc::new(RefCell::new(properties)),
        }
    }

    fn render(
        &self,
        renderer: Renderer,
        scope: Path,
        path: Path,
        nodes: Rc<Vec<HtmlNode>>,
        refs: Rc<Vec<NodeRef>>,
        child_index: usize
    ) -> Vec<VirtualNode> {
        let children = Children::new(nodes, refs, scope.clone());

        self.properties.borrow_mut().children(children.clone());

        self.kind.render(renderer, path, scope, self.properties.clone(), self.callbacks.clone(), children, child_index)
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

    pub(crate) fn render(self, renderer: Renderer, path: Path) -> Vec<VirtualNode> {
        self.refs.iter()
            .enumerate()
            .flat_map(|(child_index, node_ref)| self.nodes[node_ref.index].render(renderer.clone(), path.clone(), path.clone(), self.nodes.clone(), node_ref.refs.clone(), child_index))
            .collect::<Vec<VirtualNode>>()
    }
}


