//! This module contains everything related to the backend.

use serde::{Serialize, Deserialize};
use erased_serde::Serialize as ErasedSerialize;
use web_sys::{Request, RequestInit, RequestMode, Response};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::prelude::*;


/// Represents an empty [`ServiceTransport`].
#[derive(Serialize, Deserialize)]
pub struct NullTransport;

/// Represents the client-side caller to a [`Service`].
pub trait ServiceCaller: Service {
    /// Get the path of the service.
    fn path() -> &'static str;

    /// Call the service.
    fn call(&self) -> Result<Self::Output, Box<dyn std::error::Error>> {
        wasm_bindgen_futures::spawn_local();
    }
}

/// Represents a server-side service.
pub trait Service: ErasedSerialize {
    /// The type that the service will output.
    type Output;

    /// Handle a call to the service.
    fn handle(self) -> Result<Self::Output, Box<dyn std::error::Error>>;
}

#[inline]
async fn fetch(url: &str) -> Result<JsValue, JsValue> {
    let opts = RequestInit::new();

    opts.set_method("POST");

    opts.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;

    request.headers().set("Accept", "application/json");

    let window = web_sys::window().expect("no window found");

    let value = JsFuture::from(window.fetch_with_request(&request)).await?;

    let response = value.dyn_into::<Response>().unwrap();

    let json = JsFuture::from(response.json()?).await?;

    Ok(json)
}


