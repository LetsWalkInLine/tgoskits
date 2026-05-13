fn lost_wakeup_invariant() -> &'static str {
    // TODO: Explain why the kernel rechecks the futex word while holding the
    // bucket lock before enqueueing the waiter.
    todo!("answer the lost wakeup invariant question")
}

fn private_key_scope() -> &'static str {
    // TODO: Explain what extra identity is part of a private futex key.
    todo!("answer the private key scope question")
}

fn robust_list_purpose() -> &'static str {
    // TODO: Explain what robust lists let the kernel do when a task exits.
    todo!("answer the robust list question")
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
