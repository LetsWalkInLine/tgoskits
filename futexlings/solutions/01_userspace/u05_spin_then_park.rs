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

    fn try_fast_lock(&self) -> bool {
        self.locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    fn lock(&self) -> TinyGuard<'_> {
        for _ in 0..8 {
            if self.try_fast_lock() {
                return TinyGuard { mutex: self };
            }
            std::hint::spin_loop();
        }

        let mut parked = self.park_lock.lock().unwrap();
        loop {
            if self.try_fast_lock() {
                return TinyGuard { mutex: self };
            }
            parked = self.cvar.wait(parked).unwrap();
        }
    }

    fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
        self.cvar.notify_one();
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
