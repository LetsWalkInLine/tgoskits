fn lost_wakeup_invariant() -> &'static str {
    "The kernel must recheck the user futex word while holding the bucket lock before enqueueing; otherwise a wake can race between the user check and queue insertion, causing a lost wakeup."
}

fn private_key_scope() -> &'static str {
    "A private futex key is scoped by the process memory manager, often described as the mm identity, plus the aligned user address."
}

fn robust_list_purpose() -> &'static str {
    "Robust lists let the kernel find futex words owned by an exiting task, mark owner died state, and wake a waiter so user space can repair the mutex."
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn lower(text: &str) -> String {
        text.to_ascii_lowercase()
    }

    #[test]
    fn mentions_lost_wakeup_and_bucket_lock() {
        let answer = lower(lost_wakeup_invariant());
        assert!(answer.contains("lost wakeup"));
        assert!(answer.contains("bucket"));
        assert!(answer.contains("recheck"));
    }

    #[test]
    fn mentions_private_memory_manager_scope() {
        let answer = lower(private_key_scope());
        assert!(answer.contains("memory manager") || answer.contains("mm"));
    }

    #[test]
    fn mentions_owner_death_repair() {
        let answer = lower(robust_list_purpose());
        assert!(answer.contains("owner"));
        assert!(answer.contains("death") || answer.contains("died"));
        assert!(answer.contains("wake"));
    }
}
