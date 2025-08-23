//! The renderer is the entry point of a stunt application
//!
//! ## Example
//! ```rust,no_run
//! use stunt::prelude::*;
//!
//! pub struct App;
//!
//! impl Component for App {
//!     type Message = ();
//!     type Properties = ();
//!
//!     fn create() -> App { App }
//!
//!     fn callback(&mut self, _: &Self::Message) {}
//!
//!     fn view(&self, ctx: Context, _properties: ()) -> Tree {
//!         html! {
//!             <div></div>
//!         }
//!     }
//! }
//!
//! fn main() {
//!     Renderer::<App>::new().render();
//! }
//! ```

use crate::component::path::Path;
use crate::component::html::AttrMap;
use crate::component::{Component, BaseComponent};
use crate::virtual_dom::{VirtualKind, VirtualElement, VirtualNode};

use wasm_bindgen::prelude::*;
use spin::Mutex;

use std::collections::HashMap;
use std::sync::Arc;
use std::rc::Rc;

// - Custom panic hook
// - Custom root node
// - Node reference, a unique id can be assigned to a struct, and used as id on a element and accessed later


/// Represents the root node used by the [`Renderer`].
pub struct RenderRoot {
    element: web_sys::Node,
    xpath: String,
}

impl RenderRoot {
    /// Create a new render root.
    pub fn new(element: web_sys::Node, xpath: String) -> RenderRoot {
        RenderRoot {
            element,
            xpath,
        }
    }
}

/// Represents the renderer.
#[derive(Clone)]
pub struct Renderer {
    components: Arc<Mutex<HashMap<Path, Arc<Mutex<dyn BaseComponent + Send + Sync>>>>>,
    previous: Arc<Mutex<VirtualNode>>,
    root: Rc<RenderRoot>,
}

impl Renderer {
    /// Create a new render instance with a render root of the document body.
    pub fn new<T: Component + Send + Sync + 'static>() -> Renderer {
        let window = web_sys::window().expect("no global window exists");
        let document = window.document().expect("should have a document on window");

        Renderer::new_with_root::<T>(RenderRoot::new(document.get_root_node(), String::from("/html/body")))
    }

    /// Create a new render instance with a render root.
    pub fn new_with_root<T: Component + Send + Sync + 'static>(root: RenderRoot) -> Renderer {
        let component = Arc::new(Mutex::new(T::create()));

        Renderer {
            components: Arc::new(Mutex::new(HashMap::from([(Path::new(), component as Arc<Mutex<dyn BaseComponent + Send + Sync>>)]))),
            previous: Arc::new(Mutex::new(VirtualNode::default())),
            root: Rc::new(root),
        }
    }

    pub(crate) fn get_element_by_path(&self, path: &Path, document: &web_sys::Document) -> Result<web_sys::HtmlElement, JsValue> {
        let node = document.evaluate(&format!("{}{}", self.root.xpath, path), &self.root.element)?
            .iterate_next()?
            .ok_or(JsValue::from_str("failed to get node"))?;

        node.dyn_ref::<web_sys::HtmlElement>()
            .map(|element| element.clone())
            .ok_or(JsValue::from_str("failed to cast"))
    }

    pub(crate) fn get(&self, path: &Path) -> Arc<Mutex<dyn BaseComponent + Send + Sync>> {
        self.components.lock()[path].clone()
    }

    pub(crate) fn get_or_insert(&self, path: &Path, f: impl Fn() -> Arc<Mutex<dyn BaseComponent + Send + Sync>>) -> Arc<Mutex<dyn BaseComponent + Send + Sync>> {
        let mut components = self.components.lock();

        match components.get(path) {
            Some(component) => component.clone(),
            None => {
                components.insert(path.clone(), (f)());

                components[&path].clone()
            },
        }
    }

    /// Render the application.
    pub fn render(&self) {
        let root = self.get(&Path::new());
        let lock = root.lock();

        let render = lock.base_view(AttrMap::from(Vec::new().into_iter())).render(self.clone(), Path::new());

        let vdom = VirtualNode::new(Arc::new(Vec::new()), VirtualKind::Element(VirtualElement::new(String::from("root"), String::new(), Arc::new(render))), Path::new());

        let mut previous = self.previous.lock();

        let window = web_sys::window().expect("no global window exists");
        let document = window.document().expect("should have a document on window");

        match vdom.reconcile(self.clone(), &*previous, Path::new(), &document) {
            Ok(()) => *previous = vdom,
            Err(err) => {
                web_sys::console::error_1(&format!("failed to reconcile: {:?}", err).into());
            },
        }
    }
}


