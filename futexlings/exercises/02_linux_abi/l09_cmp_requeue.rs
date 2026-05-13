#[cfg(target_os = "linux")]
#[path = "../../support/linux.rs"]
mod support;

#[cfg(target_os = "linux")]
fn requeue_one_waiter() -> i64 {
    // TODO: Put one waiter to sleep on a source futex, move it to a target futex
    // with FUTEX_CMP_REQUEUE_PRIVATE, then wake it from the target futex.
    todo!("observe FUTEX_CMP_REQUEUE")
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
