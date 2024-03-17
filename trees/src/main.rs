extern crate trees; // 
use trees::rbtree::{NodeColor, TreeNode};

// Assuming RBTree is structured properly and has an `insert` method.

fn main() {
    let root: std::rc::Rc<std::cell::RefCell<TreeNode<u32>>> = TreeNode::new(0);
    let pointer = TreeNode::regular_insert(&root, 50, NodeColor::Black);
    TreeNode::regular_insert(&root, 25, NodeColor::Red);
    TreeNode::regular_insert(&root, 100, NodeColor::Black);;
    TreeNode::regular_insert(&root, 40, NodeColor::Red);
    let lr = TreeNode::lr_rotate(&pointer.unwrap());
    // lr.unwrap().borrow().print_tree();
    root.borrow().print_tree();
}






fn main2() {
    // Create the root of the tree with a specific key
    let root: std::rc::Rc<std::cell::RefCell<TreeNode<u32>>> = TreeNode::new(50);
    let pointer = TreeNode::regular_insert(&root, 25, NodeColor::Red);
    TreeNode::regular_insert(&root,100, NodeColor::Black);
    TreeNode::regular_insert(&root,5, NodeColor::Black);
    let pointer2 = TreeNode::regular_insert(&root, 40, NodeColor::Red);
    TreeNode::regular_insert(&root, 99, NodeColor::Black);
    TreeNode::regular_insert(&root, 101, NodeColor::Black);
    TreeNode::regular_insert(&root, 39, NodeColor::Black);
    TreeNode::regular_insert(&root, 41, NodeColor::Black);

    // let result = TreeNode::lr_rotate(&root);
    // result.unwrap().borrow().print_tree();

    let step1 = TreeNode::rr_rotate(&pointer.unwrap());
    let p = step1.unwrap().clone();
    let result = TreeNode::ll_rotate(&root);
    result.unwrap().borrow().print_tree();
    
}