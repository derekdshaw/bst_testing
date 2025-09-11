use crate::node::Node;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct BST<ValType> 
where ValType: std::fmt::Display + Ord + Clone,
{
    nodes: Vec<Node<ValType>>,
    root: Option<usize>
}

impl<ValType: std::fmt::Display + std::cmp::PartialOrd + Ord + Clone> BST<ValType> {
    pub fn new() -> Self {
        BST { nodes: vec![], root: None }
    }

    pub fn insert(&mut self, value: ValType) {
        let index = self.nodes.len();
        //let new_value = value.clone();
        self.nodes.push(Node::new(&value)); // now the new node is in the vector at index
        if let Some(root_index) = self.root {
            let mut cur = root_index;
            loop {
                let node = &mut self.nodes[cur];
                if value < node.value {
                    if let Some(l) = node.left { cur = l; continue }
                    node.left = Some(index);
                    break;
                } else {
                    if let Some(r) = node.right { cur = r; continue }
                    node.right = Some(index);
                    break;
                }

            }
        } else {
            self.root = Some(index);
        }
    }

    pub fn find(&self, value: &ValType) -> bool {
        if let Some(root_index) = self.root {
            self.find_internal(root_index, value)
        } else {
            false
        }
    }

    fn find_internal(&self, index: usize, value: &ValType) -> bool {
        let node = &self.nodes[index];
        match value.cmp(&node.value) {
            Ordering::Less => {
                if let Some(left) = node.left {
                    self.find_internal(left, value)
                } else {
                    false
                }
            },
            Ordering::Greater => {
                if let Some(right) = node.right {
                    self.find_internal(right, value)
                } else {
                    false
                }
            },
            Ordering::Equal => true
        }
    }

    pub fn delete(&mut self, value: ValType) {
        if let Some(root_index) = self.root {
            self.root = self.delete_internal(Some(root_index), &value);
        }
    }

    fn delete_internal(&mut self, current: Option<usize>, value: &ValType) -> Option<usize> {
        match current {
            Some(index) => {
                let node_value = self.nodes[index].value.clone();
                match value.cmp(&node_value) {
                    Ordering::Less => {
                        let left_child = self.nodes[index].left;
                        let new_left = self.delete_internal(left_child, value);
                        self.nodes[index].left = new_left;
                        Some(index)
                    },
                    Ordering::Greater => {
                        let right_child = self.nodes[index].right;
                        let new_right = self.delete_internal(right_child, value);
                        self.nodes[index].right = new_right;
                        Some(index)
                    },
                    Ordering::Equal => {
                        let left_child = self.nodes[index].left;
                        let right_child = self.nodes[index].right;
                        
                        match (left_child, right_child) {
                            // No children
                            (None, None) => None,
                            // Only right child
                            (None, Some(right)) => Some(right),
                            // Only left child
                            (Some(left), None) => Some(left),
                            // Two children - find inorder successor (minimum in right subtree)
                            (Some(_left), Some(right)) => {
                                let successor_index = self.find_min(right);
                                let successor_value = self.nodes[successor_index].value.clone();
                                
                                // Replace current node's value with successor's value
                                self.nodes[index].value = successor_value.clone();
                                
                                // Delete the successor node from right subtree
                                let new_right = self.delete_internal(Some(right), &successor_value);
                                self.nodes[index].right = new_right;
                                
                                Some(index)
                            }
                        }
                    }
                }
            },
            None => None
        }
    }

    fn find_min(&self, mut index: usize) -> usize {
        while let Some(left) = self.nodes[index].left {
            index = left;
        }
        index
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