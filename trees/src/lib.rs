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
                color: NodeColor::Black, // root
                key,
                parent: None,
                left: None,
                right: None,
            }))
        }


        // used in insert function. we return full RedBlackTree type. so we dont need to wrap Tree in some everytime
        pub fn new_rb(key: u32, c: NodeColor) -> RedBlackTree {
            Some(Rc::new(RefCell::new(TreeNode {
                color: c,
                key,
                parent: None,
                left: None,
                right: None,
            })))
        }

        // insert new node and set the parent which is a weak reference to the previous node
        // need to handle all the cases after insertion
        pub fn regular_insert(node: &Tree, key: u32, color: NodeColor) -> RedBlackTree {
            let mut current = node.borrow_mut();

            if key < current.key {
                if let Some(ref left_child) = current.left {
                    // Continue searching in the left subtree
                    TreeNode::regular_insert(left_child, key, color)
                } else {
                    // Insert new node here
                    let new_node = TreeNode::new_rb(key, color);
                    if let Some(ref new_node_rc) = new_node {
                        new_node_rc.borrow_mut().parent = Some(Rc::downgrade(node));
                    }
                    current.left = new_node.clone();
                    new_node
                }
            } else {
                if let Some(ref right_child) = current.right {
                    // Continue searching in the right subtree
                    TreeNode::regular_insert(right_child, key, color)
                } else {
                    // Insert new node here
                    let new_node = TreeNode::new_rb(key, color);
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
            node_left.borrow_mut().right = Some(node.clone()); // move node up. node's left right child = node
            node_left.borrow_mut().parent = node.borrow().parent.clone(); // node_left parent = curret node's parent
            node.borrow_mut().parent = Some(Rc::downgrade(&node_left)); // Change the parent of the original node to be the left node
            // if there was a left right child of original node move it to current node's left
            node.borrow_mut().left = node_left_right; // Set node_left_right as the new left child
            // Update the parent pointer of the new left child (if it exists) to point back to `node`
            if let Some(ref left_right) = node.borrow().left {
            left_right.borrow_mut().parent = Some(Rc::downgrade(&node));
            }
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
                // println!("The original node was the root node.");
            }
            let node_color = node.borrow().color.clone();
            node.borrow_mut().color = node_left.borrow().color.clone();
            node_left.borrow_mut().color = node_color;
            Some(node_left)
        }

        pub fn rr_rotate(node: &Tree) -> RedBlackTree {
            let node_right = node.borrow().right.clone()?;
            let node_right_left = node_right.borrow().left.clone();
            node_right.borrow_mut().left = Some(node.clone()); // Move node up. node's right left child = node
            node_right.borrow_mut().parent = node.borrow().parent.clone(); // node_right parent = current node's parent
            node.borrow_mut().parent = Some(Rc::downgrade(&node_right)); // Change the parent of the original node to be the right node
            // If there was a right left child of the original node, move it to the current node's right
            node.borrow_mut().right = node_right_left; // Set node_right_left as the new right child
            // Update the parent pointer of the new right child (if it exists) to point back to `node`
            if let Some(ref right_left) = node.borrow().right {
                right_left.borrow_mut().parent = Some(Rc::downgrade(&node));
            }
            // If the original node had a parent, update its child pointer
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
                // so we need to return Some() indicating the root changed
            }
            let node_color = node.borrow().color.clone();
            node.borrow_mut().color = node_right.borrow().color.clone();
            node_right.borrow_mut().color = node_color;
            Some(node_right)
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
                if is_left { "└" } else { "┌" },
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
}
// // avl tree implementation here
// i guess we take out the stuff that we need for both and put it outside hte
// // pub mod avltree { ... }
