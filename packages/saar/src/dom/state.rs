use crate::dom::component::Component;

use std::sync::{LazyLock, Mutex};


struct State {
    pub state: Vec<Box<dyn Component>>,
}

impl State {
    pub fn new() -> State {
        State {
            state: Vec::new(),
        }
    }

    pub fn push(&mut self, component: Box<dyn Component>) -> usize {
        self.state.push(component);

        self.state.len() - 1
    }
}

pub fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    thread_local! {
        static STATE: LazyLock<Mutex<State>> = LazyLock::new(|| Mutex::new(State::new()));
    }

    STATE.with(|scheduler| f(&mut scheduler.lock().expect("failed to lock")))
}


