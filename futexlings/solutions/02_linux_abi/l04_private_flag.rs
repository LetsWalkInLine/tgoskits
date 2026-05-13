const FUTEX_WAIT: i32 = 0;
const FUTEX_WAKE: i32 = 1;
const FUTEX_PRIVATE_FLAG: i32 = 128;

fn private_op(op: i32) -> i32 {
    op | FUTEX_PRIVATE_FLAG
}

fn is_private(op: i32) -> bool {
    (op & FUTEX_PRIVATE_FLAG) != 0
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_flag_is_orred_into_operation() {
        assert_eq!(private_op(FUTEX_WAIT), 128);
        assert_eq!(private_op(FUTEX_WAKE), 129);
        assert!(is_private(private_op(FUTEX_WAIT)));
        assert!(!is_private(FUTEX_WAIT));
    }
}
