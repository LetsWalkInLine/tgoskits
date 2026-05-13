#[derive(Debug, Eq, PartialEq)]
enum WaitDecision {
    Park,
    ValueChanged,
}

#[derive(Debug)]
struct WaitSlot {
    value: u32,
    queued: bool,
}

impl WaitSlot {
    fn new(value: u32) -> Self {
        Self {
            value,
            queued: false,
        }
    }
}

fn prepare_wait(slot: &mut WaitSlot, expected: u32) -> WaitDecision {
    // TODO: Re-check the value before queueing. Only queue if it still matches
    // `expected`; otherwise report that the value changed.
    todo!("re-check under the model lock before queueing the waiter")
}

fn wake(slot: &mut WaitSlot) -> bool {
    if slot.queued {
        slot.queued = false;
        true
    } else {
        false
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changed_value_does_not_park() {
        let mut slot = WaitSlot::new(1);

        assert_eq!(prepare_wait(&mut slot, 0), WaitDecision::ValueChanged);
        assert!(!slot.queued);
    }

    #[test]
    fn matching_value_queues_then_wakes() {
        let mut slot = WaitSlot::new(0);

        assert_eq!(prepare_wait(&mut slot, 0), WaitDecision::Park);
        assert!(slot.queued);
        assert!(wake(&mut slot));
        assert!(!slot.queued);
    }
}
