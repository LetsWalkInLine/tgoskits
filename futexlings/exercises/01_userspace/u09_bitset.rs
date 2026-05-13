use std::collections::VecDeque;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Waiter {
    id: u32,
    mask: u32,
}

#[derive(Default)]
struct BitsetQueue {
    waiters: VecDeque<Waiter>,
}

impl BitsetQueue {
    fn push(&mut self, id: u32, mask: u32) {
        self.waiters.push_back(Waiter { id, mask });
    }

    fn wake_bitset(&mut self, max: usize, wake_mask: u32) -> Vec<u32> {
        // TODO: Wake only waiters whose mask intersects `wake_mask`.
        todo!("remove up to max matching waiters and return their ids")
    }

    fn remaining_ids(&self) -> Vec<u32> {
        self.waiters.iter().map(|waiter| waiter.id).collect()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wakes_only_matching_bitsets() {
        let mut queue = BitsetQueue::default();
        queue.push(1, 0b0001);
        queue.push(2, 0b0010);
        queue.push(3, 0b0100);
        queue.push(4, 0b0011);

        assert_eq!(queue.wake_bitset(2, 0b0010), vec![2, 4]);
        assert_eq!(queue.remaining_ids(), vec![1, 3]);
    }
}
