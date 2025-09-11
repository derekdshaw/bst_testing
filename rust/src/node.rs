#[derive(Debug, Clone)]
pub struct Node<ValType> 
where ValType: std::fmt::Display + Ord + Clone,
{
    pub value: ValType,
    pub left: Option<Box<Node<ValType>>>,
    pub right: Option<Box<Node<ValType>>>,
}

impl<ValType: std::fmt::Display + Ord + Clone> Node<ValType> 
{
    pub fn new(value: &ValType) -> Node<ValType> {
        Node { 
            value: value.clone(), 
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

        // The BST would have a Vec<Node<T>> so simulate that here
        let mut nodes = vec![];
        
        let root = Node::new(&5);
        nodes.push(root);
        let root_index = 0; // The root is at index 0
        
        // Now work with the node through the vector
        nodes[root_index].left = Some(Box::new(Node::new(&3)));
        nodes[root_index].right = Some(Box::new(Node::new(&7)));
        
        assert_eq!(nodes[root_index].left.as_ref().unwrap().value, 3);
        assert_eq!(nodes[root_index].right.as_ref().unwrap().value, 7);
    }
}