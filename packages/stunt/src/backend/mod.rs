//! This module contains everything related to backend services.

use std::collections::HashMap;

use erased_serde::{Serialize, Deserializer};


pub trait ServiceTransport<'a>: Serialize + Deserializer<'a> {}

pub trait Service<'a, I, R: ServiceTransport<'a>> {
    fn call(&self, input: I) -> Result<R, Box<dyn std::error::Error>>;
}

/// The entry point of the backend of a stunt application.
pub struct ServiceHandler<'a> {
    services: HashMap<String, Box<dyn Service<'a, dyn ServiceTransport<'a>, dyn ServiceTransport<'a>>>>,
}

impl<'a> ServiceHandler<'a> {
    /// Create a new ServiceHandler point.
    pub fn new() -> ServiceHandler<'a> {
        ServiceHandler {
            services: HashMap::new(),
        }
    }

    /// Add a backend service.
    pub fn service(mut self, path: String, service: Box<dyn Service<'a, dyn ServiceTransport<'a>, dyn ServiceTransport<'a>>>) -> ServiceHandler<'a> {
        self.services.insert(path, service);

        self
    }
}


