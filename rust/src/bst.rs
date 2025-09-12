use crate::node::Node;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct BST<ValType> 
where ValType: std::fmt::Display + Ord + Clone,
{
    root: Option<Box<Node<ValType>>>,
}

impl<ValType: std::fmt::Display + std::cmp::PartialOrd + Ord + Clone> BST<ValType> {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn insert(&mut self, value: ValType) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node::new(&value)));
            return;
        }

        let mut current = &mut self.root;
        
        loop {
            match current {
                Some(node) => {
                    match value.cmp(&node.value) {
                        Ordering::Less => {
                            if node.left.is_none() {
                                node.left = Some(Box::new(Node::new(&value)));
                                break;
                            } else {
                                current = &mut node.left;
                            }
                        },
                        Ordering::Greater => {
                            if node.right.is_none() {
                                node.right = Some(Box::new(Node::new(&value)));
                                break;
                            } else {
                                current = &mut node.right;
                            }
                        },
                        Ordering::Equal => {
                            // For duplicates, insert to the right for consistency
                            if node.right.is_none() {
                                node.right = Some(Box::new(Node::new(&value)));
                                break;
                            } else {
                                current = &mut node.right;
                            }
                        }
                    }
                },
                None => unreachable!("Should never reach here due to initial check"),
            }
        }
    }

    pub fn find(&self, value: &ValType) -> bool {
        Self::find_recursive(&self.root, value)
    }

    fn find_recursive(node: &Option<Box<Node<ValType>>>, value: &ValType) -> bool {
        match node {
            None => false,
            Some(node) => {
                match value.cmp(&node.value) {
                    Ordering::Equal => true,
                    Ordering::Less => Self::find_recursive(&node.left, value),
                    Ordering::Greater => Self::find_recursive(&node.right, value),
                }
            }
        }
    }

    pub fn delete(&mut self, value: ValType) {
        self.root = Self::delete_recursive(self.root.take(), &value);
    }

    fn delete_recursive(node: Option<Box<Node<ValType>>>, value: &ValType) -> Option<Box<Node<ValType>>> {
        match node {
            None => None,
            Some(mut node) => {
                match value.cmp(&node.value) {
                    Ordering::Less => {
                        node.left = Self::delete_recursive(node.left.take(), value);
                        Some(node)
                    },
                    Ordering::Greater => {
                        node.right = Self::delete_recursive(node.right.take(), value);
                        Some(node)
                    },
                    Ordering::Equal => {
                        // Node to delete found
                        match (node.left.take(), node.right.take()) {
                            // No children
                            (None, None) => None,
                            // Only right child
                            (None, Some(right)) => Some(right),
                            // Only left child
                            (Some(left), None) => Some(left),
                            // Two children - find inorder successor
                            (Some(left), Some(right)) => {
                                let (min_value, new_right) = Self::extract_min(right);
                                node.value = min_value;
                                node.left = Some(left);
                                node.right = new_right;
                                Some(node)
                            }
                        }
                    }
                }
            }
        }
    }

    fn extract_min(mut node: Box<Node<ValType>>) -> (ValType, Option<Box<Node<ValType>>>) {
        match node.left.take() {
            None => {
                // This is the minimum node
                (node.value, node.right.take())
            },
            Some(left) => {
                let (min_value, new_left) = Self::extract_min(left);
                node.left = new_left;
                (min_value, Some(node))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::time::Instant;
    use rand::Rng;

    fn make_small_tree() -> BST<i32> {
        let mut bst = BST::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst
    }

    fn make_large_data() -> (HashSet<i32>, i32) {
        let mut rng = rand::thread_rng();
        let mut data = HashSet::new();
        
        while data.len() < 1_000_000 {
            data.insert(rng.gen_range(1..2_000_000));
        }
        
        let target = rng.gen_range(1..2_000_000);
        (data, target)
    }

    #[test]
    fn test_insert_one() {
        let mut bst = BST::new();
        bst.insert(5);
        assert!(bst.find(&5));
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BST::new();
        bst.insert(5);
        bst.insert(5);
        assert!(bst.find(&5));
    }

    #[test]
    fn test_insert_multiple() {
        let bst = make_small_tree();
        assert!(bst.find(&5));
        assert!(bst.find(&3));
        assert!(bst.find(&7));
    }

    #[test]
    fn test_find() {
        let bst = make_small_tree();
        assert!(!bst.find(&4));
        assert!(bst.find(&5));
    }

    #[test]
    fn test_delete_single_node() {
        let mut bst = BST::new();
        bst.insert(5);
        bst.delete(5);
        assert!(!bst.find(&5));
    }

    #[test]
    fn test_delete_root() {
        let mut bst = make_small_tree();
        bst.delete(5);
        assert!(!bst.find(&5));
        assert!(bst.find(&3));
        assert!(bst.find(&7));
    }

    #[test]
    fn test_build_large_tree() {
        let (data, _) = make_large_data();
        let mut bst = BST::new();
        let start = Instant::now();
        for &item in &data {
            bst.insert(item);
        }
        let duration = start.elapsed();
        println!("Time taken to build large tree: {:?}", duration);
    }

    #[test]
    fn test_delete_from_large_tree() {
        let (data, target) = make_large_data();
        let mut bst = BST::new();
        for &item in &data {
            bst.insert(item);
        }
        let start = Instant::now();
        bst.delete(target);
        let duration = start.elapsed();
        println!("Time taken to delete from large tree: {:?}", duration);
        assert!(!bst.find(&target));
    }
}