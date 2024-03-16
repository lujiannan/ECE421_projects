pub mod rbtree {
    // our public red black tree module, so we can publish crate, and use in main
    use std::cell::RefCell;
    // interior mutability
    use std::rc::{Rc, Weak}; // rc for multiple references
                             // weak is for parent pointers because we can't have cyclic strong references
                             // we can upgrade the parent pointers temporarily if we need to change parent values

    #[derive(Clone, Debug, PartialEq)]
    pub enum NodeColor {
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

        // used for creating root in RBtree implementation
        // notice tree type. returns pointer to the root that we can borrow and mutate
        pub fn new(key: u32) -> Tree {
            // create a new node
            Rc::new(RefCell::new(TreeNode {
                color: NodeColor::Black, // New nodes are always red
                key,
                parent: None,
                left: None,
                right: None,
            }))
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
        pub fn regular_insert(node: &Tree, key: u32) -> RedBlackTree {
            let mut current = node.borrow_mut();

            if key < current.key {
                if let Some(ref left_child) = current.left {
                    // Continue searching in the left subtree
                    TreeNode::regular_insert(left_child, key)
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
                    TreeNode::regular_insert(right_child, key)
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


        pub fn ll_rotate(node: &Tree) ->RedBlackTree {
            let node_left = node.borrow().left.clone()?;
            let node_left_right = node_left.borrow().right.clone();
            // Set the right child of the left node to be the node itself
            node_left.borrow_mut().right = Some(node.clone());
            node_left.borrow_mut().parent = node.borrow().parent.clone();
            // Change the parent of the original node to be the left node
            node.borrow_mut().parent = Some(Rc::downgrade(&node_left));
            // Adjust the left child of the original node if needed
            if let Some(ref left_right) = node_left_right {
                left_right.borrow_mut().parent = Some(Rc::downgrade(&node_left));
            }
            node.borrow_mut().left = node_left_right;
            // If the original node had a parent, update its child pointer
            if let Some(parent_weak) = node_left.borrow().parent.as_ref() {
                if let Some(parent) = parent_weak.upgrade() {
                    if Rc::ptr_eq(&parent.borrow().left.as_ref().unwrap(), node) {
                        parent.borrow_mut().left = Some(node_left.clone());
                    } else {
                        parent.borrow_mut().right = Some(node_left.clone());
                    }
                }
            } else {
                // If there is no parent, this node was the root
                // so we need to return a some() indicating the root changed
                println!("The original node was the root node.");
            }

            let node_color = node.borrow().color.clone();
            node.borrow_mut().color = node_left.borrow().color.clone();
            node_left.borrow_mut().color = node_color;

            Some(node_left)
        }



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


        pub fn rr_rotate(node: &Tree) -> RedBlackTree {
            let node_right = node.borrow().right.clone()?;
            let node_right_left = node_right.borrow().left.clone();
            node_right.borrow_mut().left = Some(node.clone());
            node_right.borrow_mut().parent = node.borrow().parent.clone();
            node.borrow_mut().parent = Some(Rc::downgrade(&node_right));
            if let Some(ref right_left) = node_right_left {
                right_left.borrow_mut().parent = Some(Rc::downgrade(&node_right));
            }
            node.borrow_mut().right = node_right_left;
            if let Some(parent_weak) = node_right.borrow().parent.as_ref() {
                if let Some(parent) = parent_weak.upgrade() {
                    if Rc::ptr_eq(&parent.borrow().left.as_ref().unwrap(), node) {
                        parent.borrow_mut().left = Some(node_right.clone());
                    } else {
                        parent.borrow_mut().right = Some(node_right.clone());
                    }
                }
            } else {
                // If there is no parent, this node was the root
                println!("The original node was the root node.");
            }
    
            Some(node_right)
        }
    }
}
// // avl tree implementation here
// i guess we take out the stuff that we need for both and put it outside hte
// // pub mod avltree { ... }
