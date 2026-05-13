use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq)]
struct RequeueReport {
    lock_order: Vec<usize>,
    woken: Vec<u32>,
    moved: Vec<u32>,
}

struct FutexTable {
    buckets: Vec<VecDeque<u32>>,
}

impl FutexTable {
    fn new(bucket_count: usize) -> Self {
        let buckets = (0..bucket_count).map(|_| VecDeque::new()).collect();
        Self { buckets }
    }

    fn bucket_index(&self, key: u64) -> usize {
        key as usize % self.buckets.len()
    }

    fn lock_order_for(&self, from_key: u64, to_key: u64) -> Vec<usize> {
        let from = self.bucket_index(from_key);
        let to = self.bucket_index(to_key);

        if from == to {
            vec![from]
        } else if from < to {
            vec![from, to]
        } else {
            vec![to, from]
        }
    }

    fn push_waiter(&mut self, key: u64, id: u32) {
        let bucket = self.bucket_index(key);
        self.buckets[bucket].push_back(id);
    }

    fn requeue(
        &mut self,
        from_key: u64,
        to_key: u64,
        wake_count: usize,
        move_count: usize,
    ) -> RequeueReport {
        let lock_order = self.lock_order_for(from_key, to_key);
        let from = self.bucket_index(from_key);
        let to = self.bucket_index(to_key);

        let mut woken = Vec::new();
        for _ in 0..wake_count {
            if let Some(id) = self.buckets[from].pop_front() {
                woken.push(id);
            }
        }

        let mut moved = Vec::new();
        for _ in 0..move_count {
            if let Some(id) = self.buckets[from].pop_front() {
                self.buckets[to].push_back(id);
                moved.push(id);
            }
        }

        RequeueReport {
            lock_order,
            woken,
            moved,
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lock_order_is_sorted_and_deduplicated() {
        let table = FutexTable::new(8);
        assert_eq!(table.lock_order_for(7, 2), vec![2, 7]);
        assert_eq!(table.lock_order_for(10, 18), vec![2]);
    }

    #[test]
    fn wakes_then_moves_waiters() {
        let mut table = FutexTable::new(8);
        table.push_waiter(7, 1);
        table.push_waiter(7, 2);
        table.push_waiter(7, 3);

        let report = table.requeue(7, 2, 1, 2);
        assert_eq!(
            report,
            RequeueReport {
                lock_order: vec![2, 7],
                woken: vec![1],
                moved: vec![2, 3],
            }
        );
        assert_eq!(table.buckets[2], VecDeque::from([2, 3]));
    }
}
