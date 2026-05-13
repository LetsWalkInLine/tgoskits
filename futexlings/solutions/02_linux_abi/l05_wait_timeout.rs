#[cfg(target_os = "linux")]
#[path = "../../support/linux.rs"]
mod support;

#[cfg(target_os = "linux")]
use std::sync::atomic::AtomicU32;

#[cfg(target_os = "linux")]
use support::{ETIMEDOUT, Timespec, futex_wait};

#[cfg(target_os = "linux")]
fn short_relative_timeout() -> Timespec {
    Timespec {
        tv_sec: 0,
        tv_nsec: 10_000_000,
    }
}

#[cfg(target_os = "linux")]
fn wait_until_timeout(word: &AtomicU32) -> Result<i64, i32> {
    let timeout = short_relative_timeout();
    futex_wait(word, 0, Some(&timeout), true)
}

fn main() {}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    #[test]
    fn wait_times_out_when_value_stays_equal() {
        let word = AtomicU32::new(0);
        assert_eq!(wait_until_timeout(&word), Err(ETIMEDOUT));
    }
}

#[cfg(all(test, not(target_os = "linux")))]
mod tests {
    #[test]
    fn skipped_on_non_linux() {
        eprintln!("raw futex syscall exercise skipped outside Linux");
    }
}
