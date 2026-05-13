#[derive(Debug, Clone, Eq, PartialEq)]
enum FutexMapping {
    Private {
        mm_id: u64,
        virtual_addr: u64,
    },
    Shared {
        inode: u64,
        page_offset: u64,
        word_offset: u16,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum FutexKey {
    Private {
        mm_id: u64,
        aligned_addr: u64,
    },
    Shared {
        inode: u64,
        page_offset: u64,
        word_offset: u16,
    },
}

fn key_for(mapping: FutexMapping) -> FutexKey {
    match mapping {
        FutexMapping::Private {
            mm_id,
            virtual_addr,
        } => FutexKey::Private {
            mm_id,
            aligned_addr: virtual_addr & !0b11,
        },
        FutexMapping::Shared {
            inode,
            page_offset,
            word_offset,
        } => FutexKey::Shared {
            inode,
            page_offset,
            word_offset,
        },
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_key_includes_mm_identity() {
        let a = key_for(FutexMapping::Private {
            mm_id: 1,
            virtual_addr: 0x1003,
        });
        let b = key_for(FutexMapping::Private {
            mm_id: 2,
            virtual_addr: 0x1003,
        });

        assert_ne!(a, b);
        assert_eq!(
            a,
            FutexKey::Private {
                mm_id: 1,
                aligned_addr: 0x1000,
            }
        );
    }

    #[test]
    fn shared_key_uses_backing_object_identity() {
        let a = key_for(FutexMapping::Shared {
            inode: 9,
            page_offset: 12,
            word_offset: 4,
        });
        let b = key_for(FutexMapping::Shared {
            inode: 9,
            page_offset: 12,
            word_offset: 4,
        });

        assert_eq!(a, b);
    }
}
