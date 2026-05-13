#[derive(Clone, Copy)]
struct ClockSnapshot {
    monotonic_ms: u64,
    realtime_ms: u64,
}

#[derive(Clone, Copy)]
enum ClockId {
    Monotonic,
    Realtime,
}

#[derive(Clone, Copy)]
enum Timeout {
    RelativeMs(u64),
    AbsoluteMs(u64),
}

fn deadline_ms(clock: ClockSnapshot, clock_id: ClockId, timeout: Timeout) -> u64 {
    // TODO: Relative timeouts add to the selected clock's current value.
    // Absolute timeouts are already deadlines on that clock.
    todo!("compute the timeout deadline")
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_timeout_uses_selected_clock_now() {
        let clock = ClockSnapshot {
            monotonic_ms: 1_000,
            realtime_ms: 10_000,
        };

        assert_eq!(
            deadline_ms(clock, ClockId::Monotonic, Timeout::RelativeMs(50)),
            1_050
        );
        assert_eq!(
            deadline_ms(clock, ClockId::Realtime, Timeout::RelativeMs(50)),
            10_050
        );
    }

    #[test]
    fn absolute_timeout_is_already_deadline() {
        let clock = ClockSnapshot {
            monotonic_ms: 1_000,
            realtime_ms: 10_000,
        };

        assert_eq!(
            deadline_ms(clock, ClockId::Realtime, Timeout::AbsoluteMs(12_000)),
            12_000
        );
    }
}
