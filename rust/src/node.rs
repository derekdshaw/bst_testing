#[derive(Debug)]
pub struct Node<ValType>
where
    ValType: Ord,
{
    pub value: ValType,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

impl<ValType: Ord> Node<ValType> {
    pub fn new(value: ValType) -> Node<ValType> {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let node = Node::new(&5);
        assert_eq!(node.value, 5);
        assert!(node.left.is_none());
        assert!(node.right.is_none());
    }

    #[test]
    fn test_linking() {
        let node2 = Node::new(3);
        let node3 = Node::new(7);

        // simulate the BST storage.
        let nodes = vec![node2, node3];

        let mut root = Node::new(5);
        root.left = Some(0);
        root.right = Some(1);

        assert_eq!(nodes[root.left.unwrap()].value, 3);
        assert_eq!(nodes[root.right.unwrap()].value, 7);
    }
}
