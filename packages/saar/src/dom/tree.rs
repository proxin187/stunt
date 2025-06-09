use crate::html::{Html, Attribute, ComponentRef};

use crate::dom::state::{self, Identity};
use crate::dom::component::Component;

use std::sync::Arc;
use std::any::Any;


pub enum Inner {
    Component(Identity),
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
            Inner::Component(component) => {
                web_sys::console::log_1(&format!("component").into());

                // TODO: the issue is here, this is because we dont pass in the context
                // the component needs to have the state from the root of the tree
                //
                // TODO: i think we can solve it by just passing in the context to the render
                // function
                //
                // TODO: here we will have to replace with a new context

                // this will get a new tree, meaning that we should not pass in our
                state::get(*component).render()
            },
            Inner::Block(f) => {
                web_sys::console::log_1(&format!("block").into());

                f(context)
            },
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
    identity: Identity,
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
            identity,
        }
    }

    pub fn render(&self) -> String {
        web_sys::console::log_1(&format!("identity: {:?}", self.identity).into());

        // TODO: figure out what we should do here :(, wtf am i doing, im such a retard, this is
        // literally so retarded, holy shit, im so fucking disapointed, i should kms.

        let state = state::get(self.identity);

        web_sys::console::log_1(&format!("name: {:?}", state.component.name()).into());

        web_sys::console::log_1(&format!("name: {:?}", self.props.render()).into());

        // TODO: we will have to visually represent this entire thing to properly understand it, at
        // this point its so cluster fucked that there is no way in hell i will ever understand it
        // just inside my head, ill have to have something visual

        let raw = self.inner.render(Context::new(state.component, &self.props, &self.attributes));

        web_sys::console::log_1(&format!("node render: {:?}", raw).into());

        raw
    }
}

pub struct Tree {
    node: Node,
}

impl Tree {
    pub fn new(view: Html, identity: Identity) -> Tree {
        Tree {
            node: Node::new(view, identity),
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


