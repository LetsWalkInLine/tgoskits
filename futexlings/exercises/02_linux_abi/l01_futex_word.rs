use std::sync::atomic::{AtomicU32, Ordering};

struct FutexWord {
    word: AtomicU32,
}

impl FutexWord {
    fn new(value: u32) -> Self {
        Self {
            word: AtomicU32::new(value),
        }
    }

    fn load_user_space(&self) -> u32 {
        self.word.load(Ordering::SeqCst)
    }

    fn store_user_space(&self, value: u32) {
        self.word.store(value, Ordering::SeqCst);
    }

    fn slow_path_address(&self) -> *const u32 {
        // TODO: Return the address of the 32-bit futex word that would be
        // passed to the kernel on a slow-path futex operation.
        todo!("return the user-memory address of the futex word")
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn futex_word_is_user_memory() {
        let futex = FutexWord::new(3);
        assert_eq!(futex.load_user_space(), 3);

        futex.store_user_space(9);
        assert_eq!(futex.load_user_space(), 9);
        assert!(!futex.slow_path_address().is_null());
    }
}
