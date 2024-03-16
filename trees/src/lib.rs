pub mod rbtree{ // our public red black tree module, so we can publish crate, and use in main
    use std::cell::RefCell;
    // interior mutability
    use std::rc::{Rc, Weak}; // rc for multiple references
    // weak is for parent pointers because we can't have cyclic strong references
    // we can upgrade the parent pointers temporarily if we need to change parent values
    
    #[derive(Clone, Debug, PartialEq)]
    enum NodeColor {
        Red,
        Black,
    }
    
    type Tree = Rc<RefCell<TreeNode<u32>>>;
    type WeakTree = Weak<RefCell<TreeNode<u32>>>;
    type RedBlackTree = Option<Tree>;
    type WeakRedBlackTree = Option<WeakTree>;
    
    #[derive(Clone, Debug)] // had to remove Partialeq because it can't be used on weak references. we can implement ourself if needed
    pub struct TreeNode<T> {
        pub color: NodeColor,
        pub key: T,
        pub parent: WeakRedBlackTree, // Weak references for cyclic stuff to prevent memory leaks
        pub left: RedBlackTree,
        right: RedBlackTree,
    }
    
    impl TreeNode<u32> {
        pub fn get_parent_key(&self) -> Option<u32> {
            // Attempt to upgrade the Weak pointer to a strong reference
            if let Some(parent_weak) = &self.parent {
                if let Some(parent_rc) = parent_weak.upgrade() {
                    // If upgrade is successful, borrow the parent to access its key
                    let parent = parent_rc.borrow();
                    Some(parent.key)
                } else {
                    // The parent has been dropped
                    None
                }
            } else {
                // No parent
                None
            }
        }
    
        // used for creating root in RBtree implementation
        // notice tree type. returns pointer to the root that we can borrow and mutate
        pub fn new(key: u32) -> Tree { // create a new node
            Rc::new(RefCell::new(TreeNode {
                color: NodeColor::Red, // New nodes are always red
                key,
                parent: None,
                left: None,
                right: None,
            }))
        }
    
        fn new_node(key: u32) -> Self {
            Self {
                color: NodeColor::Red, // New nodes are always red
                key,
                parent: None,
                left: None,
                right: None,
            }
        }
        
    
        // used in insert function. we return full RedBlackTree type. so we dont need to wrap Tree in some everytime
        pub fn new_rb(key: u32) -> RedBlackTree {
            Some(Rc::new(RefCell::new(TreeNode {
                color: NodeColor::Red,
                key,
                parent: None,
                left: None,
                right: None,
            })))
        }
        
        // insert new node and set the parent which is a weak reference to the previous node
        // need to handle all the cases after insertion
        pub fn insert(node: &Tree, key: u32) -> RedBlackTree {
            let mut current = node.borrow_mut();
            
            if key < current.key {
                if let Some(ref left_child) = current.left {
                    // Continue searching in the left subtree
                    TreeNode::insert(left_child, key)
                } else {
                    // Insert new node here
                    let new_node = TreeNode::new_rb(key);
                    if let Some(ref new_node_rc) = new_node {
                        new_node_rc.borrow_mut().parent = Some(Rc::downgrade(node));
                    }
                    current.left = new_node.clone();
                    new_node
                }
            } else {
                if let Some(ref right_child) = current.right {
                    // Continue searching in the right subtree
                    TreeNode::insert(right_child, key)
                } else {
                    // Insert new node here
                    let new_node = TreeNode::new_rb(key);
                    if let Some(ref new_node_rc) = new_node {
                        new_node_rc.borrow_mut().parent = Some(Rc::downgrade(node));
                    }
                    current.right = new_node.clone();
                    new_node
                }
            }
        }
    
    
        pub fn pretty_print(&self, prefix: String, is_left: bool) {
            if let Some(right_child) = &self.right {
                right_child.borrow().pretty_print(
                    format!("{}{}", prefix, if is_left { "│   " } else { "    " }),
                    false,
                );
            }
    
            println!(
                "{}{}── {}{}{}",
                prefix,
                if is_left { "┌" } else { "└" },
                self.key,
                match self.color {
                    NodeColor::Red => " (Red)",
                    NodeColor::Black => " (Black)",
                },
                match self.get_parent_key() {
                    Some(parent_key) => parent_key.to_string(),
                    None => "".to_string(), // No parent key available
                }
            );
    
            if let Some(left_child) = &self.left {
                left_child.borrow().pretty_print(
                    format!("{}{}", prefix, if is_left { "    " } else { "│   " }),
                    true,
                );
            }
        }
    
        // Helper method to start the pretty printing process
        pub fn print_tree(&self) {
            self.pretty_print(String::new(), false);
        }
    }

    //////////// red black tree
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
            if let Some(ref root) = self.root {
                // If the tree is not empty, insert the new key using the existing root
                TreeNode::insert(root, key);
                // Additional steps might be required here to ensure the tree maintains Red-Black properties
            } else {
                // If the tree is empty, create a new root node with the given key
                let new_root = TreeNode::new(key);
                new_root.borrow_mut().color = NodeColor::Black; // The root is always black
                self.root = Some(new_root);
            }
        }

        pub fn tree_pretty_print(&mut self) {
            if let Some(ref root) = self.root {
                // If the tree is not empty, insert the new key using the existing root
                root.borrow().print_tree();
            }
        }

    }
    }
    // // avl tree implementation here
    // i guess we take out the stuff that we need for both and put it outside hte 
    // // pub mod avltree { ... }
    
    