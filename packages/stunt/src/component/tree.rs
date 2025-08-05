//! The basic building blocks for constructing html trees. This module is mostly used by macros.

use crate::component::state::{self, Identity};
use crate::component::{Component, BaseComponent, Context};

use crate::vdom::{Node, VirtualElement, Kind};

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
    /// Render the template into the virtual dom
    fn render(&self) -> Kind;
}

impl<T: std::fmt::Display + NonTreeTemplate + Clone> Template for T {
    fn render(&self) -> Kind {
        Kind::Template(format!("{}", self))
    }
}

impl Template for Children {
    fn render(&self) -> Kind {
        let nodes = self.children.clone()
            .into_iter()
            .map(|tree| tree.render())
            .collect::<Vec<Node>>();

        Kind::Element(VirtualElement::new(String::from("span"), String::new(), Arc::new(nodes)))
    }
}

/// Represents a component, template or element.
///
/// ## Warning
/// This enum is not supposed to be used outside of the framework.
#[derive(Clone)]
pub enum ComponentRef {
    #[allow(missing_docs)]
    Component(fn() -> Arc<Mutex<dyn BaseComponent + Send + Sync>>),

    #[allow(missing_docs)]
    Template(Arc<dyn Template>),

    #[allow(missing_docs)]
    Element(Element),
}

impl ComponentRef {
    /// Create a component of the generic type
    pub fn create_component<T: Component + Send + Sync>() -> ComponentRef {
        ComponentRef::Component(|| Arc::new(Mutex::new(T::create())))
    }

    pub(crate) fn render(self, identity: Identity, attributes: AttrMap, callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>) -> Node {
        match self {
            ComponentRef::Component(component) => {
                let node = state::get_or_insert(&identity, component)
                    .lock()
                    .base_view(Context::new(identity.clone()), attributes)
                    .render();

                Node::new(
                    identity,
                    Kind::Element(VirtualElement::new(String::from("span"), String::new(), Arc::new(vec![node]))),
                    callbacks,
                )
            },
            ComponentRef::Template(template) => Node::new(identity, template.render(), callbacks),
            ComponentRef::Element(element) => {
                Node::new(
                    identity,
                    Kind::Element(VirtualElement::new(element.name, element.attributes.render(), Arc::new(element.children.render()))),
                    callbacks,
                )
            },
        }
    }
}

/// Represents a html element.
#[derive(Clone)]
pub struct Element {
    name: String,
    attributes: AttrMap,
    children: Children,
}

impl Element {
    /// Create a new html element.
    pub fn new(name: String, attributes: Vec<Vec<(String, Rc<dyn AttrValue>)>>, children: Vec<Tree>) -> Element {
        Element {
            name,
            attributes: AttrMap::from(attributes.into_iter().flatten()),
            children: Children::new(children),
        }
    }
}

/// Represents the children of a node.
#[derive(Clone, Default)]
pub struct Children {
    children: Vec<Tree>,
}

impl std::fmt::Display for Children {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_str("children")
    }
}

impl Children {
    /// Create a new children instance from a vector of trees
    pub fn new(children: Vec<Tree>) -> Children {
        Children {
            children: children,
        }
    }

    fn render(self) -> Vec<Node> {
        self.children.into_iter()
            .map(|child| child.render())
            .collect::<Vec<Node>>()
    }
}

/// The AttrValue trait represents a value in an attribute.
///
/// The trait provides a blanket implementation for types that implement Any + Display.
pub trait AttrValue: Any + std::fmt::Display {}

impl<T: Any + std::fmt::Display> AttrValue for T {}

/// Represents a map of attributes. Only a wrapper around HashMap.
#[derive(Clone)]
pub struct AttrMap {
    attributes: HashMap<String, Rc<dyn AttrValue>>,
}

impl<T: Iterator<Item = (String, Rc<dyn AttrValue>)>> From<T> for AttrMap {
    fn from(from: T) -> AttrMap {
        AttrMap {
            attributes: from.into_iter().collect(),
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

/// Represents a html Tree.
///
/// ## Warning
/// A html tree should only be built by the [`html`](crate::stunt_macro::html) macro.
#[derive(Clone)]
pub struct Tree {
    pub(crate) identity: Identity,
    pub(crate) component: ComponentRef,
    pub(crate) callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>,
    pub(crate) attributes: AttrMap,
    pub(crate) children: Children,
}

impl Tree {
    /// Create a new tree
    pub fn new(
        identity: Identity,
        component: ComponentRef,
        callbacks: Vec<(String, Arc<dyn Any + Send + Sync>)>,
        attributes: Vec<Vec<(String, Rc<dyn AttrValue>)>>,
        children: Vec<Tree>,
    ) -> Tree {
        Tree {
            identity,
            component,
            callbacks: Arc::new(callbacks),
            attributes: AttrMap::from(attributes.into_iter().flatten()),
            children: Children::new(children),
        }
    }

    pub(crate) fn render(mut self) -> Node {
        self.attributes.insert(String::from("children"), self.children);

        self.component.render(self.identity, self.attributes, self.callbacks)
    }
}


