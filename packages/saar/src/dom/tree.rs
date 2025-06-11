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

    // the outer context is the context of the tree root component
    // the inner context is the context that is inside the component, eg. its props and etc

    pub fn render(&self, inner: Context, outer: Context) -> String {
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

                f(outer)
            },
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

    pub fn render(&self, context: Context) -> String {
        self.inner.iter()
            .map(|node| node.render(context.clone()))
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
        //
        // TODO: the context for rendering the props and the context for the blocks are completely
        // different

        Node {
            inner: Inner::new(view.component),
            attributes: Attributes::new(view.attributes),
            props: Props::new(props),
            identity,
        }
    }

    pub fn render(&self, context: Context) -> String {
        self.inner.render(context)
    }
}

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

    pub fn render(&self) -> String {
        let state = state::get(self.identity);

        web_sys::console::log_1(&format!("name: {:?}", state.component.name()).into());

        self.node.render(Context::new(state.component, self.node.props.clone(), self.node.attributes.clone()))
    }
}

#[derive(Clone)]
pub struct Context {
    component: Arc<dyn Component>,
    pub props: Props,
    pub attributes: Attributes,
}

impl Context {
    pub fn new(component: Arc<dyn Component>, props: Props, attributes: Attributes) -> Context {
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


