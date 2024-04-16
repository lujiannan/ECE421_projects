pub fn rotate_ll(parent: &Tree) {
        if let Some(ref pl) = parent.borrow().left {
            // Temporarily store PL's right child
            let plr = pl.borrow().right.clone();

            // Update PL's right child to the parent node
            pl.borrow_mut().right = Some(parent.clone());
            let parent_weak = Rc::downgrade(pl); // PL will be the new parent of P
            parent.borrow_mut().parent = Some(parent_weak);

            // Update parent's left child to PLR
            parent.borrow_mut().left = plr.clone();
            if let Some(ref plr_node) = plr {
                plr_node.borrow_mut().parent = Some(Rc::downgrade(parent));
            }

            // If the parent had a parent, update its child pointer
            let parent_parent_weak = parent.borrow().parent.clone();
            if let Some(parent_parent) = parent_parent_weak {
                if let Some(pp) = parent_parent.upgrade() {
                    if pp.borrow().left.as_ref() == Some(parent) {
                        pp.borrow_mut().left = Some(pl.clone());
                    } else {
                        pp.borrow_mut().right = Some(pl.clone());
                    }
                    // Update PL's parent to P's parent
                    pl.borrow_mut().parent = Some(Rc::downgrade(&pp));
                }
            } else {
                // If the parent was the root, PL now becomes the root and should have no parent
                pl.borrow_mut().parent = None;
            }
        }
    }

    pub fn ll_rotate(node: &Tree) {
        let node_left = node.borrow().left.clone(); // Clone the left child of the unbalanced node

        if let Some(node_left) = node_left {
            let node_left_right = node_left.borrow().right.clone(); // Get the right child of the left node

            // Step 1: Set the right child of the left node as the left child of the current node
            node.borrow_mut().left = node_left_right;
            if let Some(ref node_left_right) = node_left_right {
                node_left_right.borrow_mut().parent = Some(Rc::downgrade(node));
            }

            // Step 2: Update the parent of the left node to be the parent of the current node
            let parent_weak = node.borrow().parent.clone();
            node_left.borrow_mut().parent = parent_weak.clone();

            if parent_weak.is_none() {
                // The current node is the root
                // You might need to update the root of the tree outside this function,
                // as we cannot access the tree structure here.
            } else if let Some(parent) = parent_weak.and_then(|w| w.upgrade()) {
                // If the current node is not the root, update its parent's child pointer
                if parent.borrow().left.as_ref() == Some(node) {
                    parent.borrow_mut().left = Some(node_left.clone());
                } else {
                    parent.borrow_mut().right = Some(node_left.clone());
                }
            }

            // Step 3: Set the current node as the right child of the left node
            node_left.borrow_mut().right = Some(node.clone());
            node.borrow_mut().parent = Some(Rc::downgrade(&node_left));
        }
    }