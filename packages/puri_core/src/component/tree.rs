use crate::component::state::{self, Identity};
use crate::component::{BaseComponent, Context};

use crate::vdom::{Node, VirtualElement, Kind};

use std::collections::HashMap;
use std::sync::Arc;
use std::any::Any;
use std::rc::Rc;

use spin::Mutex;


// TODO: template should either be a string or a tree or a vec of trees

pub enum ComponentRef {
    Component(fn() -> Arc<Mutex<dyn BaseComponent + Send + Sync>>),
    Template(String),
    Props(Props),
    Element(Element),
}

impl ComponentRef {
    pub fn render(&self, identity: &Identity, attributes: AttrMap, callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>) -> Node {
        match self {
            ComponentRef::Component(component) => {
                let node = state::get_or_insert(&identity, *component)
                    .lock()
                    .base_view(Context::new(identity.clone()), AttrMap::from(attributes))
                    .render();

                Node::new(
                    identity.clone(),
                    Kind::Element(VirtualElement::new(String::from("span"), String::new(), Arc::new(vec![node]))),
                    callbacks,
                )
            },
            ComponentRef::Template(template) => Node::new(identity.clone(), Kind::Template(template.clone()), callbacks),
            ComponentRef::Props(props) => Node::new(identity.clone(), Kind::Props(Arc::new(props.render())), callbacks),
            ComponentRef::Element(element) => {
                Node::new(
                    identity.clone(),
                    Kind::Element(VirtualElement::new(element.name.clone(), element.attributes.render(), Arc::new(element.props.render()))),
                    callbacks,
                )
            },
        }
    }
}

pub struct Element {
    name: String,
    attributes: AttrMap,
    props: Props,
}

impl Element {
    pub fn new(name: String, attributes: Vec<(String, Arc<dyn AttrValue>)>, props: Vec<Tree>) -> Element {
        Element {
            name,
            attributes: AttrMap::from(attributes),
            props: Props::new(props),
        }
    }
}

#[derive(Clone)]
pub struct Props {
    props: Rc<Vec<Tree>>,
}

impl Props {
    pub fn new(props: Vec<Tree>) -> Props {
        Props {
            props: Rc::new(props),
        }
    }

    fn render(&self) -> Vec<Node> {
        self.props.iter()
            .map(|prop| prop.render())
            .collect::<Vec<Node>>()
    }
}

pub trait AttrValue: Any + std::fmt::Display {}

impl<T: Any + std::fmt::Display> AttrValue for T {}

#[derive(Clone)]
pub struct AttrMap {
    attributes: Arc<HashMap<String, Arc<dyn AttrValue>>>,
}

impl From<Vec<(String, Arc<dyn AttrValue>)>> for AttrMap {
    fn from(from: Vec<(String, Arc<dyn AttrValue>)>) -> AttrMap {
        AttrMap {
            attributes: Arc::new(from.into_iter().collect()),
        }
    }
}

impl AttrMap {
    pub fn get<'a, T: Any>(&'a self, key: String) -> Option<&'a T> {
        self.attributes.get(&key)
            .and_then(|attr| (attr as &dyn Any).downcast_ref())
    }

    pub fn render(&self) -> String {
        self.attributes.iter()
            .map(|(key, value)| format!("{}=\"{}\"", key, value))
            .collect()
    }
}

pub struct Tree {
    pub(crate) identity: Identity,
    pub(crate) component: ComponentRef,
    pub(crate) callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>,
    pub(crate) attributes: AttrMap,
    pub(crate) props: Props,
}

impl Tree {
    pub fn new(
        identity: Identity,
        component: ComponentRef,
        callbacks: Vec<(String, Arc<dyn Any + Send + Sync>)>,
        attributes: Vec<(String, Arc<dyn AttrValue>)>,
        props: Vec<Tree>,
    ) -> Tree {
        Tree {
            identity,
            component,
            callbacks: Arc::new(callbacks),
            attributes: AttrMap::from(attributes),
            props: Props::new(props),
        }
    }

    pub fn render(&self) -> Node {
        self.component.render(&self.identity, self.attributes.clone(), self.callbacks.clone())
    }
}


