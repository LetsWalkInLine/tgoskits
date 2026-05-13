#[cfg(target_os = "linux")]
#[path = "../../support/linux.rs"]
mod support;

#[cfg(target_os = "linux")]
fn handoff_with_futex() -> u32 {
    // TODO: Spawn a waiter that loops on an AtomicU32 and sleeps with
    // FUTEX_WAIT_PRIVATE. Store 1 in the word and wake it with FUTEX_WAKE_PRIVATE.
    todo!("complete a thread handoff using raw futex wait/wake")
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
