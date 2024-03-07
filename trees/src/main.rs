extern crate trees; // 

use trees::rbtree::RBTree; 

fn main() {
    let mut tree = RBTree::new(5);
    // let mut tree = RBTree::new();
    tree.insert(10);
    println!("Tree with basic insertion: {:?}", tree.root);

    // need to implement command line logic for tree
    // insert, delete, count, heigh, print in order, pretty print

}

