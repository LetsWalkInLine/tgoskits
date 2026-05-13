#[derive(Clone, Debug)]
struct RobustNode {
    node_addr: isize,
    next: Option<usize>,
}

struct RobustHead {
    futex_offset: isize,
    list_op_pending: Option<isize>,
    head: Option<usize>,
    nodes: Vec<RobustNode>,
}

fn collect_futex_addresses(head: &RobustHead, limit: usize) -> Vec<isize> {
    // TODO: Walk the robust list from `head.head`, adding `futex_offset` to
    // each node address. Also include `list_op_pending` when present.
    todo!("scan robust list futex addresses")
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_nodes_and_pending_operation() {
        let head = RobustHead {
            futex_offset: -8,
            list_op_pending: Some(3_000),
            head: Some(0),
            nodes: vec![
                RobustNode {
                    node_addr: 1_000,
                    next: Some(1),
                },
                RobustNode {
                    node_addr: 2_000,
                    next: None,
                },
            ],
        };

        assert_eq!(collect_futex_addresses(&head, 16), vec![992, 1_992, 2_992]);
    }
}
