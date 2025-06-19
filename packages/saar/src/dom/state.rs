use crate::dom::component::Component;

use std::sync::{Arc, LazyLock};
use std::collections::HashMap;

use spin::Mutex;

static STATES: LazyLock<Arc<Mutex<HashMap<Identity, Arc<dyn Component + Send + Sync>>>>> = LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));


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
pub fn get(identity: &Identity) -> Arc<dyn Component + Send + Sync> {
    STATES.lock()[identity].clone()
}

#[inline]
pub fn get_or_insert(identity: Identity, f: Box<dyn Fn() -> Arc<dyn Component + Send + Sync>>) -> Arc<dyn Component + Send + Sync> {
    let mut states = STATES.lock();

    match states.get(&identity) {
        Some(component) => component.clone(),
        None => {
            states.insert(identity.clone(), (f)());

            states[&identity].clone()
        },
    }
}


