#[cfg(target_os = "linux")]
#[path = "../../support/linux.rs"]
mod support;

#[cfg(target_os = "linux")]
fn wake_two_of_three() -> i64 {
    // TODO: Start three waiters on the same futex word, store a new value, and
    // call FUTEX_WAKE_PRIVATE with count 2. Return the syscall result.
    todo!("observe the FUTEX_WAKE count")
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
