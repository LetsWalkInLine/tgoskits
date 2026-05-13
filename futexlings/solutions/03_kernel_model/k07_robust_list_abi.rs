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
    let mut addresses = Vec::new();
    let mut current = head.head;

    while let Some(index) = current {
        if addresses.len() >= limit {
            break;
        }

        let node = &head.nodes[index];
        addresses.push(node.node_addr + head.futex_offset);
        current = node.next;
    }

    if let Some(pending) = head.list_op_pending
        && addresses.len() < limit
    {
        addresses.push(pending + head.futex_offset);
    }

    addresses
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
