use crate::component::state::{self, Identity};
use crate::component::{BaseComponent, Context};

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

pub trait Template {
    fn render(&self) -> Kind;
}

impl<T: std::fmt::Display + NonTreeTemplate + Clone> Template for T {
    fn render(&self) -> Kind {
        Kind::Template(format!("{}", self))
    }
}

impl Template for Vec<Tree> {
    fn render(&self) -> Kind {
        let nodes = self.into_iter()
            .cloned()
            .map(|tree| tree.render())
            .collect::<Vec<Node>>();

        Kind::Element(VirtualElement::new(String::from("span"), String::new(), Arc::new(nodes)))
    }
}

#[derive(Clone)]
pub enum ComponentRef {
    Component(fn() -> Arc<Mutex<dyn BaseComponent + Send + Sync>>),
    Template(Arc<dyn Template>),
    Element(Element),
}

impl ComponentRef {
    pub fn render(self, identity: Identity, attributes: AttrMap, callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>) -> Node {
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

#[derive(Clone)]
pub struct Element {
    name: String,
    attributes: AttrMap,
    children: Children,
}

impl Element {
    pub fn new(name: String, attributes: Vec<(String, Rc<dyn AttrValue>)>, children: Vec<Tree>) -> Element {
        Element {
            name,
            attributes: AttrMap::from(attributes),
            children: Children::new(children),
        }
    }
}

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
    pub fn new(children: Vec<Tree>) -> Children {
        Children {
            children: children,
        }
    }

    pub fn children(self) -> Vec<Tree> { self.children }

    fn render(self) -> Vec<Node> {
        self.children.into_iter()
            .map(|child| child.render())
            .collect::<Vec<Node>>()
    }
}

pub trait AttrValue: Any + std::fmt::Display {}

impl<T: Any + std::fmt::Display> AttrValue for T {}

#[derive(Clone)]
pub struct AttrMap {
    attributes: HashMap<String, Rc<dyn AttrValue>>,
}

impl From<Vec<(String, Rc<dyn AttrValue>)>> for AttrMap {
    fn from(from: Vec<(String, Rc<dyn AttrValue>)>) -> AttrMap {
        AttrMap {
            attributes: from.into_iter().collect(),
        }
    }
}

impl AttrMap {
    pub fn new(attributes: HashMap<String, Rc<dyn AttrValue>>) -> AttrMap {
        AttrMap {
            attributes,
        }
    }

    fn insert<T: AttrValue>(&mut self, key: String, value: T) {
        self.attributes.insert(key, Rc::new(value));
    }

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

#[derive(Clone)]
pub struct Tree {
    pub(crate) identity: Identity,
    pub(crate) component: ComponentRef,
    pub(crate) callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>,
    pub(crate) attributes: AttrMap,
    pub(crate) children: Children,
}

impl Tree {
    pub fn new(
        identity: Identity,
        component: ComponentRef,
        callbacks: Vec<(String, Arc<dyn Any + Send + Sync>)>,
        attributes: Vec<(String, Rc<dyn AttrValue>)>,
        children: Vec<Tree>,
    ) -> Tree {
        Tree {
            identity,
            component,
            callbacks: Arc::new(callbacks),
            attributes: AttrMap::from(attributes),
            children: Children::new(children),
        }
    }

    pub fn render(mut self) -> Node {
        self.attributes.insert(String::from("children"), self.children);

        self.component.render(self.identity, self.attributes, self.callbacks)
    }
}


