use std::sync::{Condvar, Mutex};

#[derive(Debug, Eq, PartialEq)]
enum WaitResult {
    Woken,
    Mismatch,
}

struct MiniFutex {
    value: Mutex<u32>,
    cvar: Condvar,
}

impl MiniFutex {
    fn new(value: u32) -> Self {
        Self {
            value: Mutex::new(value),
            cvar: Condvar::new(),
        }
    }

    fn store(&self, value: u32) {
        *self.value.lock().unwrap() = value;
    }

    fn wait(&self, expected: u32) -> WaitResult {
        let mut value = self.value.lock().unwrap();
        if *value != expected {
            return WaitResult::Mismatch;
        }

        while *value == expected {
            value = self.cvar.wait(value).unwrap();
        }

        WaitResult::Woken
    }

    fn wake_all(&self) {
        self.cvar.notify_all();
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn mismatch_returns_without_sleeping() {
        let futex = MiniFutex::new(7);
        assert_eq!(futex.wait(0), WaitResult::Mismatch);
    }

    #[test]
    fn waiter_observes_wake_after_value_changes() {
        let futex = Arc::new(MiniFutex::new(0));
        let waiter_futex = Arc::clone(&futex);

        let waiter = thread::spawn(move || waiter_futex.wait(0));
        thread::sleep(Duration::from_millis(20));
        futex.store(1);
        futex.wake_all();

        assert_eq!(waiter.join().unwrap(), WaitResult::Woken);
    }
}
