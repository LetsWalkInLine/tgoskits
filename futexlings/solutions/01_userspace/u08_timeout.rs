use std::sync::{Condvar, Mutex};
use std::time::Duration;

#[derive(Debug, Eq, PartialEq)]
enum WaitResult {
    Woken,
    TimedOut,
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

    fn store_and_wake(&self, value: u32) {
        *self.value.lock().unwrap() = value;
        self.cvar.notify_all();
    }

    fn wait_timeout(&self, expected: u32, timeout: Duration) -> WaitResult {
        let value = self.value.lock().unwrap();
        if *value != expected {
            return WaitResult::Mismatch;
        }

        let (value, timeout_result) = self
            .cvar
            .wait_timeout_while(value, timeout, |value| *value == expected)
            .unwrap();

        if timeout_result.timed_out() && *value == expected {
            WaitResult::TimedOut
        } else {
            WaitResult::Woken
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn mismatch_does_not_wait() {
        let futex = MiniFutex::new(2);
        assert_eq!(
            futex.wait_timeout(0, Duration::from_secs(10)),
            WaitResult::Mismatch
        );
    }

    #[test]
    fn reports_timeout() {
        let futex = MiniFutex::new(0);
        assert_eq!(
            futex.wait_timeout(0, Duration::from_millis(10)),
            WaitResult::TimedOut
        );
    }

    #[test]
    fn reports_wake_before_timeout() {
        let futex = Arc::new(MiniFutex::new(0));
        let waiter_futex = Arc::clone(&futex);

        let waiter = thread::spawn(move || waiter_futex.wait_timeout(0, Duration::from_secs(1)));

        thread::sleep(Duration::from_millis(20));
        futex.store_and_wake(1);

        assert_eq!(waiter.join().unwrap(), WaitResult::Woken);
    }
}
