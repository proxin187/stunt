use crate::dom::component::Component;

use std::sync::{Arc, LazyLock};
use std::collections::HashMap;

use spin::Mutex;

static STATES: LazyLock<Arc<Mutex<HashMap<Identity, Arc<dyn Component + Send + Sync>>>>> = LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

// static IDENTITY: LazyLock<Arc<Mutex<Identity>>> = LazyLock::new(|| Arc::new(Mutex::new(Identity::new())));


// TODO: we can maybe have that the identity is set compile time

#[derive(Debug, Clone,  Hash, PartialEq, Eq)]
pub struct Identity {
    id: Vec<usize>,
}

impl Identity {
    pub fn new(id: usize) -> Identity {
        Identity {
            id: vec![id],
        }
    }

    pub fn intersect(self, other: Identity) -> Identity {
        Identity {
            id: [self.id, other.id].concat(),
        }
    }
}

#[inline]
pub fn get(identity: Identity) -> Arc<dyn Component + Send + Sync> {
    STATES.lock()[&identity].clone()
}

#[inline]
pub fn insert_if_none(identity: Identity, component: Box<dyn Fn() -> Arc<dyn Component + Send + Sync>>) {
    let mut states = STATES.lock();

    if states.get(&identity).is_none() {
        states.insert(identity, (component)());
    }
}


