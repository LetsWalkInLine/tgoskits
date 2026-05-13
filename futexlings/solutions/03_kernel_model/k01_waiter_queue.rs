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
        let bucket = self.bucket_index(key);
        self.buckets[bucket].push_back(Waiter { key, id });
    }

    fn wake(&mut self, key: u64, max: usize) -> Vec<u32> {
        let bucket = self.bucket_index(key);
        let mut woken = Vec::new();
        let mut remaining = VecDeque::new();

        while let Some(waiter) = self.buckets[bucket].pop_front() {
            if waiter.key == key && woken.len() < max {
                woken.push(waiter.id);
            } else {
                remaining.push_back(waiter);
            }
        }

        self.buckets[bucket] = remaining;
        woken
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
