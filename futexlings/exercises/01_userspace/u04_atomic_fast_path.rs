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
        // TODO: Acquire the lock by changing state from 0 to 1.
        todo!("use compare_exchange for the uncontended fast path")
    }

    fn unlock(&self) {
        // TODO: Release the lock by storing 0.
        todo!("store the unlocked state with release ordering")
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
