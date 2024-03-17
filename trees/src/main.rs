extern crate trees; // 
use trees::rbtree::{NodeColor, TreeNode};

// Assuming RBTree is structured properly and has an `insert` method.

fn main2() {
    let root: std::rc::Rc<std::cell::RefCell<TreeNode<u32>>> = TreeNode::new(0);
    let pointer = TreeNode::regular_insert(&root, 50, NodeColor::Black);
    TreeNode::regular_insert(&root, 25, NodeColor::Red);
    TreeNode::regular_insert(&root, 100, NodeColor::Black);;
    TreeNode::regular_insert(&root, 40, NodeColor::Red);
    let lr = TreeNode::lr_rotate(&pointer.unwrap());
    // lr.unwrap().borrow().print_tree();
    root.borrow().print_tree();
}






fn main() {
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

fn main3() {
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

    println!("number of leaves tree1 : {}", TreeNode::count_number_of_leaves(&root));
    println!("is tree1 empty: {}", TreeNode::is_tree_empty(&root));
    let root2: std::rc::Rc<std::cell::RefCell<TreeNode<u32>>> = TreeNode::new(998);
    println!("is tree2 empty: {}", TreeNode::is_tree_empty(&root2));
    println!("height of tree1: {}", TreeNode::get_height_of_tree(&root));
    println!("height of tree2: {}", TreeNode::get_height_of_tree(&root2));
    
}