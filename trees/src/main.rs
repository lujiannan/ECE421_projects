extern crate trees; //
use trees::rbtree::*;

// Assuming RBTree is structured properly and has an `insert` method.

fn main() {
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

fn main1() {
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

    // root.print_tree();
    // root.delete(20);
    // root.print_tree();
    // root.delete(50);
    root.print_tree();
    root.count_number_of_leaves();
    root.get_height_of_tree();
    root.is_tree_empty();
    root.print_in_order_traversal();
    root.print_pre_order_traversal();
}

fn main2() {
    // Create the root of the tree with a specific key

    let root: std::rc::Rc<std::cell::RefCell<TreeNode<u32>>> = TreeNode::new(20);
    TreeNode::regular_insert(&root, 10, NodeColor::Black);
    let p1 = TreeNode::regular_insert(&root, 30, NodeColor::Black);
    let p2 = TreeNode::regular_insert(&root, 50, NodeColor::Red);
    TreeNode::regular_insert(&root, 40, NodeColor::Red);
    TreeNode::ll_rotate(&p2.unwrap());
    TreeNode::rr_rotate(&p1.unwrap()); // need rr rotation to work for all cases not just when there
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

fn main1001() {
    // tester for LL case
    // works

    let mut myrbtree = RBTree::new();
    myrbtree.insert(20);
    myrbtree.insert(10);
    myrbtree.insert(30);
    myrbtree.insert(5);
    myrbtree.print_tree();
    // should be:
    /*
        ┌── 30 (Black)20
    ┌── 20 (Black)
    │   └── 10 (Black)20
    │       └── 5 (Red)10
            */
    myrbtree.insert(4);
    myrbtree.print_tree();
    // should be:
    /*
        ┌── 30 (Black)20
    ┌── 20 (Black)
    │   │   ┌── 10 (Red)5
    │   └── 5 (Black)20
    │       └── 4 (Red)5
            */
}

fn main1002() {
    // tester for RR case
    // works

    let mut myrbtree = RBTree::new();
    myrbtree.insert(20);
    myrbtree.insert(10);
    myrbtree.insert(30);
    myrbtree.insert(40);
    myrbtree.print_tree();
        // should be:
    /*
        ┌── 40 (Red)30
    ┌── 30 (Black)20
┌── 20 (Black)
│   └── 10 (Black)20
        */
    myrbtree.insert(50);
    myrbtree.print_tree();
        // should be:
    /*
        ┌── 50 (Red)40
    ┌── 40 (Black)20
    │   └── 30 (Red)40
┌── 20 (Black)
│   └── 10 (Black)20
        */
}

fn main1003() {
    // tester for LR case
    // works

    let mut myrbtree = RBTree::new();
    myrbtree.insert(20);
    myrbtree.insert(10);
    myrbtree.insert(30);
    myrbtree.insert(5);
    myrbtree.print_tree();
    // should be:
    /*
    ┌── 30 (Black)20
┌── 20 (Black)
│   └── 10 (Black)20
│       └── 5 (Red)10
        */
    myrbtree.insert(6);
    myrbtree.print_tree();
    // should be:
    /*
    ┌── 30 (Black)20
┌── 20 (Black)
│   │   ┌── 10 (Red)6
│   └── 6 (Black)20
│       └── 5 (Red)6
        */
}

fn main1004() {
    // tester for RL case
    // works

    let mut myrbtree = RBTree::new();
    myrbtree.insert(20);
    myrbtree.insert(10);
    myrbtree.insert(30);
    myrbtree.insert(40);
    myrbtree.print_tree();
    // should be:
    /*
        ┌── 40 (Red)30
    ┌── 30 (Black)20
┌── 20 (Black)
│   └── 10 (Black)20
        */
    myrbtree.insert(35);
    myrbtree.print_tree();
    // should be:
    /*
        ┌── 40 (Red)35
    ┌── 35 (Black)20
    │   └── 30 (Red)35
┌── 20 (Black)
│   └── 10 (Black)20
        */
}

fn main1005() {
    // tester for meta info functions (leave count, empty, height, in order traversal, pre order traversal)
    // works
    // 3- Count the number of leaves in a tree.
    // 4- Return the height of a tree.
    // 5- Print In-order traversal of the tree.
    // 6- Check if the tree is empty.
    // 7- Print the tree showing its structure. (Using println!(“{:#?}”,tree); is NOT sufficient)

    let mut myrbtree = RBTree::new();
    myrbtree.is_tree_empty(); // should be: true
    myrbtree.insert(20);
    myrbtree.is_tree_empty(); // should be: false
    myrbtree.insert(10);
    myrbtree.insert(30);
    myrbtree.insert(40);
    myrbtree.insert(50);

    myrbtree.count_number_of_leaves(); // should be: 6
    myrbtree.get_height_of_tree(); // should be: 3
    myrbtree.print_in_order_traversal(); // should be: 10 20 30 40 50
    myrbtree.is_tree_empty(); // should be: false
    myrbtree.print_pre_order_traversal(); // should be: 20 10 40 30 50 // not needed (extra feature; prints root first, then left, then right)
    myrbtree.print_tree();
    // should be:
    /*
                    ┌── 50 (Red)40
        ┌── 40 (Black)20
        │   └── 30 (Red)40
    ┌── 20 (Black)
    │   └── 10 (Black)20
        */
}
