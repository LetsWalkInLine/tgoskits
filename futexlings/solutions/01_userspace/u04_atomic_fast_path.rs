use std::sync::atomic::{AtomicU32, Ordering};

struct FastLock {
    state: AtomicU32,
}

impl FastLock {
    fn new() -> Self {
        Self {
            state: AtomicU32::new(0),
        }
    }

    fn try_lock(&self) -> bool {
        self.state
            .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    fn unlock(&self) {
        self.state.store(0, Ordering::Release);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_one_fast_path_acquire_succeeds() {
        let lock = FastLock::new();

        assert!(lock.try_lock());
        assert!(!lock.try_lock());

        lock.unlock();
        assert!(lock.try_lock());
    }
}
