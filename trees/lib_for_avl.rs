// // avl tree implementation here
// // i guess we take out the stuff that we need for both and put it outside hte
// pub mod avltree { 
//     // our public red black tree module, so we can publish crate, and use in main
//     use std::cell::RefCell; // interior mutability
//     use std::rc::{Rc, Weak}; // rc for multiple references
//                              // weak is for parent pointers because we can't have cyclic strong references
//                              // we can upgrade the parent pointers temporarily if we need to change parent values    
    
//     #[derive(Clone, Debug, PartialEq)]
//     pub enum NodeColor { // * delete after updating old functions (with colour) below, to without colour
//         Red,
//         Black,
//     }

//     pub enum ChildPosition {
//         Left,
//         Right,
//         None,
//     }

    
//     type Tree = Rc<RefCell<TreeNode<u32>>>;  
//     type WeakTree = Weak<RefCell<TreeNode<u32>>>;
//     type AVLnodeTree = Option<Tree>;
//     type WeakAVLnodeTree = Option<WeakTree>;

//     #[derive(Clone, Debug)] // had to remove Partialeq because it can't be used on weak references. we can implement ourself if needed
//     pub struct TreeNode<T> {
//         pub height: usize, // has to be number
//         pub key: T,
//         // pub parent: WeakAVLnodeTree, // Weak references for cyclic stuff to prevent memory leaks
//         pub left: AVLnodeTree,
//         pub right: AVLnodeTree,
//     }     

//     pub struct AVLTree{
//         root: AVLnodeTree,
//     }

//     impl TreeNode<u32> {
//         fn get_root(node: &Tree) -> AVLnodeTree {
//             let parent = node.borrow().parent.clone();
//             match parent {
//                 Some(p) => Self::get_root(&p.upgrade().unwrap()),
//                 None => Some(node.clone()),
//             }
//         }

//         // used for creating root in RBtree implementation
//         // notice tree type. returns pointer to the root that we can borrow and mutate
//         pub fn new(key: u32) -> Tree {
//             // create a new node
//             Rc::new(RefCell::new(TreeNode {
//                 height: 1,
//                 key,
//                 left: None,
//                 right: None,
//             }))
//         }


//         // used in insert function. we return full RedBlackTree type. so we dont need to wrap Tree in some everytime
//         pub fn new_rb(key: u32, c: NodeColor) -> AVLnodeTree { //new_avl
//             Some(Rc::new(RefCell::new(TreeNode {
//                 height: 1, //?confirm?
//                 key,
//                 left: None,
//                 right: None,
//             })))
//         }
//     }

//     impl AVLTree {
//         pub fn new() -> AVLTree {
//             AVLTree {root: None}
//         }

//         pub fn get_root(&self) -> AVLnodeTree {
//             self.root.clone()
//         }

//         pub fn r_insert(&mut self, key: u32, color: NodeColor) -> AVLnodeTree {
//             match self.root {
//                 Some(ref root) => {
//                     // tree is not empty do insertion
//                     TreeNode::regular_insert(root, key, color)
//                 },
//                 None => {
//                     // if tree is empty create a new new node and set as root
//                     self.root = TreeNode::new_rb(key, NodeColor::Black);
//                     self.get_root()
//                 }
//             }
//         }

//         pub fn insert(&mut self, key: u32) -> AVLnodeTree {
//             match self.root {
//                 Some(ref root) => {
//                     // tree is not empty do insertion

//                     // 1: do regular insert
//                     let mut new_node = TreeNode::regular_insert(root, key, NodeColor::Red)?;

//                     // 2: recolor up the tree. recolor -> check if need to recolor on grandparent -> recolor and so on
//                     while new_node.borrow().determine_case() == "Recolor" {
//                         new_node = TreeNode::recolor(&new_node)?;
//                     }
//                     root.borrow().print_tree();
//                     // new_node.borrow().print_tree();
//                     // we may hae a node higher up in the tree depending on how many time recoloring ran
//                     // 3: check if need rotation -> perform rotation. 
//                     // determine case on current node. but our rotations take in the top node so we need to get grandparent
//                     let rotation_case = new_node.borrow().determine_case();
//                     let rotated_root = match rotation_case.as_str() {
//                         "LL" => {
//                             let top = new_node.borrow().get_grandparent()?;
//                             TreeNode::ll_rotate(&top)
//                         },
//                         "RR" => {
//                             let top = new_node.borrow().get_grandparent()?;
//                             TreeNode::rr_rotate(&top)
//                         },
//                         "LR" => {
//                             let top = new_node.borrow().get_grandparent()?;
//                             TreeNode::lr_rotate(&top)
//                         },
//                         "RL" => {
//                             let top = new_node.borrow().get_grandparent()?;
//                             TreeNode::rl_rotate(&top)
//                         },
//                         "None" => None, // No rotation needed, or handle as appropriate
//                         _ => None, // Catch-all case, unlikely to be reached
//                     };

//                     // rotated_root.unwrap().borrow().print_node();
//                     if let Some(sub_root) = rotated_root {
//                         if sub_root.borrow().parent.is_none() {
//                             self.root = Some(sub_root.clone());
//                         }
//                     }

                    
//                     // 4: rotation might change the root. if root of new subtree has no parent then it is the new root
                    

                    
//                     None
//                 },
//                 None => {
//                     // if tree is empty create a new new node and set as root
//                     self.root = TreeNode::new_rb(key, NodeColor::Black);
//                     self.get_root()
//                 }
//             }
//         }

//         pub fn delete(&mut self, key: u32) -> AVLnodeTree{
//             match self.root {
//                 Some(ref root) => {
//                     if let Some(node_to_delete) = TreeNode::find_node(root, key) {
//                         TreeNode::delete_node(&node_to_delete)
//                     } else {
//                         println!("Cannot find the node in the RBTree, please check");
//                         self.get_root()
//                     }
//                 },
//                 None => {
//                     // if tree is empty 
//                     println!("The RBTree is empty, no deletion required");
//                     self.get_root()
//                 }
//             }
//         }

//         pub fn count_number_of_leaves(&self) -> usize {
//             let mut count = 0;
//             if let Some(ref node) = self.root {
//                 count = TreeNode::node_count_number_of_leaves(node)
//             } 
//             println!("count_number_of_leaves: {}", count);
//             count
//         }
        
//         pub fn is_tree_empty(&self) -> bool {
//             let mut state = true;
//             if let Some(ref node) = self.root {
//             // state = TreeNode::node_is_tree_empty(node)
//                 state = false
//             } else {
//                 state = true;
//             }
//             println!("is_tree_empty: {}", state);
//             state
//         }

//         pub fn get_height_of_tree(&self) -> usize {
//             let mut height = 0;
//             if let Some(ref node) = self.root {
//                 height = TreeNode::node_get_height_of_tree(node)
//             } 
//             println!("get_height_of_tree: {}", height);
//             height
//         }

//         pub fn print_in_order_traversal(&self) {
//             println!("In order traversal: ");
//             if let Some(ref node) = self.root {
//                 TreeNode::node_print_in_order_traversal(&node.borrow());
//             }
//             println!();
//         }

//         pub fn print_pre_order_traversal(&self) {
//             println!("Pre order traversal: ");
//             if let Some(ref node) = self.root {
//                 TreeNode::node_print_pre_order_traversal(&node.borrow());
//             } 
//             println!();
//         }

//         pub fn print_tree(&self) {
//             if let Some(ref root) = self.root {
//                 root.borrow().print_tree();
//             }
//         }
//     }



// }