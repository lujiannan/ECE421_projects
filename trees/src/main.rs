extern crate trees; // 
use trees::rbtree::TreeNode;

// Assuming RBTree is structured properly and has an `insert` method.

fn main() {
    // Create the root of the tree with a specific key
    let root: std::rc::Rc<std::cell::RefCell<TreeNode<u32>>> = TreeNode::new(200);

    // Insert new values into the tree
    TreeNode::regular_insert(&root, 300);
    let pointer: Option<std::rc::Rc<std::cell::RefCell<TreeNode<u32>>>> = TreeNode::regular_insert(&root, 50);
    TreeNode::regular_insert(&root, 25);
    TreeNode::regular_insert(&root, 75);
    TreeNode::regular_insert(&root, 30);
    TreeNode::regular_insert(&root, 60);
    TreeNode::regular_insert(&root, 80);
    TreeNode::regular_insert(&root, 10);
    TreeNode::regular_insert(&root, 5);
    TreeNode::regular_insert(&root, 20);

    // Print the tree to visually inspect its structure
    // root.borrow().print_tree();
    let node = pointer.unwrap();
    let new_root: Option<std::rc::Rc<std::cell::RefCell<TreeNode<u32>>>> = TreeNode::ll_rotate(&node);

    root.borrow().print_tree();


    // Note: The current implementation does not automatically balance the tree after insertions.
    // You would need to implement the red-black tree balancing logic (rotations and color changes)
    // following the tree's rules to ensure its properties are maintained.
}