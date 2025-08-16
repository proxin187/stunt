use crate::component::state::Path;
use crate::component::state;
use crate::render;

use std::sync::{LazyLock, Arc};
use std::any::Any;

use wasm_bindgen::prelude::*;
use spin::Mutex;

static PREV: LazyLock<Arc<Mutex<VirtualNode>>> = LazyLock::new(|| Arc::new(Mutex::new(VirtualNode::default())));


#[derive(Debug, Clone, PartialEq)]
pub enum VirtualKind {
    Template(String),
    Element(VirtualElement),
}

impl VirtualKind {
    pub fn render(&self) -> String {
        match self {
            VirtualKind::Template(_) => String::from("<span></span>"),
            VirtualKind::Element(element) => element.render(),
        }
    }

    pub fn children(&self) -> Arc<Vec<VirtualNode>> {
        match self {
            VirtualKind::Template(_) => Arc::new(Vec::new()),
            VirtualKind::Element(element) => element.children.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VirtualElement {
    name: String,
    attributes: String,
    children: Arc<Vec<VirtualNode>>,
}

impl PartialEq for VirtualElement {
    fn eq(&self, other: &VirtualElement) -> bool {
        self.name == other.name && self.attributes == other.attributes
    }
}

impl VirtualElement {
    pub fn new(name: String, attributes: String, children: Arc<Vec<VirtualNode>>) -> VirtualElement {
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

#[derive(Debug, Clone)]
pub struct VirtualNode {
    callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>,
    kind: VirtualKind,
    path: Path,
    scope: Path,
}

impl PartialEq for VirtualNode {
    fn eq(&self, other: &VirtualNode) -> bool {
        self.kind == other.kind
    }
}

impl Default for VirtualNode {
    fn default() -> VirtualNode {
        VirtualNode {
            callbacks: Arc::new(Vec::new()),
            kind: VirtualKind::Template(String::new()),
            path: Path::new(),
            scope: Path::new(),
        }
    }
}

impl VirtualNode {
    pub(crate) fn new(callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>, kind: VirtualKind, path: Path, scope: Path) -> VirtualNode {
        VirtualNode {
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
                    let component = state::get(&scope);

                    component.lock().base_callback(cb);
                }

                hook_callback(&scope, &cb);

                render::render();
            });

            if let Err(_) = element.add_event_listener_with_callback(&event, closure.as_ref().unchecked_ref()) {
                web_sys::console::log_1(&format!("failed to set callback: {}", self.path).into());
            }

            closure.forget();
        }
    }

    fn passover(&self, document: &web_sys::Document) {
        match &self.kind {
            VirtualKind::Template(template) => if let Ok(element) = self.path.get_element_by_path(document) {
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

    pub fn reconcile(&self, other: &VirtualNode, document: &web_sys::Document) -> Result<(), JsValue> {
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

pub fn reconcile(node: VirtualNode) {
    let vdom = VirtualNode::new(Arc::new(Vec::new()), VirtualKind::Element(VirtualElement::new(String::from("root"), String::new(), Arc::new(vec![node]))), Path::new(), Path::new());

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


