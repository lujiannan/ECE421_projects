extern crate trees; // 
use trees::rbtree::{NodeColor, TreeNode};

// Assuming RBTree is structured properly and has an `insert` method.

fn main() {
    // Create the root of the tree with a specific key
    let root: std::rc::Rc<std::cell::RefCell<TreeNode<u32>>> = TreeNode::new(50);
    let pointer = TreeNode::regular_insert(&root, 25, NodeColor::Red);
    TreeNode::regular_insert(&root,100, NodeColor::Black);
    TreeNode::regular_insert(&root,5, NodeColor::Black);
    TreeNode::regular_insert(&root, 40, NodeColor::Red);
    TreeNode::regular_insert(&root, 99, NodeColor::Black);
    TreeNode::regular_insert(&root, 101, NodeColor::Black);
    TreeNode::regular_insert(&root, 39, NodeColor::Black);
    TreeNode::regular_insert(&root, 41, NodeColor::Black);
    // root.borrow().print_tree();
    // let step1 = TreeNode::rr_rotate(&pointer.unwrap());
    // root.borrow().print_tree();

    // new_root.unwrap().borrow().print_tree();
    // root.borrow().print_tree();

    // Note: The current implementation does not automatically balance the tree after insertions.
    // You would need to implement the red-black tree balancing logic (rotations and color changes)
    // following the tree's rules to ensure its properties are maintained.
}