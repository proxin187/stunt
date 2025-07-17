use std::collections::HashMap;
use std::sync::{LazyLock, Arc};
use std::any::{Any, TypeId};

use spin::Mutex;

static GLOBALS: LazyLock<Arc<Mutex<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>>> = LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

#[inline]
pub fn use_global<T: Any + Send + Sync + Default>() -> Arc<T> {
    let mut lock = GLOBALS.lock();

    if !lock.contains_key(&TypeId::of::<T>()) {
        lock.insert(TypeId::of::<T>(), Arc::new(T::default()));
    }

    let any = lock.get(&TypeId::of::<T>()).expect("internal error").clone();

    any.downcast().expect("internal error")
}

#[inline]
pub fn update_global<T: Any + Send + Sync + Default>(f: impl Fn(Arc<T>) -> T) {
    let update = f(use_global());

    GLOBALS.lock().insert(TypeId::of::<T>(), Arc::new(update));
}


