#[cfg(target_os = "linux")]
#[path = "../../support/linux.rs"]
mod support;

#[cfg(target_os = "linux")]
fn cross_process_handoff() -> u32 {
    // TODO: Put an AtomicU32 in a shared anonymous mmap. After fork, have the
    // child store 1 and FUTEX_WAKE the parent using non-private futex ops.
    todo!("complete a cross-process futex handoff")
}

fn main() {}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    #[test]
    fn shared_mapping_can_wake_across_processes() {
        assert_eq!(cross_process_handoff(), 1);
    }
}

#[cfg(all(test, not(target_os = "linux")))]
mod tests {
    #[test]
    fn skipped_on_non_linux() {
        eprintln!("shared futex exercise skipped outside Linux");
    }
}
