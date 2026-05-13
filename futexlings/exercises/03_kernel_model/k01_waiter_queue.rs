use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq)]
struct Waiter {
    key: u64,
    id: u32,
}

struct FutexTable {
    buckets: Vec<VecDeque<Waiter>>,
}

impl FutexTable {
    fn new(bucket_count: usize) -> Self {
        let buckets = (0..bucket_count).map(|_| VecDeque::new()).collect();
        Self { buckets }
    }

    fn bucket_index(&self, key: u64) -> usize {
        key as usize % self.buckets.len()
    }

    fn wait(&mut self, key: u64, id: u32) {
        // TODO: Hash the key to a bucket and enqueue this waiter.
        todo!("enqueue a waiter in the key's bucket")
    }

    fn wake(&mut self, key: u64, max: usize) -> Vec<u32> {
        // TODO: Wake up to `max` waiters for this key in FIFO order.
        todo!("remove matching waiters from the bucket")
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wakes_fifo_for_one_key() {
        let mut table = FutexTable::new(4);
        table.wait(10, 1);
        table.wait(10, 2);
        table.wait(11, 3);

        assert_eq!(table.wake(10, 1), vec![1]);
        assert_eq!(table.wake(10, 4), vec![2]);
        assert_eq!(table.wake(11, 4), vec![3]);
    }
}
