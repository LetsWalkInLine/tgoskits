use std::sync::{Arc, Condvar, Mutex};

type Gate = Arc<(Mutex<bool>, Condvar)>;

fn wait_until_open(gate: &Gate) -> bool {
    let (lock, cvar) = &**gate;
    let mut open = lock.lock().unwrap();

    // TODO: This single `if` is vulnerable to spurious wakeups.
    // Replace it with a condition loop.
    if !*open {
        open = cvar.wait(open).unwrap();
    }

    *open
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn ignores_wakeup_when_condition_is_still_false() {
        let gate = Arc::new((Mutex::new(false), Condvar::new()));
        let waiter_gate = Arc::clone(&gate);

        let waiter = thread::spawn(move || wait_until_open(&waiter_gate));

        thread::sleep(Duration::from_millis(20));
        gate.1.notify_all();

        thread::sleep(Duration::from_millis(20));
        *gate.0.lock().unwrap() = true;
        gate.1.notify_all();

        assert!(waiter.join().unwrap());
    }
}
