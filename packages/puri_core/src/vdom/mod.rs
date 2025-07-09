use crate::component::state::Identity;


#[derive(Debug)]
pub enum Inner {
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
    inner: Inner,
}

impl Node {
    pub fn new(identity: Identity, inner: Inner) -> Node {
        Node {
            identity,
            inner,
        }
    }
}


