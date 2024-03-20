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

    #[derive(PartialEq)]
    pub enum ChildPosition {
        Left,
        Right,
        None,
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
        pub right: RedBlackTree,
    }

    pub struct RBTree{
        root: RedBlackTree,
    }

    impl TreeNode<u32> {
        fn get_root(node: &Tree) -> RedBlackTree {
            let parent = node.borrow().parent.clone();
            match parent {
                Some(p) => Self::get_root(&p.upgrade().unwrap()),
                None => Some(node.clone()),
            }
        }

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
        
        // determines whether node is the left or right child of parent
        pub fn child_position(&self) -> ChildPosition {
            if let Some(parent_weak) = &self.parent {
                if let Some(parent) = parent_weak.upgrade() {
                    let parent_borrowed = parent.borrow();

                    if let Some(ref left_child) = parent_borrowed.left {
                        if left_child.borrow().key == self.key {
                            return ChildPosition::Left;
                        }
                    }

                    if let Some(ref right_child) = parent_borrowed.right {
                        if right_child.borrow().key == self.key {
                            return ChildPosition::Right;
                        }
                    }
                }
            }
            ChildPosition::None
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
            } else if key > current.key {
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
            } else {
                // duplicate
                Some(node.clone())
            }
        }

        pub fn is_parent_red(&self) -> bool {
            if let Some(parent_weak) = &self.parent {
                if let Some(parent) = parent_weak.upgrade() {
                    return parent.borrow().color == NodeColor::Red;
                }
            }
            false
        }
        pub fn get_parent(&self) -> Option<Tree> {
            self.parent.as_ref()?.upgrade()
        }
        pub fn get_grandparent(&self) -> Option<Tree> {
            self.get_parent()?.borrow().get_parent()
        }
        pub fn get_sibling(&self) -> Option<Tree> {
            if let Some(parent) = self.get_parent() {
                let parent_borrow = parent.borrow();
                match self.child_position() {
                    ChildPosition::Left => parent_borrow.right.clone(),
                    ChildPosition::Right => parent_borrow.left.clone(),
                    ChildPosition::None => None, // This should technically never happen if the tree is properly structured.
                }
            } else {
                None // This node has no parent, hence no sibling.
            }
        }

        pub fn get_uncle(&self) -> Option<Tree> {
            self.get_parent()?.borrow().get_sibling()
        }
        pub fn is_uncle_red(&self) -> bool {
            if let Some(uncle) = self.get_uncle() {
                return uncle.borrow().color == NodeColor::Red;
            }
            false
        }
        pub fn is_uncle_black(&self) -> bool {
            if let Some(uncle) = self.get_uncle() {
                return uncle.borrow().color == NodeColor::Black;
            }
            true // No uncle is considered as black in red-black trees.
        }
        pub fn determine_rotation(&self) -> String {
            let parent_pos = self.get_parent().map_or(ChildPosition::None, |p| p.borrow().child_position());
            let node_pos = self.child_position();
    
            match (parent_pos, node_pos) {
                (ChildPosition::Left, ChildPosition::Left) => "LL".to_string(),
                (ChildPosition::Right, ChildPosition::Right) => "RR".to_string(),
                (ChildPosition::Left, ChildPosition::Right) => "LR".to_string(),
                (ChildPosition::Right, ChildPosition::Left) => "RL".to_string(),
                _ => "None".to_string(),
            }
        }
        pub fn determine_case(&self) -> String {
            if let Some(parent) = self.get_parent() {
                // Root node or parent is black
                if parent.borrow().parent.is_none() || !self.is_parent_red() {
                    return "Nothing".to_string();
                }
                // Parent is red
                if self.is_parent_red() {
                    if self.is_uncle_red() {
                        // Uncle is red -> Recolor
                        return "Recolor".to_string();
                    } else {
                        // Uncle is black, determine rotation
                        return self.determine_rotation();
                    }
                }
            } else {
                // Node is root
                return "Nothing".to_string();
            }
            // Default case, although we should cover all scenarios above
            "Undefined".to_string()
        }


        pub fn print_node(&self) {
            // Determine the parent key if available
            let parent_key = self.get_parent().map_or("None".to_string(), |parent| {
                parent.borrow().key.to_string()
            });
    
            // Determine the color as a string
            let color = match self.color {
                NodeColor::Red => "Red",
                NodeColor::Black => "Black",
            };
    
            // Determine the keys of left and right children if available
            let left_key = self.left.as_ref().map_or("None".to_string(), |left| {
                left.borrow().key.to_string()
            });
            let right_key = self.right.as_ref().map_or("None".to_string(), |right| {
                right.borrow().key.to_string()
            });
    
            // Print the node information
            println!("Node Key: {}, Color: {}, Parent Key: {}, Left Child Key: {}, Right Child Key: {}", self.key, color, parent_key, left_key, right_key);
        }

        pub fn recolor(node: &Tree) -> RedBlackTree {
            // perform 1 iteration of recoloring and return the grandparent

            if let Some(parent) = node.borrow().get_parent() {
                parent.borrow_mut().color = NodeColor::Black;
                if let Some(uncle) = node.borrow().get_uncle() {
                    uncle.borrow_mut().color = NodeColor::Black;
                }
            }
        
            // 2: Set grandparent to red unless it is the root (root does not have a parent)
            if let Some(grandparent) = node.borrow().get_grandparent() {
                // Check if grandparent is not the root by confirming it has a parent
                if grandparent.borrow().get_parent().is_some() {
                    grandparent.borrow_mut().color = NodeColor::Red;
                }
                return Some(grandparent);
            }
        
            // If there's no grandparent, return None
            None
        }

        pub fn ll_rotate(node: &Tree) -> RedBlackTree {
            // special case for right rotation (include recoloring, for insertion use)
            let node_left = node.borrow().left.clone().expect("Left child must exist for LL rotation"); // Get the left child of the node
            let node_left_right = node_left.borrow().right.clone(); // Get the right child of the node's left child
            let parent = node.borrow().get_parent();
            let pos = node.borrow().child_position(); // Get child position before making changes
        
            node_left.borrow_mut().right = Some(node.clone()); // Move the node down. node's left right child = node
            node_left.borrow_mut().parent = node.borrow().parent.clone(); // node_left's parent = current node's parent
            node.borrow_mut().parent = Some(Rc::downgrade(&node_left)); // Change the parent of the original node to be the left node
            // If there was a left right child of the original node, move it to the current node's left
            node.borrow_mut().left = node_left_right; // Set node_left_right as the new left child
        
            // Update the parent pointer of the new left child (if it exists) to point back to `node`
            if let Some(ref left_right) = node.borrow().left {
                left_right.borrow_mut().parent = Some(Rc::downgrade(&node));
            }
        
            // Update the child pointer of the node's parent
            if let Some(parent) = parent {
                match pos {
                    ChildPosition::Left => parent.borrow_mut().left = Some(node_left.clone()),
                    ChildPosition::Right => parent.borrow_mut().right = Some(node_left.clone()),
                    _ => {}
                }
                // parent.borrow().print_tree();
            }
        
            let node_color = node.borrow().color.clone();
            node.borrow_mut().color = node_left.borrow().color.clone(); // Swap the colors of the node and its left child
            node_left.borrow_mut().color = node_color;
        
            Some(node_left) // Return the new root of the subtree
        }

        pub fn right_rotate(node: &Tree) -> RedBlackTree {
            let node_left = node.borrow().left.clone().expect("Left child must exist for LL rotation"); // Get the left child of the node
            let node_left_right = node_left.borrow().right.clone(); // Get the right child of the node's left child
            let parent = node.borrow().get_parent();
            let pos = node.borrow().child_position(); // Get child position before making changes
        
            node_left.borrow_mut().right = Some(node.clone()); // Move the node down. node's left right child = node
            node_left.borrow_mut().parent = node.borrow().parent.clone(); // node_left's parent = current node's parent
            node.borrow_mut().parent = Some(Rc::downgrade(&node_left)); // Change the parent of the original node to be the left node
            // If there was a left right child of the original node, move it to the current node's left
            node.borrow_mut().left = node_left_right; // Set node_left_right as the new left child
        
            // Update the parent pointer of the new left child (if it exists) to point back to `node`
            if let Some(ref left_right) = node.borrow().left {
                left_right.borrow_mut().parent = Some(Rc::downgrade(&node));
            }
        
            // Update the child pointer of the node's parent
            if let Some(parent) = parent {
                match pos {
                    ChildPosition::Left => parent.borrow_mut().left = Some(node_left.clone()),
                    ChildPosition::Right => parent.borrow_mut().right = Some(node_left.clone()),
                    _ => {}
                }
                // parent.borrow().print_tree();
            }
        
            Some(node_left) // Return the new root of the subtree
        }
        
        pub fn rr_rotate(node: &Tree) -> RedBlackTree {
            // special case for left rotation (include recoloring, for insertion use)
            let node_right = node.borrow().right.clone()?; // Get the right child of the node
            let node_right_left = node_right.borrow().left.clone(); // Get the left child of the node's right child
            let parent = node.borrow().get_parent();
            let pos = node.borrow().child_position(); // need to get child postion before we make any changes that might mess stuff

            node_right.borrow_mut().left = Some(node.clone()); // Move node up. node's right left child = node
            node_right.borrow_mut().parent = node.borrow().parent.clone(); // node_right parent = current node's parent
            node.borrow_mut().parent = Some(Rc::downgrade(&node_right)); // Change the parent of the original node to be the right node
            // If there was a right left child of the original node, move it to the current node's right
            node.borrow_mut().right = node_right_left; // Set node_right_left as the new right child

            // Update the parent pointer of the new right child (if it exists) to point back to `node`
            if let Some(ref right_left) = node.borrow().right {
                right_left.borrow_mut().parent = Some(Rc::downgrade(&node));
            }

            // update child pointer of node's parent
            if let Some(parent) = parent {
                match pos {
                    ChildPosition::Left => parent.borrow_mut().left = Some(node_right.clone()),
                    ChildPosition::Right => parent.borrow_mut().right = Some(node_right.clone()),
                    _ => {}
                }
                // parent.borrow().print_tree();
            }
            let node_color = node.borrow().color.clone();
            node.borrow_mut().color = node_right.borrow().color.clone(); // Swap the colors of the node and its right child
            node_right.borrow_mut().color = node_color;
            Some(node_right) // Return the new root of the subtree
        }

        pub fn left_rotate(node: &Tree) -> RedBlackTree {
            let node_right = node.borrow().right.clone()?; // Get the right child of the node
            let node_right_left = node_right.borrow().left.clone(); // Get the left child of the node's right child
            let parent = node.borrow().get_parent();
            let pos = node.borrow().child_position(); // need to get child postion before we make any changes that might mess stuff

            node_right.borrow_mut().left = Some(node.clone()); // Move node up. node's right left child = node
            node_right.borrow_mut().parent = node.borrow().parent.clone(); // node_right parent = current node's parent
            node.borrow_mut().parent = Some(Rc::downgrade(&node_right)); // Change the parent of the original node to be the right node
            // If there was a right left child of the original node, move it to the current node's right
            node.borrow_mut().right = node_right_left; // Set node_right_left as the new right child

            // Update the parent pointer of the new right child (if it exists) to point back to `node`
            if let Some(ref right_left) = node.borrow().right {
                right_left.borrow_mut().parent = Some(Rc::downgrade(&node));
            }

            // update child pointer of node's parent
            if let Some(parent) = parent {
                match pos {
                    ChildPosition::Left => parent.borrow_mut().left = Some(node_right.clone()),
                    ChildPosition::Right => parent.borrow_mut().right = Some(node_right.clone()),
                    _ => {}
                }
                // parent.borrow().print_tree();
            }
            Some(node_right) // Return the new root of the subtree
        }
        
        pub fn lr_rotate(node: &Tree) -> RedBlackTree {
            // Safety check: ensure the node has a left child
            if node.borrow().left.is_none() {
                return None;
            }
            let left_child = node.borrow().left.clone().unwrap();
            let step1 = TreeNode::rr_rotate(&left_child);
            // Step 2: Perform LL rotation on the node itself; ll rotate will return new node on top
            TreeNode::ll_rotate(node)
        }

        pub fn rl_rotate(node: &Tree) -> RedBlackTree {
            // Safety check: ensure the node has a left child
            if node.borrow().right.is_none() {
                return None;
            }
            let right_child = node.borrow().right.clone().unwrap();
            let step1 = TreeNode::ll_rotate(&right_child);
            // Step 2: Perform LL rotation on the node itself; ll rotate will return new node on top
            TreeNode::rr_rotate(node)
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
            println!();
        }
        
        pub fn node_count_number_of_leaves(node: &Tree) -> usize {
            let mut count = 0;

            let node_borrowed = node.borrow();
            if node_borrowed.left.is_none() {
                count += 1;
                // println!("left none {}, count: {}",node_borrowed.key , count);
            } else if let Some(ref left_child) = node_borrowed.left {
                count += TreeNode::node_count_number_of_leaves(left_child);
            }
            if node_borrowed.right.is_none() {
                count += 1;
                // println!("right none {}, count: {}",node_borrowed.key , count);
            } else if let Some(ref right_child) = node_borrowed.right {
                count += TreeNode::node_count_number_of_leaves(right_child);
            }
            count
        }
        
        // pub fn node_is_tree_empty(node: &Tree) -> bool {
        //     node.borrow().left.is_none() && node.borrow().right.is_none()
        // }

        pub fn node_get_height_of_tree (node: &Tree) -> usize {
            let mut height = 1;
            let mut height_left = 0;
            let mut height_right = 0;
            let node_borrowed = node.borrow();
            if node_borrowed.left.is_none() && node_borrowed.right.is_none() {
                return height
            }
            if let Some(ref left_child) = node_borrowed.left {
                height_left = height + TreeNode::node_get_height_of_tree(left_child);
            }
            if let Some(ref right_child) = node_borrowed.right {
                height_right = height + TreeNode::node_get_height_of_tree(right_child);
            }
            height = std::cmp::max(height_left, height_right);
            height
        }

        pub fn node_print_in_order_traversal(&self) {
            if let Some(ref left) = self.left {
                left.borrow().node_print_in_order_traversal();
            }
            print!("{:?} ", self.key);
            if let Some(ref right) = self.right {
                right.borrow().node_print_in_order_traversal();
            }
        }

        pub fn node_print_pre_order_traversal(&self) {
            print!("{:?} ", self.key);
            if let Some(ref left) = self.left {
                left.borrow().node_print_pre_order_traversal();
            }
            if let Some(ref right) = self.right {
                right.borrow().node_print_pre_order_traversal();
            }
        }

        // find a node with a given key
        pub fn find_node(node: &Tree, key: u32) -> Option<Tree> {
            if node.borrow().key == key {
                Some(node.clone())
            } else if key < node.borrow().key {
                if let Some(left_child) = &node.borrow().left {
                    Self::find_node(left_child, key)
                } else {
                    None
                }
            } else {
                if let Some(right_child) = &node.borrow().right {
                    Self::find_node(right_child, key)
                } else {
                    None
                }
            }
        }

        // find the successor of a node in the tree
        fn find_successor(node: &Tree) -> Option<Tree> {
            let mut current = node.borrow().right.clone();
            while current.is_some() {
                let next = current.clone().unwrap().borrow().left.clone();
                if next.is_none() {
                    break;
                } else {
                    current = next;
                }
            }
            // println!("{:#?}", current);
            current
        }

        // remove a node from the tree
        pub fn delete_node(node: &Tree) -> RedBlackTree{
            let node_left = node.borrow().left.clone();
            let node_right = node.borrow().right.clone();
            let node_parent = node.borrow().parent.clone();
            let node_left_exist = node_left.is_some();
            let node_right_exist = node_right.is_some();

            let child_position = node.borrow().child_position();
            // set child of the parent of the node depending on child of the node
            match child_position {
                ChildPosition::Left => {
                    if node_left_exist && node_right_exist {
                        // delete node with two child
                        let successor = Self::find_successor(&node);
                        if let Some(ref successor_node) = successor {
                            // Replace the current node with its successor
                            std::mem::swap(&mut node.borrow_mut().key, &mut successor_node.borrow_mut().key);
                            Self::delete_node(successor_node);
                        }
                    } else {
                        // delete node with one or no child
                        if node.borrow().color == NodeColor::Red {
                            // current node is red
                            node_parent.unwrap().upgrade().unwrap().borrow_mut().left = None;
                        } else {
                            // current node is black
                            if node_left_exist && !node_right_exist {
                                let node_left_cp = node_left.unwrap();
                                // Black + left red case
                                match node_parent {
                                    None => {
                                        node_left_cp.borrow_mut().color = node.borrow().color.clone();
                                        node_left_cp.borrow_mut().parent = None;
                                        return Some(node_left_cp);
                                    }
                                    Some(node_parent) => {
                                        // node is left child of parent
                                        node_parent.upgrade().unwrap().borrow_mut().left = Some(node_left_cp.clone());
                                        node_left_cp.borrow_mut().parent = Some(node_parent.clone());
                                        node_left_cp.borrow_mut().color = node.borrow().color.clone();
                                    }
                                }
                            } else if !node_left_exist && node_right_exist {
                                let node_right_cp = node_right.unwrap();
                                // Black + right red case
                                match node_parent {
                                    None => {
                                        node_right_cp.borrow_mut().color = node.borrow().color.clone();
                                        node_right_cp.borrow_mut().parent = None;
                                        return Some(node_right_cp);
                                    }
                                    Some(node_parent) => {
                                        // node is left child of parent
                                        node_parent.upgrade().unwrap().borrow_mut().left = Some(node_right_cp.clone());
                                        node_right_cp.borrow_mut().parent = Some(node_parent.clone());
                                        node_right_cp.borrow_mut().color = node.borrow().color.clone();
                                    }
                                }
                            } else {
                                // black + no children case
                                match node_parent {
                                    None => return None,
                                    Some(node_parent) => {
                                        Self::delete_maintain(&node.clone());
                                        node_parent.upgrade().unwrap().borrow_mut().left = None;
                                        node.borrow_mut().parent = None;
                                    }
                                }
                            }
                        }
                    }
                }
                ChildPosition::Right => {
                    if node_left_exist && node_right_exist {
                        // delete node with two child
                        let successor = Self::find_successor(&node);
                        if let Some(ref successor_node) = successor {
                            // Replace the current node with its successor
                            std::mem::swap(&mut node.borrow_mut().key, &mut successor_node.borrow_mut().key);
                            Self::delete_node(successor_node);
                        }
                    } else {
                        // delete node with one or no child
                        if node.borrow().color == NodeColor::Red {
                            // current node is red
                            node_parent.unwrap().upgrade().unwrap().borrow_mut().right = None;
                        } else {
                            // current node is black
                            if node_left_exist && !node_right_exist {
                                let node_left_cp = node_left.unwrap();
                                // Black + left red case
                                match node_parent {
                                    None => {
                                        node_left_cp.borrow_mut().color = node.borrow().color.clone();
                                        node_left_cp.borrow_mut().parent = None;
                                        return Some(node_left_cp);
                                    }
                                    Some(node_parent) => {
                                        // node is right child of parent
                                        node_parent.upgrade().unwrap().borrow_mut().right = Some(node_left_cp.clone());
                                        node_left_cp.borrow_mut().parent = Some(node_parent.clone());
                                        node_left_cp.borrow_mut().color = node.borrow().color.clone();
                                    }
                                }
                            } else if !node_left_exist && node_right_exist {
                                let node_right_cp = node_right.unwrap();
                                // Black + right red case
                                match node_parent {
                                    None => {
                                        node_right_cp.borrow_mut().color = node.borrow().color.clone();
                                        node_right_cp.borrow_mut().parent = None;
                                        return Some(node_right_cp);
                                    }
                                    Some(node_parent) => {
                                        // node is left child of parent
                                        node_parent.upgrade().unwrap().borrow_mut().right = Some(node_right_cp.clone());
                                        node_right_cp.borrow_mut().parent = Some(node_parent.clone());
                                        node_right_cp.borrow_mut().color = node.borrow().color.clone();
                                    }
                                }
                            } else {
                                // black + no children case
                                match node_parent {
                                    None => return None,
                                    Some(node_parent) => {
                                        Self::delete_maintain(&node.clone());
                                        node_parent.upgrade().unwrap().borrow_mut().right = None;
                                        node.borrow_mut().parent = None;
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                    // delete root
                    if node_left_exist && !node_right_exist {
                        let node_left_cp = node_left.unwrap();
                        node_left_cp.borrow_mut().color = node.borrow().color.clone();
                        node_left_cp.borrow_mut().parent = None;
                        return Some(node_left_cp);
                    } else if !node_left_exist && node_right_exist {
                        let node_right_cp = node_right.unwrap();
                        node_right_cp.borrow_mut().color = node.borrow().color.clone();
                        node_right_cp.borrow_mut().parent = None;
                        return Some(node_right_cp);
                    } else if node_left_exist && node_right_exist {
                        // delete root with two child
                        let successor = Self::find_successor(&node);
                        if let Some(ref successor_node) = successor {
                            // Replace the current node with its successor
                            std::mem::swap(&mut node.borrow_mut().key, &mut successor_node.borrow_mut().key);
                            Self::delete_node(successor_node);
                        }
                    } else {
                        return None;
                    }
                }
            }
            Self::get_root(node)
        }

        fn get_color(node: &RedBlackTree) -> NodeColor {
            // a None node returns black color
            match node {
                None => NodeColor::Black,
                Some(node) => node.borrow().color.clone(),
            }
        }

        fn delete_maintain(node: &Tree) {
            // maintain rbtree property after delete a black node with no children (& not root)
            let parent = node.borrow().parent.clone();
            match parent {
                None => return,
                Some(parent) => {
                    let parent = parent.upgrade().unwrap();
                    let sibling = Self::get_sibling(&node.clone().borrow());
                    let node_position = node.borrow().child_position();
                    match sibling {
                        None => return,
                        Some(sibling) => {
                            // sibling is black
                            if sibling.borrow().color == NodeColor::Black {
                                let sibling_cclose; // sibling's cloest child to node
                                let sibling_cfar;   // sibling's distant child to node
                                if node_position == ChildPosition::Left {
                                    sibling_cclose = sibling.borrow().left.clone();
                                    sibling_cfar = sibling.borrow().right.clone();
                                } else {
                                    sibling_cclose = sibling.borrow().right.clone();
                                    sibling_cfar = sibling.borrow().left.clone();
                                }
                                if Self::get_color(&sibling_cclose) == NodeColor::Black && Self::get_color(&sibling_cfar) == NodeColor::Black {
                                    // close and distant are black
                                    if parent.borrow().color == NodeColor::Black {
                                        // parent is also black
                                        sibling.clone().borrow_mut().color = NodeColor::Red;
                                        Self::delete_maintain(&parent.clone());
                                    } else {
                                        // parent is red
                                        sibling.clone().borrow_mut().color = NodeColor::Red;
                                        parent.clone().borrow_mut().color = NodeColor::Black;
                                    }
                                } else if Self::get_color(&sibling_cclose) == NodeColor::Red && Self::get_color(&sibling_cfar) == NodeColor::Black {
                                    // close is red, distant is black
                                    if node_position == ChildPosition::Left {
                                        Self::right_rotate(&sibling.clone());
                                    } else {
                                        Self::left_rotate(&sibling.clone());
                                    }
                                    sibling.clone().borrow_mut().color = NodeColor::Red;
                                    sibling_cclose.clone().unwrap().borrow_mut().color = NodeColor::Black;
                                    Self::delete_maintain(&node.clone());
                                } else if Self::get_color(&sibling_cfar) == NodeColor::Red {
                                    // distant is red
                                    if node_position == ChildPosition::Left {
                                        Self::left_rotate(&parent.clone());
                                    } else {
                                        Self::right_rotate(&parent.clone());
                                    }
                                    sibling.clone().borrow_mut().color = parent.borrow().color.clone();
                                    parent.clone().borrow_mut().color = NodeColor::Black;
                                    sibling_cfar.clone().unwrap().borrow_mut().color = NodeColor::Black;
                                }
                            } else {
                                // sibling is red
                                if node_position == ChildPosition::Left {
                                    Self::left_rotate(&parent.clone());
                                } else {
                                    let result = Self::right_rotate(&parent.clone());
                                    // result.unwrap().borrow().print_tree();
                                }
                                parent.clone().borrow_mut().color = NodeColor::Red;
                                sibling.clone().borrow_mut().color = NodeColor::Black;
                                Self::delete_maintain(&node.clone());
                            }
                        }
                    }
                }
            }
        }
    }



    impl RBTree {
        pub fn new() -> RBTree {
            RBTree {root: None}
        }

        pub fn get_root(&self) -> RedBlackTree {
            self.root.clone()
        }
        pub fn r_insert(&mut self, key: u32, color: NodeColor) -> RedBlackTree {
            match self.root {
                Some(ref root) => {
                    // tree is not empty do insertion
                    TreeNode::regular_insert(root, key, color)

                },
                None => {
                    // if tree is empty create a new new node and set as root
                    self.root = TreeNode::new_rb(key, NodeColor::Black);
                    self.get_root()
                }
            }
        }

        pub fn insert(&mut self, key: u32) -> RedBlackTree {
            match self.root {
                Some(ref root) => {
                    // tree is not empty do insertion

                    // 1: do regular insert
                    let mut new_node = TreeNode::regular_insert(root, key, NodeColor::Red)?;

                    // 2: recolor up the tree. recolor -> check if need to recolor on grandparent -> recolor and so on
                    while new_node.borrow().determine_case() == "Recolor" {
                        new_node = TreeNode::recolor(&new_node)?;
                    }

                    
                    

                    // new_node.borrow().print_tree();
                    // we may hae a node higher up in the tree depending on how many time recoloring ran
                    // 3: check if need rotation -> perform rotation. 
                    // determine case on current node. but our rotations take in the top node so we need to get grandparent
                    let rotation_case = new_node.borrow().determine_case();
                    let rotated_root = match rotation_case.as_str() {
                        "LL" => {
                            let top = new_node.borrow().get_grandparent()?;
                            TreeNode::ll_rotate(&top)
                        },
                        "RR" => {
                            let top = new_node.borrow().get_grandparent()?;
                            TreeNode::rr_rotate(&top)
                        },
                        "LR" => {
                            let top = new_node.borrow().get_grandparent()?;
                            TreeNode::lr_rotate(&top)
                        },
                        "RL" => {
                            let top = new_node.borrow().get_grandparent()?;
                            TreeNode::rl_rotate(&top)
                        },
                        "None" => None, // No rotation needed, or handle as appropriate
                        _ => None, // Catch-all case, unlikely to be reached
                    };

                    // // rotated_root.unwrap().borrow().print_node();
                    if let Some(sub_root) = rotated_root {
                        if sub_root.borrow().parent.is_none() {
                            self.root = Some(sub_root.clone());
                        }
                    }

                    
                    // 4: rotation might change the root. if root of new subtree has no parent then it is the new root
                    

                    
                    None
                },
                None => {
                    // if tree is empty create a new new node and set as root
                    self.root = TreeNode::new_rb(key, NodeColor::Black);
                    self.get_root()
                }
            }
        }

        pub fn delete(&mut self, key: u32) -> RedBlackTree{
            match self.root {
                Some(ref root) => {
                    if let Some(node_to_delete) = TreeNode::find_node(root, key) {
                        let result = TreeNode::delete_node(&node_to_delete);
                        self.root = result;
                        None
                    } else {
                        println!("Cannot find the node in the RBTree, please check");
                        self.get_root()
                    }
                },
                None => {
                    // if tree is empty 
                    println!("The RBTree is empty, no deletion required");
                    self.get_root()
                }
            }
        }

        pub fn count_number_of_leaves(&self) -> usize {
            let mut count = 0;
            if let Some(ref node) = self.root {
                count = TreeNode::node_count_number_of_leaves(node)
            } 
            println!("count_number_of_leaves: {}", count);
            count
        }
        
        pub fn is_tree_empty(&self) -> bool {
            let mut state = true;
            if let Some(ref node) = self.root {
                // state = TreeNode::node_is_tree_empty(node)
                state = false
            } else {
                state = true;
            }
            println!("is_tree_empty: {}", state);
            state
        }

        pub fn get_height_of_tree(&self) -> usize {
            let mut height = 0;
            if let Some(ref node) = self.root {
                height = TreeNode::node_get_height_of_tree(node)
            } 
            println!("get_height_of_tree: {}", height);
            height
        }

        pub fn print_in_order_traversal(&self) {
            println!("In order traversal: ");
            if let Some(ref node) = self.root {
                TreeNode::node_print_in_order_traversal(&node.borrow());
            }
            println!();
        }

        pub fn print_pre_order_traversal(&self) {
            println!("Pre order traversal: ");
            if let Some(ref node) = self.root {
                TreeNode::node_print_pre_order_traversal(&node.borrow());
            } 
            println!();
        }

        pub fn print_tree(&self) {
            if let Some(ref root) = self.root {
                root.borrow().print_tree();
            }
        }
    }
}




// // avl tree implementation here
// // i guess we take out the stuff that we need for both and put it outside hte
// pub mod avltree {}