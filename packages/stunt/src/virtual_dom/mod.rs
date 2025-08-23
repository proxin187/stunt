use crate::component::path::{Path, PathNode};
use crate::render::Renderer;

use std::sync::Arc;
use std::any::Any;

use wasm_bindgen::prelude::*;


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
            scope: Path::new(),
        }
    }
}

impl VirtualNode {
    pub(crate) fn new(callbacks: Arc<Vec<(String, Arc<dyn Any + Send + Sync>)>>, kind: VirtualKind, scope: Path) -> VirtualNode {
        VirtualNode {
            callbacks,
            kind,
            scope,
        }
    }

    fn attach_listener(&self, renderer: Renderer, old_element: web_sys::HtmlElement, event: &str, cb: &Arc<dyn Any + Send + Sync>) -> Result<(), JsValue> {
        if let Some(parent) = old_element.parent_node() {
            let new_node = old_element.clone_node_with_deep(true)?;

            parent.replace_child(&new_node, &old_element)?;

            let scope = self.scope.clone();
            let cb = cb.clone();

            let closure = Closure::<dyn Fn()>::new(move || {
                fn hook_callback(renderer: &Renderer, scope: &Path, cb: &Arc<dyn Any + Send + Sync>) {
                    let component = renderer.get(&scope);

                    component.lock().base_callback(cb);
                }

                hook_callback(&renderer, &scope, &cb);

                renderer.render();
            });

            new_node.add_event_listener_with_callback(&event, closure.as_ref().unchecked_ref())?;

            closure.forget();
        }

        Ok(())
    }

    fn passover(&self, renderer: Renderer, path: Path, document: &web_sys::Document) {
        match &self.kind {
            VirtualKind::Template(template) => if let Ok(element) = renderer.get_element_by_path(&path, document) {
                let node = document.create_text_node(&template);

                if let Err(_) = element.append_child(&node) {
                    web_sys::console::error_1(&format!("failed to set template: {}", path).into());
                }
            },
            _ => {},
        }

        for (index, child) in self.kind.children().iter().enumerate() {
            let path = path.clone().concat(PathNode::new(index, String::from("virtual_node")));

            for (event, cb) in child.callbacks.iter() {
                if let Ok(element) = renderer.get_element_by_path(&path, document) {
                    if let Err(_) = child.attach_listener(renderer.clone(), element, event, cb) {
                        web_sys::console::error_1(&format!("failed to attach listener: {}", path).into());
                    }
                }
            }

            child.passover(renderer.clone(), path, document);
        }
    }

    pub fn reconcile(&self, renderer: Renderer, other: &VirtualNode, path: Path, document: &web_sys::Document) -> Result<(), JsValue> {
        if self.kind.children() != other.kind.children() {
            let children = self.kind.children()
                .iter()
                .map(|child| child.kind.render())
                .collect::<String>();

            let element = renderer.get_element_by_path(&path, document)?;

            element.set_inner_html(&children);

            self.passover(renderer, path, document);
        } else {
            for (index, (a, b)) in self.kind.children().iter().zip(other.kind.children().iter()).enumerate() {
                let path = path.clone().concat(PathNode::new(index, String::from("virtual_node")));

                a.reconcile(renderer.clone(), &b, path, &document)?;
            }
        }

        Ok(())
    }
}


