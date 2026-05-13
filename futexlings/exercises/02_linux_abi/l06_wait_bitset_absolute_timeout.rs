#[cfg(target_os = "linux")]
#[path = "../../support/linux.rs"]
mod support;

#[cfg(target_os = "linux")]
use std::sync::atomic::AtomicU32;

#[cfg(target_os = "linux")]
use support::{EINVAL, ETIMEDOUT, FUTEX_BITSET_MATCH_ANY, Timespec, futex_wait_bitset};

#[cfg(target_os = "linux")]
fn past_absolute_deadline() -> Timespec {
    Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    }
}

#[cfg(target_os = "linux")]
fn valid_wait_bitset() -> u32 {
    // TODO: FUTEX_WAIT_BITSET requires val3, the bitset, to be nonzero.
    todo!("return a nonzero wait bitset")
}

fn main() {}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    #[test]
    fn zero_bitset_is_invalid() {
        let word = AtomicU32::new(0);
        assert_eq!(
            futex_wait_bitset(&word, 0, &past_absolute_deadline(), 0, true),
            Err(EINVAL)
        );
    }

    #[test]
    fn nonzero_bitset_uses_absolute_timeout() {
        let word = AtomicU32::new(0);
        assert_eq!(
            futex_wait_bitset(
                &word,
                0,
                &past_absolute_deadline(),
                valid_wait_bitset(),
                true
            ),
            Err(ETIMEDOUT)
        );
    }
}

#[cfg(all(test, not(target_os = "linux")))]
mod tests {
    #[test]
    fn skipped_on_non_linux() {
        eprintln!("raw futex syscall exercise skipped outside Linux");
    }
}
