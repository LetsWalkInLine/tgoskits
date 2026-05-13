use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq)]
struct RequeueReport {
    woken: Vec<u32>,
    requeued: Vec<u32>,
}

#[derive(Debug, Eq, PartialEq)]
enum RequeueError {
    CompareFailed,
}

struct Queues {
    word: u32,
    source: VecDeque<u32>,
    destination: VecDeque<u32>,
}

impl Queues {
    fn new(word: u32, source: impl IntoIterator<Item = u32>) -> Self {
        Self {
            word,
            source: source.into_iter().collect(),
            destination: VecDeque::new(),
        }
    }

    fn cmp_requeue(
        &mut self,
        expected: u32,
        wake_count: usize,
        requeue_count: usize,
    ) -> Result<RequeueReport, RequeueError> {
        // TODO: Compare `word`, wake from the source queue, then move more
        // source waiters to the destination queue.
        todo!("implement compare, wake, and requeue")
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_failure_does_not_move_waiters() {
        let mut queues = Queues::new(5, [1, 2, 3]);

        assert_eq!(
            queues.cmp_requeue(4, 1, 2),
            Err(RequeueError::CompareFailed)
        );
        assert_eq!(queues.source, VecDeque::from([1, 2, 3]));
        assert!(queues.destination.is_empty());
    }

    #[test]
    fn wakes_then_requeues_from_source() {
        let mut queues = Queues::new(5, [1, 2, 3, 4]);

        assert_eq!(
            queues.cmp_requeue(5, 1, 2).unwrap(),
            RequeueReport {
                woken: vec![1],
                requeued: vec![2, 3],
            }
        );
        assert_eq!(queues.source, VecDeque::from([4]));
        assert_eq!(queues.destination, VecDeque::from([2, 3]));
    }
}
