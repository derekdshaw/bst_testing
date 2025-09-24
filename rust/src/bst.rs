use crate::node::Node;
use std::boxed::Box;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct BST<ValType> 
where ValType: Ord,
{
    root: Option<Box<Node<ValType>>>
}
impl<ValType: Ord> BST<ValType> {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn insert(&mut self, value: ValType) {
        Self::insert_internal(&mut self.root, value);
    }

    fn insert_internal(node: &mut Option<Box<Node<ValType>>>, value: ValType) {   
        match node {
            Some(cur) => {
                match value.cmp(&cur.value) {
                    Ordering::Less => {
                        Self::insert_internal(&mut cur.left, value);
                    },
                    Ordering::Greater => {
                        Self::insert_internal(&mut cur.right, value);
                    },
                    Ordering::Equal => {} // Duplicate value, do nothing
                }
            },
            None => {
                *node = Some(Box::new(Node::new(value)));
            }
        }
    }

    pub fn find(&self, value: &ValType) -> bool {
        Self::find_internal(&self.root, value)
    }
    fn find_internal(current: &Option<Box<Node<ValType>>>, value: &ValType) -> bool {
        if let Some(cur) = current {
            match value.cmp(&cur.value) {
                Ordering::Less => Self::find_internal(&cur.left, value),
                Ordering::Greater => Self::find_internal(&cur.right, value),
                Ordering::Equal => true
            }
        } else {
            false
        }
    }

    pub fn delete(&mut self, value: ValType) {
        self.root = Self::delete_internal(&mut self.root, value);
    }

    fn delete_internal(node: &mut Option<Box<Node<ValType>>>, value: ValType) -> Option<Box<Node<ValType>>> {
        if let Some(cur) = node {
            match value.cmp(&cur.value) {
                Ordering::Less => {
                    let new_left = Self::delete_internal(&mut cur.left, value);
                    cur.left = new_left;
                    Some(node.take().unwrap())
                },
                Ordering::Greater => {
                    let new_right = Self::delete_internal(&mut cur.right, value);
                    cur.right = new_right;
                    Some(node.take().unwrap())
                },
                Ordering::Equal => {
                    // Case 1: No children
                    if cur.left.is_none() && cur.right.is_none() {
                        None
                    }
                    // Case 2: Only right child
                    else if cur.left.is_none() {
                        Some(cur.right.take().unwrap())
                    }
                    // Case 3: Only left child
                    else if cur.right.is_none() {
                        Some(cur.left.take().unwrap())
                    }
                    // Case 4: Two children
                    else {
                        if let Some(successor) = Self::find_min_node(&mut cur.right) {
                            std::mem::swap(&mut cur.value, &mut successor.value);
                            Self::delete_internal(&mut cur.right, value)
                            // let value = successor.value;
                            // let mut new_node = Box::new(Node::new(value.clone()));
                            // new_node.left = Some(cur.left.take().unwrap());
                            // new_node.right = Self::delete_internal(&mut cur.right, value);
                            // Some(new_node)
                        } else {
                            None
                        }
                    }
                }
            }
        } else {
            None
        }
    }

    fn find_min_node(node: &mut Option<Box<Node<ValType>>>) -> Option<&mut Box<Node<ValType>>> {
        match node {
            Some(n) => {
                if n.left.is_none() {
                    Some(n)
                } else {
                    Self::find_min_node(&mut n.left)
                }
            },
            None => None
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