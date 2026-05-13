use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq)]
enum WaitError {
    WouldBlock,
}

#[derive(Debug, Eq, PartialEq)]
struct Waiter {
    key: u64,
    id: u32,
}

#[derive(Default)]
struct Bucket {
    waiters: VecDeque<Waiter>,
}

fn atomic_wait(
    bucket: &mut Bucket,
    key: u64,
    user_value_after_lock: u32,
    expected: u32,
    id: u32,
) -> Result<(), WaitError> {
    // TODO: Re-read the user value after taking the bucket lock. Only enqueue
    // when it still equals `expected`.
    todo!("preserve the atomic wait invariant")
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changed_value_does_not_enqueue() {
        let mut bucket = Bucket::default();

        assert_eq!(
            atomic_wait(&mut bucket, 7, 1, 0, 10),
            Err(WaitError::WouldBlock)
        );
        assert!(bucket.waiters.is_empty());
    }

    #[test]
    fn matching_value_enqueues() {
        let mut bucket = Bucket::default();

        assert_eq!(atomic_wait(&mut bucket, 7, 0, 0, 10), Ok(()));
        assert_eq!(bucket.waiters.pop_front(), Some(Waiter { key: 7, id: 10 }));
    }
}
