#[cfg(target_os = "linux")]
#[path = "../../support/linux.rs"]
mod support;

#[cfg(target_os = "linux")]
fn wake_two_of_three() -> i64 {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
    use std::thread;
    use std::time::Duration;
    use support::{EAGAIN, futex_wait, futex_wake};

    let word = Arc::new(AtomicU32::new(0));
    let ready = Arc::new(AtomicUsize::new(0));
    let mut waiters = Vec::new();

    for _ in 0..3 {
        let word = Arc::clone(&word);
        let ready = Arc::clone(&ready);
        waiters.push(thread::spawn(move || {
            ready.fetch_add(1, Ordering::Release);
            while word.load(Ordering::Acquire) == 0 {
                match futex_wait(&word, 0, None, true) {
                    Ok(_) | Err(EAGAIN) => {}
                    Err(errno) => panic!("futex_wait failed with errno {errno}"),
                }
            }
        }));
    }

    while ready.load(Ordering::Acquire) != 3 {
        thread::yield_now();
    }
    thread::sleep(Duration::from_millis(20));

    word.store(1, Ordering::Release);
    let woke = futex_wake(&word, 2, true).unwrap();
    assert_eq!(futex_wake(&word, 3, true).unwrap(), 1);

    for waiter in waiters {
        waiter.join().unwrap();
    }

    woke
}

fn main() {}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    #[test]
    fn wake_count_is_capped_by_requested_count() {
        assert_eq!(wake_two_of_three(), 2);
    }
}

#[cfg(all(test, not(target_os = "linux")))]
mod tests {
    #[test]
    fn skipped_on_non_linux() {
        eprintln!("raw futex syscall exercise skipped outside Linux");
    }
}
