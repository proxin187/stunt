use crate::html::{Html, Attribute, ComponentRef};

use crate::dom::component::Component;
use crate::dom::state;

use std::sync::Arc;
use std::any::Any;


pub enum Inner {
    Component(usize),
    Block(fn(Context) -> String),
}

impl Inner {
    pub fn new(component: ComponentRef) -> Inner {
        match component {
            ComponentRef::Component(component) => {
                let view = component.view();

                Inner::Component(state::push(component, view))
            },
            ComponentRef::Block(f) => Inner::Block(f),
        }
    }

    pub fn render(&self, context: Context) -> String {
        match self {
            Inner::Component(component) => state::get(*component).render(),
            Inner::Block(f) => f(context),
        }
    }
}

pub struct Attributes {
    inner: Vec<Attribute>,
}

impl Attributes {
    pub fn new(inner: Vec<Attribute>) -> Attributes {
        Attributes {
            inner,
        }
    }

    pub fn render(&self) -> String {
        self.inner.iter()
            .map(|attribute| attribute.render())
            .collect::<String>()
    }
}

pub struct Props {
    inner: Vec<Node>,
}

impl Props {
    pub fn new(inner: Vec<Node>) -> Props {
        Props {
            inner,
        }
    }

    pub fn render(&self) -> String {
        self.inner.iter()
            .map(|node| node.render())
            .collect::<String>()
    }
}

pub struct Node {
    inner: Inner,
    attributes: Attributes,
    props: Props,
    scope: usize,
}

impl Node {
    pub fn new(view: Html, scope: usize) -> Node {
        let props = view.props.into_iter()
            .map(|html| Node::new(html, scope))
            .collect::<Vec<Node>>();

        // TODO: here we should hook up the callbacks

        Node {
            inner: Inner::new(view.component),
            attributes: Attributes::new(view.attributes),
            props: Props::new(props),
            scope,
        }
    }

    pub fn render(&self) -> String {
        web_sys::console::log_1(&format!("scope: {:?}", self.scope).into());

        // TODO: the issue is here, i would guess this is because we are trying to access it nested
        let component = state::get(self.scope).component.clone();

        web_sys::console::log_1(&format!("twice scope: {:?}", self.scope).into());

        self.inner.render(Context::new(component, &self.props, &self.attributes))
    }
}

pub struct Tree {
    node: Node,
}

impl Tree {
    pub fn new(view: Html, scope: usize) -> Tree {
        Tree {
            node: Node::new(view, scope),
        }
    }

    pub fn render(&self) -> String {
        self.node.render()
    }
}

pub struct Context<'a> {
    component: Arc<dyn Component>,
    pub props: &'a Props,
    pub attributes: &'a Attributes,
}

impl<'a> Context<'a> {
    pub fn new(component: Arc<dyn Component>, props: &'a Props, attributes: &'a Attributes) -> Context<'a> {
        Context {
            component,
            props,
            attributes,
        }
    }

    pub fn extract<T: Any>(&self, extract: T) -> String {
        self.component.extract(Box::new(extract))
    }
}


