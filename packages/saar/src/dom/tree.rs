use crate::html::{Html, Attribute, ComponentRef};

use crate::dom::component::{Component, Context};
use crate::dom::state::{self, Identity};

use std::sync::Arc;
use std::any::Any;


/*
pub enum Inner {
    Component(Identity),
    Block(Box<dyn Fn() -> String>),
}

impl Inner {
    pub fn new(identity: Identity, component: ComponentRef) -> Inner {
        match component {
            ComponentRef::Component(component) => {
                state::insert_if_none(identity.clone(), component);

                Inner::Component(identity)
            },
            ComponentRef::Block(f) => Inner::Block(f),
        }
    }

    pub fn render(&self, props: Props, attributes: Attributes) -> String {
        match self {
            Inner::Component(component) => state::get(*component).render(props, attributes),
            Inner::Block(f) => f(),
        }
    }
}

#[derive(Clone)]
pub struct Attributes {
    inner: Arc<Vec<Attribute>>,
}

impl Attributes {
    pub fn new(inner: Vec<Attribute>) -> Attributes {
        Attributes {
            inner: Arc::new(inner),
        }
    }

    pub fn render(&self) -> String {
        self.inner.iter()
            .map(|attribute| attribute.render())
            .collect::<String>()
    }
}

#[derive(Clone)]
pub struct Props {
    inner: Arc<Vec<Node>>,
}

impl Props {
    pub fn new(inner: Vec<Node>) -> Props {
        Props {
            inner: Arc::new(inner),
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
}

impl Node {
    pub fn new(view: Html, identity: Identity) -> Node {
        let props = view.props.into_iter()
            .map(|html| Node::new(html, identity))
            .collect::<Vec<Node>>();

        // TODO: here we should hook up the callbacks

        Node {
            inner: Inner::new(view.component),
            attributes: Attributes::new(view.attributes),
            props: Props::new(props),
        }
    }

    pub fn render(&self) -> String {
        self.inner.render(self.props.clone(), self.attributes.clone())
    }
}

// TODO: the tree structure will now be rebuilt every render
//
// TODO: i dont think we need to build a tree anymore

pub struct Tree {
    node: Node,
    identity: Identity,
}

impl Tree {
    pub fn new(view: Html, identity: Identity) -> Tree {
        Tree {
            node: Node::new(view, identity),
            identity,
        }
    }

    pub fn render(&self, props: Props, attributes: Attributes) -> String {
        let state = state::get(self.identity);

        self.node.render(Context::new(state.component, props, attributes))
    }
}
*/


pub enum ComponentRef {
    Component(Box<dyn Fn() -> Arc<dyn Component + Send + Sync>>),
    Block(Box<dyn Fn() -> String>),
}

impl ComponentRef {
    pub fn render(self, identity: Identity, context: Context) -> String {
        match self {
            ComponentRef::Component(component) => {
                state::get_or_insert(identity, component)
                    .view(context)
                    .render()
            },
            ComponentRef::Block(f) => f(),
        }
    }
}

pub struct Attribute {
    key: String,
    value: fn() -> String,
}

impl Attribute {
    pub fn new(key: String, value: fn() -> String) -> Attribute {
        Attribute {
            key,
            value,
        }
    }

    pub fn render(&self) -> String {
        format!("{}=\"{}\"", self.key, (self.value)())
    }
}

pub struct Node {
    pub(crate) identity: Identity,
    pub(crate) component: ComponentRef,
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) callback: Vec<(String, fn() -> Box<dyn Any>)>,
    pub(crate) props: Vec<Html>,
}

impl Node {
    pub fn new(
        identity: Identity,
        component: ComponentRef,
        attributes: Vec<Attribute>,
        callback: Vec<(String, fn() -> Box<dyn Any>)>,
        props: Vec<Html>,
    ) -> Node {
        Node {
            identity,
            component,
            attributes,
            callback,
            props,
        }
    }

    pub fn render(self) -> String {
        let context = Context::new(self.props, self.attributes, self.identity.clone());

        self.component.render(self.identity, context)
    }
}


