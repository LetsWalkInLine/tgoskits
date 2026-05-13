use std::sync::{Arc, Condvar, Mutex};

#[derive(Default)]
struct State {
    ready: bool,
    value: u32,
}

type Shared = Arc<(Mutex<State>, Condvar)>;

fn wait_for_value(shared: &Shared) -> u32 {
    let (lock, cvar) = &**shared;
    let mut state = lock.lock().unwrap();

    // TODO: Wait until `state.ready` is true, using a loop around `cvar.wait`.
    todo!("wait in a condition loop, then return state.value");
}

fn publish(shared: &Shared, value: u32) {
    let (lock, cvar) = &**shared;
    let mut state = lock.lock().unwrap();
    state.value = value;
    state.ready = true;
    cvar.notify_one();
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn consumer_gets_published_value() {
        let shared = Arc::new((Mutex::new(State::default()), Condvar::new()));
        let consumer_shared = Arc::clone(&shared);

        let consumer = thread::spawn(move || wait_for_value(&consumer_shared));
        thread::sleep(Duration::from_millis(20));
        publish(&shared, 42);

        assert_eq!(consumer.join().unwrap(), 42);
    }
}
