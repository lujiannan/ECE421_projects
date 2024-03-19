extern crate trees; // 
use trees::rbtree::*;

// Assuming RBTree is structured properly and has an `insert` method.


fn main1() {
    let mut root = RBTree::new();
    root.insert(50);
    root.insert(25);
    root.insert(100);
    root.insert(15);
    root.insert(10);
    root.insert(20);
    root.insert(17);
    root.insert(12);
    root.insert(16);
    root.print_tree();
}

fn passed_example2() {
    // Create the root of the tree with a specific key
    let mut root = RBTree::new();
    root.insert(10);
    root.insert(20);
    root.insert(30);
    root.insert(50);
    root.insert(40);
    root.insert(60);
    root.insert(70);
    root.insert(80);
    root.insert(4);
    root.insert(8); 
    root.print_tree();
}

fn main() {
    // Create the root of the tree with a specific key
    let mut root = RBTree::new();
    // root.is_tree_empty();
    // root.print_tree();
    root.insert(50);
    // root.is_tree_empty();
    // root.print_tree();
    root.insert(25);
    // root.is_tree_empty();
    // root.print_tree();
    root.insert(100);
    root.insert(15);
    root.insert(10);
    root.insert(20);
    root.insert(17);
    root.insert(12);
    root.insert(5);

    root.print_tree();
    root.delete(20);
    root.print_tree();
    root.delete(50);
    root.print_tree();
    // root.count_number_of_leaves();
    // root.get_height_of_tree();
    // root.is_tree_empty();
    // root.print_in_order_traversal();
    // root.print_pre_order_traversal();
}

fn main2() {
    // Create the root of the tree with a specific key
    

    let root: std::rc::Rc<std::cell::RefCell<TreeNode<u32>>> = TreeNode::new(20);
    TreeNode::regular_insert(&root, 10, NodeColor::Black);
    let p1 = TreeNode::regular_insert(&root, 30, NodeColor::Black);
    let p2 = TreeNode::regular_insert(&root, 50, NodeColor::Red);
    TreeNode::regular_insert(&root, 40, NodeColor::Red);
    TreeNode::ll_rotate(&p2.unwrap());
    TreeNode::rr_rotate(&p1.unwrap());  // need rr rotation to work for all cases not just when there
    root.borrow().print_tree();
    
    
    
}

fn main3() {
    // Create the root of the tree with a specific key
    let root: std::rc::Rc<std::cell::RefCell<TreeNode<u32>>> = TreeNode::new(20);
    TreeNode::regular_insert(&root, 10, NodeColor::Black);
    let p1 = TreeNode::regular_insert(&root, 30, NodeColor::Black);
    let p2 = TreeNode::regular_insert(&root, 50, NodeColor::Red);
    let p3 = TreeNode::regular_insert(&root, 40, NodeColor::Red);
    // root.borrow().print_tree(); need to perform RL rotation after inserting 40
    // need to do LL on 50 and then RR on 30. 
    if let Some(p2) = p2 {
        TreeNode::ll_rotate(&p2);
        TreeNode::rr_rotate(&p1.unwrap());
        root.borrow().print_tree();
    }
}
    
