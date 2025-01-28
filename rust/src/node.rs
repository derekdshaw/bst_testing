use std::boxed::Box;

#[derive(Debug)]
pub struct Node<ValType> 
where ValType: std::fmt::Display + std::cmp::PartialOrd + Ord + Clone,
{
    pub value: ValType,
    pub left: Option<Box<Node<ValType>>>,
    pub right: Option<Box<Node<ValType>>>,
}

impl<ValType: std::fmt::Display + std::cmp::PartialOrd + Ord + Clone> Node<ValType> 
{
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
        let node = Node::new(5);
        assert_eq!(node.value, 5);
        assert!(node.left.is_none());
        assert!(node.right.is_none());
    }

    #[test]
    fn test_linking() {
        let node2 = Box::new(Node::new(3));
        let node3 = Box::new(Node::new(7));
        
        let mut root = Box::new(Node::new(5));
        root.left = Some(node2);
        root.right = Some(node3);
        
        assert_eq!(root.left.unwrap().value, 3);
        assert_eq!(root.right.unwrap().value, 7);
    }
}