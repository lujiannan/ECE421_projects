extern crate trees; // 
use trees::rbtree::{NodeColor, TreeNode, RBTree};

// Assuming RBTree is structured properly and has an `insert` method.

fn main() {
    // Create the root of the tree with a specific key
    let mut root = RBTree::new();
    root.insert(50);
    root.insert(25);
    root.insert(100);
    root.insert(15);
    root.insert(10);
    root.insert(20);
    root.insert(17);
    root.insert(12);
    root.insert(5);
    // let pointer = root.insert(200);
    root.print_tree();
    root.delete(25);
    root.print_tree();
    root.delete(15);
    root.print_tree();
    
    // let case = pointer.unwrap().borrow().determine_case();
    // println!("{}", case);



    // let root: std::rc::Rc<std::cell::RefCell<TreeNode<u32>>> = TreeNode::new(50);
    // TreeNode::regular_insert(&root, 25, NodeColor::Red);
    // TreeNode::regular_insert(&root, 100, NodeColor::Red);
    // let new = TreeNode::regular_insert(&root, 200, NodeColor::Red);

    // let result = TreeNode::rl_rotate(&root);
    // let case = new.unwrap().borrow().determine_case();
    // println!("{}", case);
    // result.unwrap().borrow().print_tree();
    // let result = new.unwrap().borrow().determine_rotation();
    // println!("{}", result);

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