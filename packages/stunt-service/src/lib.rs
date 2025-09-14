//! This module contains everything related to backend services.

use erased_serde::Serialize as ErasedSerialize;
use serde::{Serialize, Deserialize};

/// Represents an empty [`ServiceTransport`].
#[derive(Serialize, Deserialize)]
pub struct NullTransport;

/// Represents the client-side caller to a [`Service`].
pub trait ServiceCaller: Service {
    /// Get the path of the service.
    fn path() -> &'static str;

    fn call(&self) -> Result<Self::Output, Box<dyn std::error::Error>> {
        let mut response = ureq::post("127.0.0.1/api/register")
            .send_json(self)?;

        Ok(response.body_mut().read_json::<Self::Output>()?)
    }
}

/// Represents a server-side service.
pub trait Service: ErasedSerialize {
    /// The type that the service will output.
    type Output: ErasedSerialize;

    /// Handle a call to the service.
    fn handle(self) -> Result<Self::Output, Box<dyn std::error::Error>>;
}


