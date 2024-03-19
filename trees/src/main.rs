extern crate trees; // 
use trees::rbtree::{NodeColor, TreeNode, RBTree};

// Assuming RBTree is structured properly and has an `insert` method.

fn main() {
    // Create the root of the tree with a specific key
    let mut root = RBTree::new();
    root.r_insert(20, NodeColor::Black);
    root.r_insert(10, NodeColor::Black);
    root.r_insert(40, NodeColor::Red);
    root.r_insert(30, NodeColor::Black);
    root.r_insert(60, NodeColor::Black);
    root.r_insert(50, NodeColor::Black);
    root.r_insert(70, NodeColor::Black);
    root.insert(80);
    // root.insert(4);
    // root.insert(8);
    // root.print_tree();
    // root.insert(40);
    // root.insert(60);
    // root.insert(70);
    // root.insert(80);
    // root.insert(4);
    // root.insert(8);
    // root.print_tree();
}

fn main5() {
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
    root.print_tree();
    root.delete(25);
    root.print_tree();
    root.delete(15);
    root.print_tree();
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
    


fn main4() {
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