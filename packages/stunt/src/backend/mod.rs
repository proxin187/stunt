//! This module contains everything related to backend services.

use serde::{Serialize, Deserialize};


/// Represents input that can be passed to a [`Service`].
pub trait ServiceInput: Serialize + Deserialize<'static> {}

pub struct Service<I: ServiceInput> {
    path: String,
    service: Box<dyn Fn(I)>,
}

/// The entry point of the backend of a stunt application.
pub struct Entry {
    services: Vec<Service<Box<dyn ServiceInput>>>,
}

impl Entry {
    /// Create a new entry point.
    pub fn new() -> Entry {
        Entry {
            services: Vec::new(),
        }
    }

    /// Add a backend service.
    pub fn service(mut self, path: String, service: Box<dyn Fn()>) -> Entry {
        self.services.push(Service {
            path,
            service,
        });

        self
    }
}


