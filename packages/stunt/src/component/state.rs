//! The state of each component is stored globally with each its own [`Identity`].

use crate::component::BaseComponent;

use std::sync::{Arc, LazyLock};
use std::collections::HashMap;

use spin::Mutex;

static STATES: LazyLock<Arc<Mutex<HashMap<Identity, Arc<Mutex<dyn BaseComponent + Send + Sync>>>>>> = LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Represents the identity of a node within the virtual dom.
///
/// ## Warning
/// Identities are not supposed to be used outside the framework.
#[derive(Debug, Clone,  Hash, PartialEq, Eq)]
pub struct Identity {
    id: Vec<usize>,
}

impl Identity {
    pub(crate) fn new(id: usize) -> Identity {
        Identity {
            id: vec![id],
        }
    }

    /// Create an intersection of two identites
    pub fn intersect(&self, other: usize) -> Identity {
        Identity {
            id: [self.id.clone(), vec![other]].concat(),
        }
    }

    pub(crate) fn outer(&self) -> Identity {
        Identity {
            id: self.id[..self.id.len() - 1].to_vec(),
        }
    }

    pub(crate) fn render(&self) -> String {
        self.id.iter()
            .map(|id| format!("@{}", id))
            .collect::<String>()
    }
}

#[inline]
pub(crate) fn get(identity: &Identity) -> Arc<Mutex<dyn BaseComponent + Send + Sync>> {
    STATES.lock()[identity].clone()
}

#[inline]
pub(crate) fn get_or_insert(
    identity: &Identity,
    f: impl Fn() -> Arc<Mutex<dyn BaseComponent + Send + Sync>>
) -> Arc<Mutex<dyn BaseComponent + Send + Sync>> {
    let mut states = STATES.lock();

    match states.get(&identity) {
        Some(component) => component.clone(),
        None => {
            states.insert(identity.clone(), (f)());

            states[&identity].clone()
        },
    }
}


