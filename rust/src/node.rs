#[derive(Debug, Clone)]
pub struct Node<ValType> 
where ValType: std::fmt::Display + Ord + Clone,
{
    pub value: ValType,
    pub left: Option<usize>, // these will be indexes into the main BST built of nodes.
    pub right: Option<usize>,
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
        nodes[root_index].left = Some(nodes.len());
        nodes.push(Node::new(&3));
        nodes[root_index].right = Some(nodes.len());
        nodes.push(Node::new(&7));
        
        assert_eq!(nodes[nodes[root_index].left.unwrap()].value, 3);
        assert_eq!(nodes[nodes[root_index].right.unwrap()].value, 7);
    }
}