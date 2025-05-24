use crate::error::Error;

use std::sync::{Mutex, Condvar};
use std::collections::VecDeque;
use std::any::Any;


pub struct Callback {
    inner: Box<dyn Any>,
    id: usize,
}

impl Callback {
    pub fn new(inner: Box<dyn Any>, id: usize) -> Callback {
        Callback {
            inner,
            id,
        }
    }
}

pub struct Scheduler {
    queue: Mutex<VecDeque<Callback>>,
    cond: Condvar,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            queue: Mutex::new(VecDeque::new()),
            cond: Condvar::new(),
        }
    }

    pub fn send(&self, callback: Callback) -> Result<(), Box<dyn std::error::Error>> {
        let mut lock = self.queue.lock().map_err(|_| Error::LockFailed)?;

        lock.push_back(callback);

        Ok(())
    }

    pub fn recv(&self) -> Result<Callback, Box<dyn std::error::Error>> {
        let mut lock = self.queue.lock().map_err(|_| Error::LockFailed)?;

        loop {
            if let Some(callback) = lock.pop_front() {
                return Ok(callback);
            } else {
                lock = self.cond.wait(lock).map_err(|_| Error::LockFailed)?;
            }
        }
    }
}

pub fn with<R>(f: impl FnOnce(&Scheduler) -> R) -> R {
    thread_local! {
        static SCHEDULER: Scheduler = Scheduler::new();
    }

    SCHEDULER.with(|scheduler| f(scheduler))
}


