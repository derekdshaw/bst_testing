use crate::node::Node;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct BST<ValType>
where
    ValType: Ord + Clone,
{
    nodes: Vec<Node<ValType>>,
    root: Option<usize>,
}
impl<ValType: Ord + Clone> BST<ValType> {
    pub fn new() -> Self {
        BST {
            nodes: Vec::with_capacity(10000),
            root: None,
        }
    }

    fn alloc_node(&mut self, value: ValType) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(Node::new(value));
        idx
    }

    pub fn insert(&mut self, value: ValType) {
        if self.root.is_none() {
            let idx = self.alloc_node(value);
            self.root = Some(idx);
        } else {
            self.insert_non_empty(value);
        }
    }

    fn insert_non_empty(&mut self, value: ValType) {
        let mut current = self.root;
        let mut parent = None;
        let mut went_left = false;

        while let Some(idx) = current {
            parent = Some(idx);
            match value.cmp(&self.nodes[idx].value) {
                Ordering::Less => {
                    current = self.nodes[idx].left;
                    went_left = true;
                }
                Ordering::Greater => {
                    current = self.nodes[idx].right;
                    went_left = false;
                }
                Ordering::Equal => return,
            }
        }

        let new_idx = self.alloc_node(value);
        if let Some(parent_idx) = parent {
            let parent_node = &mut self.nodes[parent_idx];
            if went_left {
                parent_node.left = Some(new_idx);
            } else {
                parent_node.right = Some(new_idx);
            }
        }
    }

    pub fn find(&self, value: &ValType) -> bool {
        self.find_internal(&self.root, value)
    }
  
    fn find_internal(&self, current: &Option<usize>, value: &ValType) -> bool {
        if let Some(cur_idx) = current {
            let node = &self.nodes[*cur_idx];
            match value.cmp(&node.value) {
                Ordering::Less => self.find_internal(&node.left, value),
                Ordering::Greater => self.find_internal(&node.right, value),
                Ordering::Equal => true,
            }
        }
    }

    pub fn delete(&mut self, value: ValType) {
        self.root = self.delete_internal(self.root, &value);
    }

    fn delete_internal(&mut self, node_idx: Option<usize>, value: &ValType) -> Option<usize> {
        let idx = match node_idx {
            Some(i) => i,
            None => return None,
        };

        match value.cmp(&self.nodes[idx].value) {
            Ordering::Less => {
                let left = self.nodes[idx].left;
                self.nodes[idx].left = self.delete_internal(left, value);
                Some(idx)
            }
            Ordering::Greater => {
                let right = self.nodes[idx].right;
                self.nodes[idx].right = self.delete_internal(right, value);
                Some(idx)
            }
            Ordering::Equal => {
                let left = self.nodes[idx].left;
                let right = self.nodes[idx].right;

                match (left, right) {
                    (None, None) => None,
                    (Some(child), None) | (None, Some(child)) => Some(child),
                    (Some(_), Some(right_idx)) => {
                        let successor_idx = self.find_min_idx(right_idx);
                        let successor_value = self.nodes[successor_idx].value.clone();
                        self.nodes[idx].value = successor_value.clone();
                        let new_right = self.delete_internal(right, &successor_value);
                        self.nodes[idx].right = new_right;
                        Some(idx)
                    }
                }
            }
        }
    }

    fn find_min_idx(&self, mut idx: usize) -> usize {
        while let Some(next) = self.nodes[idx].left {
            idx = next;
        }
        idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::collections::HashSet;
    use std::time::Instant;

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
