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

use crate::component::state::{self, Path, PathBuilder};
use crate::component::html::AttrMap;
use crate::component::Component;
use crate::virtual_dom;

use spin::Mutex;

use std::marker::PhantomData;
use std::sync::Arc;


/// Represents the renderer
pub struct Renderer<T: Component + Send + Sync + 'static> {
    _component: PhantomData<T>,
}

impl<T: Component + Send + Sync + 'static> Renderer<T> {
    /// Create a new render instance
    pub fn new() -> Renderer<T> {
        Renderer {
            _component: PhantomData,
        }
    }

    /// Render the application
    pub fn render(self) {
        state::get_or_insert(&Path::new(), || Arc::new(Mutex::new(T::create())), "root");

        render();
    }
}

#[inline]
pub(crate) fn render() {
    let path = Path::new();

    let root = state::get(&path);
    let lock = root.lock();

    /*
    let render = lock.base_view(AttrMap::from(Vec::new().into_iter())).render(PathBuilder::default(), Path::new(), 0);

    web_sys::console::log_1(&format!("render: {:#?}", render).into());

    virtual_dom::reconcile(render);
    */
}


