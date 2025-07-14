use crate::component::state::Identity;
use crate::component::callback;

use std::sync::{LazyLock, Arc};
use std::any::Any;

use wasm_bindgen::JsValue;
use spin::Mutex;

static PREV: LazyLock<Arc<Mutex<Node>>> = LazyLock::new(|| Arc::new(Mutex::new(Node::default())));


#[derive(Debug, PartialEq)]
pub enum Kind {
    Template(String),
    Props(Arc<Vec<Node>>),
    Element(VirtualElement),
}

impl Kind {
    pub fn render(&self, identity: &Identity) -> String {
        match self {
            Kind::Template(template) => template.clone(),
            Kind::Props(props) => props.iter().map(|prop| prop.kind.render(&prop.identity)).collect(),
            Kind::Element(element) => element.render(identity),
        }
    }

    pub fn props(&self) -> Arc<Vec<Node>> {
        match self {
            Kind::Template(_) => Arc::new(Vec::new()),
            Kind::Props(props) => props.clone(),
            Kind::Element(element) => element.props.clone(),
        }
    }
}

#[derive(Debug)]
pub struct VirtualElement {
    name: String,
    attributes: String,
    props: Arc<Vec<Node>>,
}

impl PartialEq for VirtualElement {
    fn eq(&self, other: &VirtualElement) -> bool {
        self.name == other.name && self.attributes == other.attributes
    }
}

impl VirtualElement {
    pub fn new(name: String, attributes: String, props: Arc<Vec<Node>>) -> VirtualElement {
        VirtualElement {
            name,
            attributes,
            props,
        }
    }

    pub fn render(&self, identity: &Identity) -> String {
        let props = self.props.iter()
            .map(|prop| prop.kind.render(&prop.identity))
            .collect::<String>();

        format!("<{} id=\"{}\" {}>{}</{}>", self.name, identity.render(), self.attributes, props, self.name)
    }
}

#[derive(Debug)]
pub struct Node {
    callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>,
    identity: Identity,
    kind: Kind,
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.identity == other.identity && self.kind == other.kind
    }
}

impl Default for Node {
    fn default() -> Node {
        Node {
            callbacks: Arc::new(Vec::new()),
            identity: Identity::new(0),
            kind: Kind::Template(String::new()),
        }
    }
}

impl Node {
    pub fn new(identity: Identity, kind: Kind, callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>) -> Node {
        Node {
            callbacks,
            identity,
            kind,
        }
    }

    pub fn reattach_callback(&self) {
        for prop in self.kind.props().iter() {
            for (event, cb) in prop.callbacks.iter() {
                callback::push(prop.identity.clone(), event.clone(), cb.clone());
            }

            prop.reattach_callback();
        }
    }

    pub fn reconcile(&self, other: &Node, document: &web_sys::Document, body: Option<web_sys::HtmlElement>) -> Result<(), JsValue> {
        if self.kind.props() != other.kind.props() {
            let props = self.kind.props()
                .iter()
                .map(|prop| prop.kind.render(&prop.identity))
                .collect::<String>();

            match body {
                Some(body) => {
                    body.set_inner_html(&props);
                },
                None => {
                    let element = document.get_element_by_id(&self.identity.render()).ok_or(JsValue::from_str("failed to get element"))?;

                    element.set_inner_html(&props);
                },
            }

            self.reattach_callback();
        } else {
            for (a, b) in self.kind.props().iter().zip(other.kind.props().iter()) {
                a.reconcile(&b, &document, None)?;
            }
        }

        Ok(())
    }
}

#[inline]
pub fn reconcile(vdom: Node) {
    let mut prev = PREV.lock();

    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    match vdom.reconcile(&*prev, &document, Some(body)) {
        Ok(()) => *prev = vdom,
        Err(err) => {
            web_sys::console::log_1(&format!("failed to reconcile: {:?}", err).into());
        },
    }
}


