use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq)]
struct Waiter {
    id: u32,
    mask: u32,
}

#[derive(Default)]
struct Bucket {
    waiters: VecDeque<Waiter>,
}

impl Bucket {
    fn push(&mut self, id: u32, mask: u32) {
        self.waiters.push_back(Waiter { id, mask });
    }

    fn wake_filtered(&mut self, max: usize, wake_mask: u32) -> Vec<u32> {
        let mut woken = Vec::new();
        let mut remaining = VecDeque::new();

        while let Some(waiter) = self.waiters.pop_front() {
            if woken.len() < max && (waiter.mask & wake_mask) != 0 {
                woken.push(waiter.id);
            } else {
                remaining.push_back(waiter);
            }
        }

        self.waiters = remaining;
        woken
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wake_count_and_bitset_filter_both_apply() {
        let mut bucket = Bucket::default();
        bucket.push(1, 0b0001);
        bucket.push(2, 0b0010);
        bucket.push(3, 0b0010);
        bucket.push(4, 0b0100);

        assert_eq!(bucket.wake_filtered(1, 0b0010), vec![2]);
        assert_eq!(bucket.wake_filtered(10, 0b0010), vec![3]);
        assert_eq!(bucket.wake_filtered(10, 0b0101), vec![1, 4]);
    }
}
