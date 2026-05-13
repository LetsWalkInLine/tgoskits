#[derive(Debug, Eq, PartialEq)]
enum RobustLockStatus {
    Locked,
    OwnerDied,
    NotRecoverable,
    OtherError(i32),
}

const EOWNERDEAD: i32 = 130;
const ENOTRECOVERABLE: i32 = 131;

fn classify_pthread_lock_result(rc: i32) -> RobustLockStatus {
    // TODO: Convert the pthread robust mutex lock return code into a surface
    // status the caller can handle.
    todo!("classify robust mutex lock return codes")
}

fn caller_must_mark_consistent(status: &RobustLockStatus) -> bool {
    matches!(status, RobustLockStatus::OwnerDied)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owner_death_is_recoverable_surface_state() {
        let status = classify_pthread_lock_result(EOWNERDEAD);

        assert_eq!(status, RobustLockStatus::OwnerDied);
        assert!(caller_must_mark_consistent(&status));
    }

    #[test]
    fn not_recoverable_is_distinct_from_owner_death() {
        assert_eq!(
            classify_pthread_lock_result(ENOTRECOVERABLE),
            RobustLockStatus::NotRecoverable
        );
        assert!(!caller_must_mark_consistent(
            &RobustLockStatus::NotRecoverable
        ));
    }
}
