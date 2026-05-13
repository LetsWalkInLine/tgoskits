#[derive(Clone, Copy)]
enum FutexKind {
    Normal,
    PriorityInheritance,
}

#[derive(Debug, Eq, PartialEq)]
struct KernelResponsibilities {
    blocks_waiters: bool,
    tracks_owner: bool,
    donates_priority: bool,
}

fn responsibilities(kind: FutexKind) -> KernelResponsibilities {
    match kind {
        FutexKind::Normal => KernelResponsibilities {
            blocks_waiters: true,
            tracks_owner: false,
            donates_priority: false,
        },
        FutexKind::PriorityInheritance => KernelResponsibilities {
            blocks_waiters: true,
            tracks_owner: true,
            donates_priority: true,
        },
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_futex_does_not_track_ownership() {
        assert_eq!(
            responsibilities(FutexKind::Normal),
            KernelResponsibilities {
                blocks_waiters: true,
                tracks_owner: false,
                donates_priority: false,
            }
        );
    }

    #[test]
    fn pi_futex_adds_owner_tracking_and_priority_donation() {
        assert_eq!(
            responsibilities(FutexKind::PriorityInheritance),
            KernelResponsibilities {
                blocks_waiters: true,
                tracks_owner: true,
                donates_priority: true,
            }
        );
    }
}
