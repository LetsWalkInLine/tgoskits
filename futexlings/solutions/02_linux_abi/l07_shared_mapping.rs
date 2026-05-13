#[cfg(target_os = "linux")]
#[path = "../../support/linux.rs"]
mod support;

#[cfg(target_os = "linux")]
fn cross_process_handoff() -> u32 {
    use std::ffi::c_void;
    use std::ptr;
    use std::sync::atomic::{AtomicU32, Ordering};
    use support::{
        _exit, MAP_ANONYMOUS, MAP_FAILED, MAP_SHARED, PROT_READ, PROT_WRITE, fork, futex_wait,
        futex_wake, mmap, munmap, usleep, waitpid,
    };

    let len = size_of::<AtomicU32>();
    let map = unsafe {
        mmap(
            ptr::null_mut(),
            len,
            PROT_READ | PROT_WRITE,
            MAP_SHARED | MAP_ANONYMOUS,
            -1,
            0,
        )
    };
    assert_ne!(map, MAP_FAILED);

    let word = map.cast::<AtomicU32>();
    unsafe {
        word.write(AtomicU32::new(0));
    }

    let pid = unsafe { fork() };
    assert!(pid >= 0);

    if pid == 0 {
        unsafe {
            usleep(20_000);
            (*word).store(1, Ordering::Release);
            let _ = futex_wake(&*word, 1, false);
            _exit(0);
        }
    }

    while unsafe { (*word).load(Ordering::Acquire) } == 0 {
        let _ = unsafe { futex_wait(&*word, 0, None, false) };
    }

    let mut status = 0;
    assert_eq!(unsafe { waitpid(pid, &mut status, 0) }, pid);
    let observed = unsafe { (*word).load(Ordering::Acquire) };
    assert_eq!(unsafe { munmap(map.cast::<c_void>(), len) }, 0);
    observed
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
