#[cfg(target_os = "linux")]
#[path = "../../support/linux.rs"]
mod support;

#[cfg(target_os = "linux")]
use std::sync::atomic::AtomicU32;

#[cfg(target_os = "linux")]
use support::{EAGAIN, futex_wait};

#[cfg(target_os = "linux")]
fn wait_with_wrong_expected(word: &AtomicU32) -> Result<i64, i32> {
    futex_wait(word, 0, None, true)
}

fn main() {}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    #[test]
    fn mismatched_value_returns_eagain() {
        let word = AtomicU32::new(1);
        assert_eq!(wait_with_wrong_expected(&word), Err(EAGAIN));
    }
}

#[cfg(all(test, not(target_os = "linux")))]
mod tests {
    #[test]
    fn skipped_on_non_linux() {
        eprintln!("raw futex syscall exercise skipped outside Linux");
    }
}
