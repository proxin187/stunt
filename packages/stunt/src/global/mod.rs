
//! Share a mutable value globally across your application. Globals only allow for one
//! value per type, if you want to use multiple values of the same type at the same time you should
//! use a Struct.
//!
//!
//! ## Example
//! ```rust,no_run
//! use stunt::prelude::*;
//! use stunt::global;
//!
//! #[derive(Clone, Default, PartialEq)]
//! pub enum Theme {
//!     #[default]
//!     Light,
//!     Dark,
//! }
//!
//! impl Theme {
//!     pub fn background(&self) -> &'static str {
//!         match self {
//!             Theme::Light => "#000000ff",
//!             Theme::Dark => "#ffffffff",
//!         }
//!     }
//! }
//!
//! pub enum Message {
//!    Change,
//! }
//!
//! pub struct App;
//!
//! impl Component for App {
//!     type Message = Message;
//!     type Properties = ();
//!
//!     fn create() -> App { App }
//!
//!     fn callback(&mut self, message: &Message) {
//!         match message {
//!             Message::Change => {
//!                 global::use_global(|theme: &mut Theme| {
//!                     match theme {
//!                         Theme::Light => *theme = Theme::Dark,
//!                         Theme::Dark => *theme = Theme::Light,
//!                     }
//!                 });
//!             },
//!         }
//!     }
//!
//!     fn view(&self, ctx: Context, _properties: ()) -> Tree {
//!         let theme = global::use_global(|theme: &mut Theme| theme.clone());
//!
//!         html! {
//!             <div>
//!                 <button onclick={ Message::Change } >
//!                     { "change theme" }
//!                 </button>
//!                 <h1 style={ format!("background-color: {};", theme.background()) }>
//!                     { "the background color of this text should change on click" }
//!                 </h1>
//!             </div>
//!         }
//!     }
//! }
//!
//! fn main() {
//!     Renderer::<App>::new().render();
//! }
//! ```

use std::collections::HashMap;
use std::sync::{LazyLock, Arc};
use std::any::{Any, TypeId};

use spin::Mutex;

static GLOBALS: LazyLock<Arc<Mutex<HashMap<TypeId, Box<dyn Any + Send + Sync>>>>> = LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));


/// Calls a closure with a mutable reference to a global of the generic type. You can use this
/// function to both mutate the state of the global and to extract a value for use outside of your closure.
///
/// ## Example
/// ```rust,no_run
/// use stunt::global;
///
/// global::use_global(|foo: &mut usize| *foo += 1);
/// ```

#[inline]
pub fn use_global<T: Any + Send + Sync + Default, R>(f: impl Fn(&mut T) -> R) -> R {
    let mut lock = GLOBALS.lock();

    if !lock.contains_key(&TypeId::of::<T>()) {
        lock.insert(TypeId::of::<T>(), Box::new(T::default()));
    }

    let any = lock.get_mut(&TypeId::of::<T>()).expect("internal error");

    f(any.downcast_mut().expect("internal error"))
}


