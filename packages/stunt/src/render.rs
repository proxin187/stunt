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

use crate::component::tree::AttrMap;
use crate::component::state::{self, Identity};
use crate::component::{Component, Context};
use crate::vdom;

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
        state::get_or_insert(&Identity::new(0), || Arc::new(Mutex::new(T::create())));

        render();
    }
}

#[inline]
pub(crate) fn render() {
    let identity = Identity::new(0);

    let root = state::get(&identity);
    let lock = root.lock();

    let render = lock.base_view(Context::new(identity), AttrMap::from(Vec::new().into_iter())).render();

    vdom::reconcile(render);
}


