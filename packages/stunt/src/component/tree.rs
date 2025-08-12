//! The basic building blocks for constructing html trees. This module is mostly used by macros.

use crate::component::state::{self, Path, PathNode, PathBuilder};
use crate::component::{Component, BaseComponent};
use crate::component::html::HtmlNode;

use crate::virtual_dom::{Node, VirtualElement, Kind};

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
    fn template(&self, path: PathBuilder, scope: Path) -> Kind;
}

impl<T: std::fmt::Display + NonTreeTemplate + Clone> Template for T {
    fn template(&self, _: PathBuilder, _: Path) -> Kind {
        Kind::Template(format!("{}", self))
    }
}

impl Template for Children {
    fn template(&self, path: PathBuilder, _: Path) -> Kind {
        Kind::Element(VirtualElement::new(String::from("span"), String::new(), Arc::new(self.clone().render(path))))
    }
}

impl Template for Vec<Tree> {
    fn template(&self, path: PathBuilder, scope: Path) -> Kind {
        let children = Children::new(self.clone(), scope);

        children.template(path, Path::new())
    }
}

/// Represents a component, template or element.
///
/// ## Warning
/// This enum is not supposed to be used outside of the framework.
#[derive(Clone)]
pub enum TreeKind {
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

impl TreeKind {
    /// Create a component of the generic type
    pub fn create_component<T: Component + Send + Sync>(name: String) -> TreeKind {
        TreeKind::Component {
            builder: || Arc::new(Mutex::new(T::create())),
            name,
        }
    }

    pub(crate) fn render(
        self,
        path: PathBuilder,
        scope: Path,
        attributes: AttrMap,
        callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>
    ) -> Node {
        match self {
            TreeKind::Component { builder, name } => {
                state::get_or_insert(&path.real, builder, &name)
                    .lock()
                    .base_view(attributes)
                    .render(path.clone(), scope, 0)
            },
            TreeKind::Template(template) => Node::new(callbacks, template.template(path.clone(), scope.clone()), path.virt, scope),
            TreeKind::Element(element) => {
                let children = element.children.into_iter()
                    .enumerate()
                    .map(|(index, child)| child.render(path.clone(), scope.clone(), index))
                    .collect::<Vec<Node>>();

                let kind = Kind::Element(VirtualElement::new(element.name, element.attributes.render(), Arc::new(children)));

                Node::new(callbacks, kind, path.virt, scope)
            },
        }
    }
}

/// Represents a html element.
#[derive(Clone)]
pub struct Element {
    name: String,
    attributes: AttrMap,
    children: Vec<Tree>,
}

impl Element {
    /// Create a new html element.
    pub fn new(name: String, attributes: Vec<Vec<(String, Rc<dyn AttrValue>)>>, children: Vec<Tree>) -> Element {
        Element {
            name,
            attributes: AttrMap::from(attributes.into_iter().flatten()),
            children,
        }
    }
}

/// Represents the children of a node.
#[derive(Clone, Default)]
pub struct Children {
    children: Vec<Tree>,
    scope: Path,
}

impl std::fmt::Display for Children {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_str("children")
    }
}

impl Children {
    fn new(children: Vec<Tree>, scope: Path) -> Children {
        Children {
            children: children,
            scope,
        }
    }

    fn render(self, path: PathBuilder) -> Vec<Node> {
        self.children.into_iter()
            .enumerate()
            .map(|(index, child)| child.render(path.clone(), self.scope.clone(), index))
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
    pub(crate) node: HtmlNode,
    pub(crate) children: Vec<Tree>,
    pub(crate) scope: Path,
}

impl Tree {
    /// Create a new tree
    pub fn new(
        node: HtmlNode,
        children: Vec<Tree>,
        scope: Path,
    ) -> Tree {
        Tree {
            node,
            children,
            scope,
        }
    }

    pub(crate) fn render(mut self, path: PathBuilder, scope: Path, index: usize) -> Node {
        web_sys::console::log_1(&format!("path: {:?}", path).into());
        web_sys::console::log_1(&format!("scope: {:?}", scope).into());

        // TODO: maybe we could define the children in a Vec<Vec<Tree>> before the main Tree
        //
        //
        // we could also have so that the main tree is called Html and only takes in a root element
        // and the children in a list, which it will then build a tree from

        // TODO: this gets the scope of the parent meaning that it doesnt work when you nest
        self.attributes.insert(String::from("children"), Children::new(self.children, scope.clone()));

        let path_node = self.kind.path_node(index);

        // let new_scope = path.real.clone();

        if let TreeKind::Component { .. } = self.kind {
            self.kind.render(PathBuilder::new(path.real.concat(path_node), path.virt), scope, self.attributes, self.callbacks)
        } else {
            self.kind.render(PathBuilder::new(path.real.concat(path_node.clone()), path.virt.concat(path_node)), scope, self.attributes, self.callbacks)
        }
    }
}



