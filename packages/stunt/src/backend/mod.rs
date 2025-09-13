//! This module contains everything related to backend services.

use std::collections::HashMap;

use erased_serde::Serialize as ErasedSerialize;
use serde::{Serialize, Deserialize};


/// Represents an empty [`ServiceTransport`].
#[derive(Serialize, Deserialize)]
pub struct NullTransport;

/// Represents the client-side caller to a [`Service`].
pub trait ServiceCaller: Service {
    /// Get the path of the service.
    fn path() -> &'static str;

    /// Call the service.
    fn call(self) -> Self::Output;
}

/// Represents a server-side service.
pub trait Service: ErasedSerialize {
    /// The type that the service will output.
    type Output: ErasedSerialize;

    /// Handle a call to the service.
    fn handle(self) -> Result<Self::Output, Box<dyn std::error::Error>>;
}

/// The entry point of the backend of a stunt application.
pub struct ServiceHandler {
    services: HashMap<String, Box<dyn Service<Output = dyn ErasedSerialize>>>,
}

impl ServiceHandler {
    /// Create a new ServiceHandler point.
    pub fn new() -> ServiceHandler {
        ServiceHandler {
            services: HashMap::new(),
        }
    }

    /// Add a backend service.
    pub fn service(
        mut self,
        path: String,
        service: Box<dyn Service<Output = dyn ErasedSerialize>>
    ) -> ServiceHandler {
        self.services.insert(path, service);

        self
    }
}


