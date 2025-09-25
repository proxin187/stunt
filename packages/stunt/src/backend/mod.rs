//! This module contains everything related to the backend.
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
//!     fn view(&self, _properties: ()) -> Html {
//!         html! {
//!             <div></div>
//!         }
//!     }
//! }
//!
//! fn main() {
//!     Renderer::new::<App>().render();
//! }
//! ```

use crate::http;

use serde::Serialize;
use serde::de::DeserializeOwned;


/// Represents a server-side service.
pub trait Service: Serialize + Clone + Sized + 'static {
    /// The path of the service.
    const PATH: &'static str;

    /// The type that the service will output.
    type Output: Serialize + DeserializeOwned;

    /// Handle a call to the service.
    fn handle(self) -> Self::Output;

    /// Call the service.
    fn call(self, f: impl Fn(Self::Output) + 'static) {
        wasm_bindgen_futures::spawn_local(async move {
            match http::post::<Self, Self::Output>(Self::PATH.to_string(), &self).await {
                Ok(output) => f(output),
                Err(err) => {
                    web_sys::console::error_1(&format!("{:?}", err).into());
                },
            }
        });
    }

    /// An actix-web route handler for the service.
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "actix")]
    #[allow(async_fn_in_trait)]
    async fn actix_handler(json: actix_web::web::Json<Self>) -> impl actix_web::Responder {
        let response = json.clone().handle();

        actix_web::HttpResponse::Ok()
            .json(response)
    }
}


