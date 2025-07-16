use crate::component::BaseComponent;

use std::sync::{Arc, LazyLock};
use std::collections::HashMap;

use spin::Mutex;

static STATES: LazyLock<Arc<Mutex<HashMap<Identity, Arc<Mutex<dyn BaseComponent + Send + Sync>>>>>> = LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));


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

    pub fn intersect(&self, other: Identity) -> Identity {
        Identity {
            id: [self.id.clone(), other.id].concat(),
        }
    }

    pub fn outer(&self) -> Identity {
        Identity {
            id: self.id[..self.id.len() - 1].to_vec(),
        }
    }

    pub fn render(&self) -> String {
        self.id.iter()
            .map(|id| format!("@{}", id))
            .collect::<String>()
    }
}

#[inline]
pub fn get(identity: &Identity) -> Arc<Mutex<dyn BaseComponent + Send + Sync>> {
    STATES.lock()[identity].clone()
}

#[inline]
pub fn get_or_insert(
    identity: &Identity,
    f: fn() -> Arc<Mutex<dyn BaseComponent + Send + Sync>>
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


