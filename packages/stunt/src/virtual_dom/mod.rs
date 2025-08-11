use crate::component::state::Path;
use crate::component::state;
use crate::render;

use std::sync::{LazyLock, Arc};
use std::any::Any;

use wasm_bindgen::prelude::*;
use spin::Mutex;

static PREV: LazyLock<Arc<Mutex<Node>>> = LazyLock::new(|| Arc::new(Mutex::new(Node::default())));


#[derive(Debug, PartialEq)]
pub enum Kind {
    Template(String),
    Element(VirtualElement),
}

impl Kind {
    pub fn render(&self) -> String {
        match self {
            Kind::Template(_) => String::from("<span></span>"),
            Kind::Element(element) => element.render(),
        }
    }

    pub fn children(&self) -> Arc<Vec<Node>> {
        match self {
            Kind::Template(_) => Arc::new(Vec::new()),
            Kind::Element(element) => element.children.clone(),
        }
    }
}

#[derive(Debug)]
pub struct VirtualElement {
    name: String,
    attributes: String,
    children: Arc<Vec<Node>>,
}

impl PartialEq for VirtualElement {
    fn eq(&self, other: &VirtualElement) -> bool {
        self.name == other.name && self.attributes == other.attributes
    }
}

impl VirtualElement {
    pub fn new(name: String, attributes: String, children: Arc<Vec<Node>>) -> VirtualElement {
        VirtualElement {
            name,
            attributes,
            children,
        }
    }

    pub fn render(&self) -> String {
        let children = self.children.iter()
            .map(|child| child.kind.render())
            .collect::<String>();

        format!("<{} {}>{}</{}>", self.name, self.attributes, children, self.name)
    }
}

#[derive(Debug)]
pub struct Node {
    callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>,
    kind: Kind,
    path: Path,
    scope: Path,
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.kind == other.kind
    }
}

impl Default for Node {
    fn default() -> Node {
        Node {
            callbacks: Arc::new(Vec::new()),
            kind: Kind::Template(String::new()),
            path: Path::new(),
            scope: Path::new(),
        }
    }
}

impl Node {
    pub(crate) fn new(callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>, kind: Kind, path: Path, scope: Path) -> Node {
        Node {
            callbacks,
            kind,
            path,
            scope,
        }
    }

    fn attach_listener(&self, document: &web_sys::Document, event: &str, cb: &Arc<dyn Any + Send + Sync>) {
        if let Ok(element) = self.path.get_element_by_path(document) {
            let scope = self.scope.clone();
            let cb = cb.clone();

            let closure = Closure::<dyn Fn()>::new(move || {
                fn hook_callback(scope: &Path, cb: &Arc<dyn Any + Send + Sync>) {
                    web_sys::console::log_1(&format!("do we fail here? scope: {:?}", scope).into());

                    let component = state::get(&scope);

                    web_sys::console::log_1(&format!("do we get here?").into());

                    // TODO: the issue is inside base_callback
                    component.lock().base_callback(cb);

                    web_sys::console::log_1(&format!("we return here").into());
                }

                hook_callback(&scope, &cb);

                // TODO: the issue is not inside the renderer
                // render::render();
            });

            if let Err(_) = element.add_event_listener_with_callback(&event, closure.as_ref().unchecked_ref()) {
                web_sys::console::log_1(&format!("failed to set callback: {}", self.path).into());
            }

            closure.forget();
        }
    }

    fn passover(&self, document: &web_sys::Document) {
        match &self.kind {
            Kind::Template(template) => if let Ok(element) = self.path.get_element_by_path(document) {
                let node = document.create_text_node(&template);

                if let Err(_) = element.append_child(&node) {
                    web_sys::console::log_1(&format!("failed to set template: {}", self.path).into());
                }
            },
            _ => {},
        }

        for prop in self.kind.children().iter() {
            for (event, cb) in prop.callbacks.iter() {
                prop.attach_listener(document, event, cb);
            }

            prop.passover(document);
        }
    }

    pub fn reconcile(&self, other: &Node, document: &web_sys::Document) -> Result<(), JsValue> {
        if self.kind.children() != other.kind.children() {
            let props = self.kind.children()
                .iter()
                .map(|prop| prop.kind.render())
                .collect::<String>();

            let element = self.path.get_element_by_path(document)?;

            element.set_inner_html(&props);

            self.passover(document);
        } else {
            for (a, b) in self.kind.children().iter().zip(other.kind.children().iter()) {
                a.reconcile(&b, &document)?;
            }
        }

        Ok(())
    }
}

pub fn reconcile(node: Node) {
    let vdom = Node::new(Arc::new(Vec::new()), Kind::Element(VirtualElement::new(String::from("root"), String::new(), Arc::new(vec![node]))), Path::new(), Path::new());

    let mut prev = PREV.lock();

    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("should have a document on window");

    match vdom.reconcile(&*prev, &document) {
        Ok(()) => *prev = vdom,
        Err(err) => {
            web_sys::console::log_1(&format!("failed to reconcile: {:?}", err).into());
        },
    }
}


