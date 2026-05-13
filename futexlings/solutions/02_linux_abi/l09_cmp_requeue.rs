#[cfg(target_os = "linux")]
#[path = "../../support/linux.rs"]
mod support;

#[cfg(target_os = "linux")]
fn requeue_one_waiter() -> i64 {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
    use std::thread;
    use std::time::Duration;
    use support::{EAGAIN, futex_cmp_requeue, futex_wait, futex_wake};

    let source = Arc::new(AtomicU32::new(0));
    let target = Arc::new(AtomicU32::new(0));
    let ready = Arc::new(AtomicBool::new(false));

    let waiter_source = Arc::clone(&source);
    let waiter_ready = Arc::clone(&ready);
    let waiter = thread::spawn(move || {
        waiter_ready.store(true, Ordering::Release);
        match futex_wait(&waiter_source, 0, None, true) {
            Ok(_) | Err(EAGAIN) => 1,
            Err(errno) => panic!("futex_wait failed with errno {errno}"),
        }
    });

    while !ready.load(Ordering::Acquire) {
        thread::yield_now();
    }
    thread::sleep(Duration::from_millis(20));

    let moved = futex_cmp_requeue(&source, &target, 0, 1, 0, true).unwrap();
    assert_eq!(moved, 1);
    assert_eq!(futex_wake(&target, 1, true).unwrap(), 1);

    waiter.join().unwrap()
}

fn main() {}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicU32;
    use support::{EAGAIN, futex_cmp_requeue};

    #[test]
    fn compare_mismatch_returns_eagain() {
        let source = AtomicU32::new(1);
        let target = AtomicU32::new(0);

        assert_eq!(
            futex_cmp_requeue(&source, &target, 0, 1, 0, true),
            Err(EAGAIN)
        );
    }

    #[test]
    fn requeue_moves_waiter_to_target_queue() {
        assert_eq!(requeue_one_waiter(), 1);
    }
}

#[cfg(all(test, not(target_os = "linux")))]
mod tests {
    #[test]
    fn skipped_on_non_linux() {
        eprintln!("raw futex syscall exercise skipped outside Linux");
    }
}
