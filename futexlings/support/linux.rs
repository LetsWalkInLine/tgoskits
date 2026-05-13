#![cfg(target_os = "linux")]

use std::ffi::{c_int, c_long, c_uint, c_void};
use std::io;
use std::ptr;
use std::sync::atomic::AtomicU32;

pub const FUTEX_WAIT: c_int = 0;
pub const FUTEX_WAKE: c_int = 1;
pub const FUTEX_CMP_REQUEUE: c_int = 4;
pub const FUTEX_WAIT_BITSET: c_int = 9;
pub const FUTEX_PRIVATE_FLAG: c_int = 128;
pub const FUTEX_BITSET_MATCH_ANY: u32 = u32::MAX;

pub const EAGAIN: i32 = 11;
pub const EINVAL: i32 = 22;
pub const ETIMEDOUT: i32 = 110;

pub const PROT_READ: c_int = 0x1;
pub const PROT_WRITE: c_int = 0x2;
pub const MAP_SHARED: c_int = 0x01;
pub const MAP_ANONYMOUS: c_int = 0x20;
pub const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

#[cfg(target_arch = "x86")]
const SYS_FUTEX: c_long = 240;
#[cfg(target_arch = "x86_64")]
const SYS_FUTEX: c_long = 202;
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "riscv64",
    target_arch = "loongarch64"
))]
const SYS_FUTEX: c_long = 98;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

unsafe extern "C" {
    fn syscall(num: c_long, ...) -> c_long;

    pub fn _exit(status: c_int) -> !;
    pub fn fork() -> c_int;
    pub fn mmap(
        addr: *mut c_void,
        len: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    pub fn munmap(addr: *mut c_void, len: usize) -> c_int;
    pub fn usleep(usec: c_uint) -> c_int;
    pub fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
}

fn last_errno() -> i32 {
    io::Error::last_os_error().raw_os_error().unwrap_or(-1)
}

fn op_with_private(op: c_int, private: bool) -> c_int {
    if private { op | FUTEX_PRIVATE_FLAG } else { op }
}

pub fn futex_cmp_requeue(
    source: &AtomicU32,
    target: &AtomicU32,
    wake_count: u32,
    requeue_count: u32,
    cmp: u32,
    private: bool,
) -> Result<i64, i32> {
    let op = op_with_private(FUTEX_CMP_REQUEUE, private);
    let rc = unsafe {
        syscall(
            SYS_FUTEX,
            source.as_ptr(),
            op,
            wake_count,
            requeue_count,
            target.as_ptr(),
            cmp,
        )
    };

    if rc == -1 {
        Err(last_errno())
    } else {
        Ok(rc as i64)
    }
}

pub fn futex_wait(
    word: &AtomicU32,
    expected: u32,
    timeout: Option<&Timespec>,
    private: bool,
) -> Result<i64, i32> {
    let op = op_with_private(FUTEX_WAIT, private);
    let timeout_ptr = timeout.map_or(ptr::null::<Timespec>(), |timeout| timeout);
    let rc = unsafe { syscall(SYS_FUTEX, word.as_ptr(), op, expected, timeout_ptr) };

    if rc == -1 {
        Err(last_errno())
    } else {
        Ok(rc as i64)
    }
}

pub fn futex_wait_bitset(
    word: &AtomicU32,
    expected: u32,
    timeout: &Timespec,
    bitset: u32,
    private: bool,
) -> Result<i64, i32> {
    let op = op_with_private(FUTEX_WAIT_BITSET, private);
    let rc = unsafe {
        syscall(
            SYS_FUTEX,
            word.as_ptr(),
            op,
            expected,
            timeout as *const Timespec,
            ptr::null::<u32>(),
            bitset,
        )
    };

    if rc == -1 {
        Err(last_errno())
    } else {
        Ok(rc as i64)
    }
}

pub fn futex_wake(word: &AtomicU32, count: u32, private: bool) -> Result<i64, i32> {
    let op = op_with_private(FUTEX_WAKE, private);
    let rc = unsafe { syscall(SYS_FUTEX, word.as_ptr(), op, count) };

    if rc == -1 {
        Err(last_errno())
    } else {
        Ok(rc as i64)
    }
}
