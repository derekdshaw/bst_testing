use crate::node::Node;
use std::rc::Rc;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct BTree<ValType> 
where ValType: std::fmt::Display + std::cmp::PartialOrd + Ord + Clone,
{
    root: Option<Rc<Node<ValType>>>
}

impl<ValType: std::fmt::Display + std::cmp::PartialOrd + Ord + Clone> BTree<ValType> {
    pub fn new() -> Self {
        BTree { root: None }
    }

    pub fn insert(&mut self, value: ValType) {
        let new_node = Rc::new(Node::new(value));
        self.root = Self::insert_internal(&self.root, new_node);
    }

    fn insert_internal(current: &Option<Rc<Node<ValType>>>, new_node: Rc<Node<ValType>>) -> Option<Rc<Node<ValType>>> {   
        if let Some(cur) = current {
            match new_node.value.cmp(&cur.value) {
                Ordering::Less => {
                    let next_node = Self::insert_internal(&cur.left.borrow(), new_node);
                    *cur.left.borrow_mut() = next_node;
                    Some(Rc::clone(cur))
                },
                Ordering::Greater => {
                    let next_node = Self::insert_internal(&cur.right.borrow(), new_node);
                    *cur.right.borrow_mut() = next_node;
                    Some(Rc::clone(cur))
                },
                Ordering::Equal => Some(Rc::clone(cur))
            }
        } else {
            Some(new_node)
        }
    }

    pub fn find(&self, value: &ValType) -> bool {
        Self::find_internal(&self.root, value)
    }

    fn find_internal(current: &Option<Rc<Node<ValType>>>, value: &ValType) -> bool {
        if let Some(cur) = current {
            match value.cmp(&cur.value) {
                Ordering::Less => Self::find_internal(&cur.left.borrow(), value),
                Ordering::Greater => Self::find_internal(&cur.right.borrow(), value),
                Ordering::Equal => true
            }
        } else {
            false
        }
    }

    pub fn delete(&mut self, value: ValType) {
        self.root = Self::delete_internal(&self.root, value);
    }

    fn delete_internal(current: &Option<Rc<Node<ValType>>>, value: ValType) -> Option<Rc<Node<ValType>>> {
        if let Some(cur) = current {
            match value.cmp(&cur.value) {
                Ordering::Less => {
                    let new_left = Self::delete_internal(&cur.left.borrow(), value);
                    *cur.left.borrow_mut() = new_left;
                    Some(Rc::clone(cur))
                },
                Ordering::Greater => {
                    let new_right = Self::delete_internal(&cur.right.borrow(), value);
                    *cur.right.borrow_mut() = new_right;
                    Some(Rc::clone(cur))
                },
                Ordering::Equal => {
                    // Case 1: No children
                    if cur.left.borrow().is_none() && cur.right.borrow().is_none() {
                        None
                    }
                    // Case 2: Only right child
                    else if cur.left.borrow().is_none() {
                        cur.right.borrow().clone()
                    }
                    // Case 3: Only left child
                    else if cur.right.borrow().is_none() {
                        cur.left.borrow().clone()
                    }
                    // Case 4: Two children
                    else {
                        if let Some(successor) = Self::find_min_node(&cur.right.borrow()) {
                            let new_node = Node::new(successor.value.clone());
                            *new_node.left.borrow_mut() = cur.left.borrow().clone();
                            *new_node.right.borrow_mut() = Self::delete_internal(&cur.right.borrow(), successor.value.clone());
                            Some(Rc::new(new_node))
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

    fn find_min_node(node: &Option<Rc<Node<ValType>>>) -> Option<Rc<Node<ValType>>> {
        if let Some(n) = node {
            if n.left.borrow().is_none() {
                Some(Rc::clone(n))
            } else {
                Self::find_min_node(&n.left.borrow())
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::time::Instant;
    use rand::Rng;

    fn make_small_tree() -> BTree<i32> {
        let mut btree = BTree::new();
        btree.insert(5);
        btree.insert(3);
        btree.insert(7);
        btree
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
        let mut btree = BTree::new();
        btree.insert(5);
        assert!(btree.find(&5));
    }

    #[test]
    fn test_insert_duplicate() {
        let mut btree = BTree::new();
        btree.insert(5);
        btree.insert(5);
        assert!(btree.find(&5));
    }

    #[test]
    fn test_insert_multiple() {
        let btree = make_small_tree();
        assert!(btree.find(&5));
        assert!(btree.find(&3));
        assert!(btree.find(&7));
    }

    #[test]
    fn test_find() {
        let btree = make_small_tree();
        assert!(!btree.find(&4));
        assert!(btree.find(&5));
    }

    #[test]
    fn test_delete_single_node() {
        let mut btree = BTree::new();
        btree.insert(5);
        btree.delete(5);
        assert!(!btree.find(&5));
    }

    #[test]
    fn test_delete_root() {
        let mut btree = make_small_tree();
        btree.delete(5);
        assert!(!btree.find(&5));
        assert!(btree.find(&3));
        assert!(btree.find(&7));
    }

    #[test]
    fn test_build_large_tree() {
        let (data, _) = make_large_data();
        let mut btree = BTree::new();
        let start = Instant::now();
        for &item in &data {
            btree.insert(item);
        }
        let duration = start.elapsed();
        println!("Time taken to build large tree: {:?}", duration);
    }

    #[test]
    fn test_delete_from_large_tree() {
        let (data, target) = make_large_data();
        let mut btree = BTree::new();
        for &item in &data {
            btree.insert(item);
        }
        let start = Instant::now();
        btree.delete(target);
        let duration = start.elapsed();
        println!("Time taken to delete from large tree: {:?}", duration);
        assert!(!btree.find(&target));
    }
}