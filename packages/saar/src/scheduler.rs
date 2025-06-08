use std::collections::VecDeque;
use std::any::Any;

use std::sync::{LazyLock, Arc};

use spin::Mutex;

static SCHEDULER: LazyLock<Arc<Scheduler>> = LazyLock::new(|| Arc::new(Scheduler::new()));


pub struct Callback {
    inner: Box<dyn Any + Send + Sync>,
    scope: usize,
}

impl Callback {
    pub fn new(inner: Box<dyn Any + Send + Sync>, scope: usize) -> Callback {
        Callback {
            inner,
            scope,
        }
    }
}

pub struct Scheduler {
    queue: Mutex<VecDeque<Callback>>,
}

impl Scheduler {
    fn new() -> Scheduler {
        Scheduler {
            queue: Mutex::new(VecDeque::new()),
        }
    }

    fn send(&self, callback: Callback) {
        self.queue.lock().push_back(callback);
    }

    fn recv(&self) -> Callback {
        loop {
            if let Some(callback) = self.queue.lock().pop_front() {
                return callback;
            }
        }
    }
}

#[inline]
pub fn send(callback: Callback) {
    SCHEDULER.send(callback)
}

#[inline]
pub fn recv() -> Callback {
    SCHEDULER.recv()
}


