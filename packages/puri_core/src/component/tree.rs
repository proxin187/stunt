use crate::component::{Component, Context};
use crate::component::state::{self, Identity};

use crate::vdom::{Node, VirtualElement, Kind};

use std::sync::Arc;
use std::any::Any;
use std::rc::Rc;

use spin::Mutex;


// TODO: element will only be used internally and does not need macro support
// props will be available to the user and will need macro support
//
// TODO: template should either be a string or a tree or a vec of trees

pub enum ComponentRef {
    Component(fn() -> Arc<Mutex<dyn Component + Send + Sync>>),
    Template(String),
    Props(Props),
    Element(Element),
}

impl ComponentRef {
    pub fn render(&self, identity: &Identity, context: Context, callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>) -> Node {
        match self {
            ComponentRef::Component(component) => {
                let node = state::get_or_insert(&identity, *component)
                    .lock()
                    .view(context)
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
    attributes: Attributes,
    props: Props,
}

impl Element {
    pub fn new(name: String, attributes: Attributes, props: Props) -> Element {
        Element {
            name,
            attributes,
            props,
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

#[derive(Clone)]
pub struct Attributes {
    attributes: Rc<Vec<(String, String)>>,
}

impl Attributes {
    pub fn new(attributes: Vec<(String, String)>) -> Attributes {
        Attributes {
            attributes: Rc::new(attributes),
        }
    }

    pub fn render(&self) -> String {
        self.attributes.iter()
            .map(|(key, value)| format!("{}=\"{}\"", key, value))
            .collect::<String>()
    }
}

pub struct Tree {
    pub(crate) identity: Identity,
    pub(crate) component: ComponentRef,
    pub(crate) callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>,
    pub(crate) attributes: Attributes,
    pub(crate) props: Props,
}

impl Tree {
    pub fn new(
        identity: Identity,
        component: ComponentRef,
        callbacks: Vec<(String, Arc<dyn Any + Send + Sync>)>,
        attributes: Vec<(String, String)>,
        props: Vec<Tree>,
    ) -> Tree {
        Tree {
            identity,
            component,
            callbacks: Arc::new(callbacks),
            attributes: Attributes::new(attributes),
            props: Props::new(props),
        }
    }

    pub fn render(&self) -> Node {
        let context = Context::new(self.props.clone(), self.attributes.clone(), self.identity.clone());

        self.component.render(&self.identity, context, self.callbacks.clone())
    }
}


