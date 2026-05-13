const FUTEX_WAITERS: u32 = 0x8000_0000;
const FUTEX_OWNER_DIED: u32 = 0x4000_0000;
const FUTEX_TID_MASK: u32 = 0x3fff_ffff;

fn handle_futex_death(word: &mut u32, exiting_tid: u32) -> bool {
    if (*word & FUTEX_TID_MASK) != exiting_tid {
        return false;
    }

    *word = (*word & !FUTEX_TID_MASK) | FUTEX_OWNER_DIED;
    true
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marks_owner_died_and_preserves_waiters_bit() {
        let mut word = FUTEX_WAITERS | 123;

        assert!(handle_futex_death(&mut word, 123));
        assert_eq!(word & FUTEX_TID_MASK, 0);
        assert_ne!(word & FUTEX_OWNER_DIED, 0);
        assert_ne!(word & FUTEX_WAITERS, 0);
    }

    #[test]
    fn ignores_words_owned_by_someone_else() {
        let mut word = 456;
        assert!(!handle_futex_death(&mut word, 123));
        assert_eq!(word, 456);
    }
}
