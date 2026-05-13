use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Condvar, Mutex};

struct TinyMutex {
    locked: AtomicBool,
    park_lock: Mutex<()>,
    cvar: Condvar,
}

struct TinyGuard<'a> {
    mutex: &'a TinyMutex,
}

impl TinyMutex {
    fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
            park_lock: Mutex::new(()),
            cvar: Condvar::new(),
        }
    }

    fn lock(&self) -> TinyGuard<'_> {
        // TODO: First try a few atomic CAS attempts. If the lock is still held,
        // park on the condvar and retry after each wake.
        todo!("spin, then park, then retry the atomic fast path")
    }

    fn unlock(&self) {
        // TODO: Store the unlocked state and wake one parked waiter.
        todo!("unlock and notify one parked waiter")
    }
}

impl Drop for TinyGuard<'_> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn serializes_counter_updates() {
        let mutex = Arc::new(TinyMutex::new());
        let counter = Arc::new(Mutex::new(0));
        let mut handles = Vec::new();

        for _ in 0..6 {
            let mutex = Arc::clone(&mutex);
            let counter = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                for _ in 0..500 {
                    let _guard = mutex.lock();
                    *counter.lock().unwrap() += 1;
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 3_000);
    }
}
