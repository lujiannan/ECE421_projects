extern crate trees; // 

use trees::rbtree::RBTree; 
use trees::rbtree::TreeNode;

// Assuming RBTree is structured properly and has an `insert` method.
fn main() {
    let mut tree = RBTree::new(20);
    tree.insert(10);
    tree.insert(30);
    tree.insert(5);
    tree.insert(15);
    tree.insert(25);
    tree.insert(35);
    if let Some(root) = tree.root {
        root.borrow().print_tree(); // Assuming tree.root is of type Option<Rc<RefCell<TreeNode<u32>>>>
    }
}



// fn main() {
//     let mut root = TreeNode::new(20);
//     let c = TreeNode::insert(&root, 5);

//     if let Some(node_rc) = c {
//         let node = node_rc.borrow();
//         println!("{:?}", node);

//         if let Some(parent_key) = node.get_parent_key() {
//             println!("Parent key: {}", parent_key);
//         } else {
//             println!("This node has no parent.");
//         }

//     } else {
//         println!("No node was appended.");
//     }
// }
