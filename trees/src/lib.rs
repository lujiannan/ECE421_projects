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
        


        pub fn ll_rotate(node: &Tree) -> RedBlackTree  {
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
                    match node.borrow().child_position() {
                        ChildPosition::Left => {
                            parent.borrow_mut().left = Some(node_left.clone());
                        },
                        ChildPosition::Right => {
                            parent.borrow_mut().right = Some(node_left.clone());
                        },
                        _ => {} // original node did not have parent
                    }
                }
            }
            let node_color = node.borrow().color.clone();
            node.borrow_mut().color = node_left.borrow().color.clone();
            node_left.borrow_mut().color = node_color;
            Some(node_left)
        }

        
        pub fn rr_rotate(node: &Tree) -> RedBlackTree {
            let node_right = node.borrow().right.clone()?;
            let node_right_left = node_right.borrow().left.clone();
        
            // Move node down and to the left, making the node's right child the new root of this subtree
            node_right.borrow_mut().left = Some(node.clone());
            node_right.borrow_mut().parent = node.borrow().parent.clone();
            node.borrow_mut().parent = Some(Rc::downgrade(&node_right));
        
            // If there was a right-left child, it becomes the right child of the node
            node.borrow_mut().right = node_right_left.clone();
            if let Some(ref new_right_child) = node_right_left {
                new_right_child.borrow_mut().parent = Some(Rc::downgrade(node));
            }
        
            // Update the original node's parent to point to the new top node of the subtree
            if let Some(parent_weak) = node_right.borrow().parent.as_ref() {
                if let Some(parent) = parent_weak.upgrade() {
                    match node.borrow().child_position() {
                        ChildPosition::Left => {
                            parent.borrow_mut().left = Some(node_right.clone());
                        },
                        ChildPosition::Right => {
                            parent.borrow_mut().right = Some(node_right.clone());
                        },
                        _ => {} // original node did not have a parent
                    }
                }
            }
        
            // Swap colors of node and node_right to maintain red-black properties
            let node_color = node.borrow().color.clone();
            node.borrow_mut().color = node_right.borrow().color.clone();
            node_right.borrow_mut().color = node_color;
        
            Some(node_right)
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
        }

        
        pub fn count_number_of_leaves(node: &Tree) -> usize {
            let mut count = 0;

            let node_borrowed = node.borrow();
            if node_borrowed.left.is_none() {
                count += 1;
                println!("left none {}, count: {}",node_borrowed.key , count);
            } else if let Some(ref left_child) = node_borrowed.left {
                count += TreeNode::count_number_of_leaves(left_child);
            }
            if node_borrowed.right.is_none() {
                count += 1;
                println!("right none {}, count: {}",node_borrowed.key , count);
            } else if let Some(ref right_child) = node_borrowed.right {
                count += TreeNode::count_number_of_leaves(right_child);
            }
            count
        }
        
        pub fn is_tree_empty(node: &Tree) -> bool {
            node.borrow().left.is_none() && node.borrow().right.is_none()
        }

        // pub fn get_height_of_tree (node: &Tree) -> usize {
        //     let mut height = 0;
        //     let node_borrowed = node.borrow();
        //     if node_borrowed.left.is_none() && node_borrowed.right.is_none() {
        //         return 1;
        //     }
        //     if let Some(ref left_child) = node_borrowed.left {
        //         height = 1 + TreeNode::get_height_of_tree(left_child);
        //     }
        //     if let Some(ref right_child) = node_borrowed.right {
        //         height = 1 + TreeNode::get_height_of_tree(right_child);
        //     }
        //     height
        // }

        // pub fn get_height_of_tree(node: &Tree) -> usize {
        //     let mut height = 0;
        //     let node_borrowed = node.borrow();
        //     if node_borrowed.left.is_none() && node_borrowed.right.is_none() {
        //         return 1;
        //     }
        //     if let Some(ref left_child) = node_borrowed.left {
        //         height = 1 + TreeNode::get_height_of_tree(left_child);
        //     }
        //     if let Some(ref right_child) = node_borrowed.right {
        //         height = std::cmp::max(height, 1 + TreeNode::get_height_of_tree(right_child));
        //     }
        //     height
        // }

        pub fn get_height_of_tree (node: &Tree) -> usize {
            let mut height = 1;
            let mut height_left = 0;
            let mut height_right = 0;
            let node_borrowed = node.borrow();
            if node_borrowed.left.is_none() && node_borrowed.right.is_none() {
                return height
            }
            if let Some(ref left_child) = node_borrowed.left {
                height_left = height + TreeNode::get_height_of_tree(left_child);
            }
            if let Some(ref right_child) = node_borrowed.right {
                height_right = height + TreeNode::get_height_of_tree(right_child);
            }
            height = std::cmp::max(height_left, height_right);
            height
        }

        // find a node with a given key
        pub fn find_node(node: &Tree, key: u32) -> Option<Tree> {
            if node.borrow().key == key {
                Some(node.clone())
            } else if key < node.borrow().key {
                if let Some(left_child) = &node.borrow().left {
                    TreeNode::find_node(left_child, key)
                } else {
                    None
                }
            } else {
                if let Some(right_child) = &node.borrow().right {
                    TreeNode::find_node(right_child, key)
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
        pub fn delete_node(node: &Tree) {
            let node_left = node.borrow().left.clone();
            let node_right = node.borrow().right.clone();
            let node_left_exist = node_left.is_some();
            let node_right_exist = node_right.is_some();

            if let Some(parent_weak) = &node.borrow().parent {
                if let Some(parent) = parent_weak.upgrade() {
                    // set child of the parent of the node depending on child of the node
                    match node.borrow().child_position() {
                        ChildPosition::Left => {
                            if node_left_exist && node_right_exist {
                                // deleted node has two child
                                let successor = TreeNode::find_successor(&node);
                                if let Some(ref successor_node) = successor {
                                    // Replace the current node with its successor
                                    std::mem::swap(&mut node.borrow_mut().key, &mut successor_node.borrow_mut().key);
                                    println!("{:#?}", &successor_node);
                                    // TreeNode::delete_node();
                                    // node.borrow_mut().right = new_right;
                                }
                            } else {
                                // deleted node has one or no child
                                if node_left_exist && !node_right_exist {
                                    parent.borrow_mut().left = node_left;
                                } else if node_right_exist && !node_left_exist {
                                    parent.borrow_mut().left = node_right;
                                } else if !node_left_exist && !node_right_exist{
                                    parent.borrow_mut().left = None;
                                }

                                if let Some(ref left) = node.borrow().left {
                                    left.borrow_mut().parent = Some(Rc::downgrade(&parent));
                                }
                            }
                        }
                        ChildPosition::Right => {
                            if node_left_exist && !node_right_exist {
                                parent.borrow_mut().right = node_left;
                            } else if node_right_exist && !node_left_exist {
                                parent.borrow_mut().right = node_right;
                            } else if node_left_exist && node_right_exist {
                                println!("jesus");
                                // deleted node has two child
                                let successor = TreeNode::find_successor(&node.borrow().right.clone().unwrap());
                                if let Some(ref successor_node) = successor {
                                    // Replace the current node with its successor
                                    std::mem::swap(&mut node.borrow_mut().key, &mut successor_node.borrow_mut().key);
                                    println!("{:#?}", node.borrow_mut().key);
                                    // TreeNode::delete_node();
                                    // node.borrow_mut().right = new_right;
                                }
                            } else {
                                // deleted node has no child
                                parent.borrow_mut().left = None;
                            }

                            if let Some(ref right) = node.borrow().right {
                                right.borrow_mut().parent = Some(Rc::downgrade(&parent));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub struct RBTree{
        root: RedBlackTree,
    }

    impl RBTree {
        pub fn new() -> RBTree {
            RBTree {root: None}
        }

        pub fn get_root(&self) -> RedBlackTree {
            self.root.clone()
        }

        pub fn insert(&mut self, key: u32) -> RedBlackTree {
            match self.root {
                Some(ref root) => {
                    // tree is not empty do insertion
                    TreeNode::regular_insert(root, key, NodeColor::Red)
                    // do recoloring up the tree if needed return new node
                    // perform rotation if needed (we will only need to do 1 rotation at most);
                    // rotation returns new root of subtree
                    // compare returned with current root
                    // update root
                },
                None => {
                    // if tree is empty create a new new node and set as root
                    self.root = TreeNode::new_rb(key, NodeColor::Black);
                    self.get_root()
                }
            }
        }

        pub fn delete(&mut self, key: u32) {
            match self.root {
                Some(ref root) => {
                    if let Some(node_to_delete) = TreeNode::find_node(root, key) {
                        TreeNode::delete_node(&node_to_delete);
                    } else {
                        println!("Cannot find the node in the RBTree, please check");
                        self.get_root();
                    }
                },
                None => {
                    // if tree is empty 
                    println!("The RBTree is empty, no deletion required");
                    self.get_root();
                }
            }
        }

        pub fn print_tree(&self) {
            if let Some(ref root) = self.root {
                root.borrow().print_tree();
            }
            println!();
        }
    }
}
// // avl tree implementation here
// i guess we take out the stuff that we need for both and put it outside hte
// // pub mod avltree { ... }
