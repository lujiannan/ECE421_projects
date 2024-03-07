pub mod rbtree { // our public red black tree module, so we can publish crate, and use in main
use std::cell::RefCell; // interior mutability
use std::rc::Rc; // multiple references

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

type Tree = Rc<RefCell<TreeNode<u32>>>;
type RedBlackTree = Option<Tree>; // just wrapping the whole thing in an option

#[derive(Clone, Debug, PartialEq)]
pub struct TreeNode<T> {
    color: NodeColor,
    key: T,
    parent: RedBlackTree,
    left: RedBlackTree,
    right: RedBlackTree,
}

impl TreeNode<u32> {
    pub fn new(key: u32) -> Tree { // create a new node
        Rc::new(RefCell::new(TreeNode {
            color: NodeColor::Red, // New nodes are always red
            key,
            parent: None,
            left: None,
            right: None,
        }))
    }

    pub fn insert(&mut self, new_key: u32) {
        if new_key < self.key { // smaller value check left child
            match &self.left {
                Some(left_child) => left_child.borrow_mut().insert(new_key),
                None => self.left = Some(TreeNode::new(new_key)), // leaf node we actually insert
            }
        } else if new_key > self.key { // larger value check right child
            match &self.right {
                Some(right_child) => right_child.borrow_mut().insert(new_key), // call insert recursively
                None => self.right = Some(TreeNode::new(new_key)), // leaf node so we actually insert
            }
        }
    }
}

pub struct RBTree {
    pub root: RedBlackTree,
}

impl RBTree {
    pub fn new_empty() -> Self {
        RBTree { root: None }
    }

    pub fn new(key: u32) -> Self {
        let root_node = TreeNode::new(key);
        root_node.borrow_mut().color = NodeColor::Black; // root node should be black

        RBTree { 
            root: Some(root_node),
        }
    }

    pub fn insert(&mut self, key: u32) {
        match &self.root {
            Some(root) => root.borrow_mut().insert(key),
            None => self.root = Some(TreeNode::new(key)),
        }
    }

    
}
}

// // avl tree implementation here
// i guess we take out the stuff that we need for both and put it outside hte 
// // pub mod avltree { ... }

