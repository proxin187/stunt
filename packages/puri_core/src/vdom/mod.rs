use crate::component::state::Identity;

use std::sync::{LazyLock, Arc};

use spin::Mutex;

static DIFF: LazyLock<Arc<Mutex<Node>>> = LazyLock::new(|| Arc::new(Mutex::new(Node::default())));


#[derive(Debug)]
pub enum Kind {
    Component(Box<Node>),
    Template(String),
    Props(Vec<Node>),
    Element(VirtualElement),
}

#[derive(Debug)]
pub struct VirtualElement {
    name: String,
    attributes: String,
    props: Vec<Node>,
}

impl VirtualElement {
    pub fn new(name: String, attributes: String, props: Vec<Node>) -> VirtualElement {
        VirtualElement {
            name,
            attributes,
            props,
        }
    }
}

#[derive(Debug)]
pub struct Node {
    identity: Identity,
    kind: Kind,
}

impl Default for Node {
    fn default() -> Node {
        Node {
            identity: Identity::new(0),
            kind: Kind::Template(String::new()),
        }
    }
}

impl Node {
    pub fn new(identity: Identity, kind: Kind) -> Node {
        Node {
            identity,
            kind,
        }
    }
}

pub fn reconciliation() {
    let diff = DIFF.lock();
}


