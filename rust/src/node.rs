use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Node<ValType> 
where ValType: std::fmt::Display + std::cmp::PartialOrd + Ord + Clone,
{
    pub value: ValType,
    pub left: RefCell<Option<Rc<Node<ValType>>>>,
    pub right: RefCell<Option<Rc<Node<ValType>>>>, 
}

impl<ValType: std::fmt::Display + std::cmp::PartialOrd + Ord + Clone> Node<ValType> 
{
    pub fn new(value: ValType) -> Node<ValType> {
        Node { 
            value, 
            left: RefCell::new(None),
            right: RefCell::new(None),
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
        assert!(node.left.borrow().is_none());
        assert!(node.right.borrow().is_none());
    }

    #[test]
    fn test_linking() {
        let node2 = Rc::new(Node::new(3));
        let node3 = Rc::new(Node::new(7));
        
        let root = Node::new(5);
        *root.left.borrow_mut() = Some(node2);
        *root.right.borrow_mut() = Some(node3);
        
        assert_eq!(root.left.borrow().as_ref().unwrap().value, 3);
        assert_eq!(root.right.borrow().as_ref().unwrap().value, 7);
    }
}