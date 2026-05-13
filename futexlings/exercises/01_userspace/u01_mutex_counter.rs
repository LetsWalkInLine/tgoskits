use std::sync::{Arc, Mutex};
use std::thread;

fn increment(shared: Arc<Mutex<u32>>, times: u32) {
    for _ in 0..times {
        // TODO: Lock the mutex and increment the shared counter.
        *shared.lock().unwrap() += 1;
    }
}

fn run_workers(workers: usize, per_worker: u32) -> u32 {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..workers {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || increment(counter, per_worker)));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    *counter.lock().unwrap()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_all_increments() {
        assert_eq!(run_workers(8, 1_000), 8_000);
    }
}
