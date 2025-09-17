//! This module contains everything related to the backend.

use crate::http;

use serde::{Serialize, Deserialize};
use erased_serde::Serialize as ErasedSerialize;


/// Represents an empty [`ServiceTransport`].
#[derive(Serialize, Deserialize)]
pub struct NullTransport;

/// Represents a server-side service.
pub trait Service: ErasedSerialize {
    /// The path of the service.
    const PATH: &'static str;

    /// The type that the service will output.
    type Output;

    /// Handle a call to the service.
    fn handle(self) -> Result<Self::Output, Box<dyn std::error::Error>>;

    /// Call the service.
    #[allow(async_fn_in_trait)]
    async fn call(&self) -> Result<Self::Output, Box<dyn std::error::Error>> {
        web_sys::console::log_1(&format!("called: {:?}", Self::PATH).into());

        let json = http::post(Self::PATH.to_string(), String::new()).await;

        web_sys::console::log_1(&format!("json: {:?}", json).into());

        todo!()
    }
}


