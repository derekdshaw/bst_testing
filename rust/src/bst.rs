use crate::node::Node;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct BST<ValType> 
where ValType: std::fmt::Display + Ord + Clone,
{
    nodes: Vec<Node<ValType>>,
    root: Option<usize>,
}

impl<ValType: std::fmt::Display + std::cmp::PartialOrd + Ord + Clone> BST<ValType> {
    pub fn new() -> Self {
        BST { root: None, nodes: Vec::new() }
    }
    
    #[inline(never)]
    pub fn insert(&mut self, value: ValType) {
        if self.root.is_none() {
            self.root = Some(self.nodes.len());
            self.nodes.push(Node::new(&value));
            return;
        }

        let mut current = self.root;
        
        loop {
            match current {
                Some(node_idx) => {
                    let next_node_idx = self.nodes.len();
                    let node = &mut self.nodes[node_idx];
                    match value.cmp(&node.value) {
                        Ordering::Less => {
                            if node.left.is_none() {
                                node.left = Some(next_node_idx);
                                self.nodes.push(Node::new(&value));
                                break;
                            } else {
                                current = node.left;
                            }
                        },
                        Ordering::Greater => {
                            if node.right.is_none() {
                                node.right = Some(next_node_idx);
                                self.nodes.push(Node::new(&value));
                                break;
                            } else {
                                current = node.right;
                            }
                        },
                        Ordering::Equal => {
                            // For duplicates, insert to the right for consistency
                            if node.right.is_none() {
                                node.right = Some(next_node_idx);
                                self.nodes.push(Node::new(&value));
                                break;
                            } else {
                                current = node.right;
                            }
                        }
                    }
                },
                None => unreachable!("Should never reach here due to initial check"),
            }
        }
    }

    pub fn find(&self, value: &ValType) -> bool {
        self.find_recursive(&self.root, value)
    }

    fn find_recursive(&self, node: &Option<usize>, value: &ValType) -> bool {
        match node {
            None => false,
            Some(node) => {
                match value.cmp(&self.nodes[*node].value) {
                    Ordering::Equal => true,
                    Ordering::Less => self.find_recursive(&self.nodes[*node].left, value),
                    Ordering::Greater => self.find_recursive(&self.nodes[*node].right, value),
                }
            }
        }
    }

    pub fn delete(&mut self, value: ValType) {
        let root_idx = match self.root {
            Some(idx) => idx,
            None => return,
        };
        self.root = self.delete_recursive(Some(root_idx), &value);
    }

    fn delete_recursive(&mut self, node: Option<usize>, value: &ValType) -> Option<usize> {
        match node {
            None => None,
            Some(node_idx) => {
                // Store the node's children before we potentially modify them
                let left_child = self.nodes[node_idx].left;
                let right_child = self.nodes[node_idx].right;
                
                match value.cmp(&self.nodes[node_idx].value) {
                    Ordering::Less => {
                        let new_left = self.delete_recursive(left_child, value);
                        self.nodes[node_idx].left = new_left;
                        Some(node_idx)
                    },
                    Ordering::Greater => {
                        let new_right = self.delete_recursive(right_child, value);
                        self.nodes[node_idx].right = new_right;
                        Some(node_idx)
                    },
                    Ordering::Equal => {
                        // Node to delete found
                        match (left_child, right_child) {
                            // No children
                            (None, None) => None,
                            // Only right child
                            (None, Some(right)) => Some(right),
                            // Only left child
                            (Some(left), None) => Some(left),
                            // Two children - find inorder successor
                            (Some(left), Some(right)) => {
                                let (min_value, new_right) = self.extract_min(right);
                                self.nodes[node_idx].value = min_value;
                                self.nodes[node_idx].left = Some(left);
                                self.nodes[node_idx].right = new_right;
                                Some(node_idx)
                            }
                        }
                    }
                }
            }
        }
    }

    fn extract_min(&mut self, node_idx: usize) -> (ValType, Option<usize>) {
        let left_idx = self.nodes[node_idx].left;
        match left_idx {
            None => {
                // This is the minimum node - return its value and right child
                let value = self.nodes[node_idx].value.clone();
                let right_child = self.nodes[node_idx].right;
                (value, right_child)
            },
            Some(left_idx) => {
                // Recursively find minimum in left subtree
                let (min_value, new_left) = self.extract_min(left_idx);
                // Update the left child of current node
                self.nodes[node_idx].left = new_left;
                (min_value, Some(node_idx))
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