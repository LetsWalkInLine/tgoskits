#[cfg(target_os = "linux")]
#[path = "../../support/linux.rs"]
mod support;

#[cfg(target_os = "linux")]
fn handoff_with_futex() -> u32 {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::thread;
    use std::time::Duration;
    use support::{EAGAIN, futex_wait, futex_wake};

    let word = Arc::new(AtomicU32::new(0));
    let waiter_word = Arc::clone(&word);

    let waiter = thread::spawn(move || {
        while waiter_word.load(Ordering::Acquire) == 0 {
            match futex_wait(&waiter_word, 0, None, true) {
                Ok(_) | Err(EAGAIN) => {}
                Err(errno) => panic!("futex_wait failed with errno {errno}"),
            }
        }
        waiter_word.load(Ordering::Acquire)
    });

    thread::sleep(Duration::from_millis(20));
    word.store(1, Ordering::Release);
    assert_eq!(futex_wake(&word, 1, true).unwrap(), 1);

    waiter.join().unwrap()
}

fn main() {}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    #[test]
    fn raw_futex_handoff_completes() {
        assert_eq!(handoff_with_futex(), 1);
    }
}

#[cfg(all(test, not(target_os = "linux")))]
mod tests {
    #[test]
    fn skipped_on_non_linux() {
        eprintln!("raw futex syscall exercise skipped outside Linux");
    }
}
