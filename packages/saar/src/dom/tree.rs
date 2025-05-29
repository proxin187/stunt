use crate::html::{Html, Attribute, ComponentRef};

use crate::dom::component::Component;
use crate::dom::state;

use std::any::Any;


pub enum Inner {
    Component(usize),
    Block(fn(Context) -> String),
}

impl Inner {
    pub fn new(component: ComponentRef) -> Inner {
        match component {
            ComponentRef::Component(component) => Inner::Component(state::with(|state| state.push(component))),
            ComponentRef::Block(f) => Inner::Block(f),
        }
    }

    pub fn render(&self, context: Context) {
        match self {
            Inner::Component(component) => state::with(|state| ),
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

    pub fn render(&self) {
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

    pub fn render(&self) {
    }
}

pub struct Node {
    inner: Inner,
    attributes: Attributes,
    props: Props,
}

impl Node {
    pub fn new(html: Html) -> Node {
        let props = html.props.into_iter()
            .map(|html| Node::new(html))
            .collect::<Vec<Node>>();

        Node {
            inner: Inner::new(html.component),
            attributes: Attributes::new(html.attributes),
            props: Props::new(props),
        }
    }

    pub fn render(&self) -> String {
        // TODO: we need to somehow keep track of which component we are inside of
        // maybe we can have a seperate tree for each component?
        //
        // maybe we need some sort of a scope mechanism

        self.inner.render(self.attributes, self.props)
    }
}

pub struct Context<'a> {
    pub component: &'a Box<dyn Component>,
    pub props: &'a Props,
    pub attributes: &'a Attributes,
}

impl<'a> Context<'a> {
    pub fn new(component: &'a Box<dyn Component>, props: &'a Props, attributes: &'a Attributes) -> Context<'a> {
        Context {
            component,
            props,
            attributes,
        }
    }
}

pub struct Tree {
    node: Node,
}

impl Tree {
    pub fn new<T: Component>(component: &T) -> Tree {
        let html = component.view();

        Tree {
            node: Node::new(html),
        }
    }

    pub fn render(&self) -> String {
        self.node.render()
    }
}


