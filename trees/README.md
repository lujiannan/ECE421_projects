# Our crate Set of Binary Trees (Red-Black tree and AVL tree)
- ECE421 Group Project 2, due Mar 20, 2024  
- Version: 1.0.0?
- Authors: Prabh Kooner, Jiannan Lu, Brandon Hoynick

## Get Started – How to add crate to your project:
1. Copy the crate file to your project: Copy ‘lib.rs’ to project’s ‘src’ folder.
2. In your project’s ‘main.rs’ (for example), add ‘extern crate trees;’ to top; 
then add ‘use trees::tree::*;’ for RB trees or for AVL trees.
3. Begin using supplied structs and associated commands. The example snippet below contains all high level commands and some print out results; note that self-printing commands begin with print (removing these will return functional result instead of print).
    let mut myrbtree = RBTree::new();
    myrbtree.print_find(30); // should be: Cannot find the 30 node in the tree.
    myrbtree.print_is_tree_empty(); // should be: true
    myrbtree.insert(20);
    myrbtree.print_is_tree_empty(); // should be: false
    myrbtree.insert(10);
    myrbtree.insert(30);
    myrbtree.insert(40);
    myrbtree.insert(50);
    myrbtree.print_count_number_of_leaves(); // should be: 6
    myrbtree.print_get_height_of_tree(); // should be: 3
    myrbtree.print_in_order_traversal(); // should be: 10 20 30 40 50
    myrbtree.print_is_tree_empty(); // should be: false
    myrbtree.print_pre_order_traversal(); // should be: 20 10 40 30 50
    myrbtree.print_tree();
    // should be:
            ┌── 50 (Red)
        ┌── 40 (Black)
        │   └── 30 (Red)
    ┌── 20 (Black)
    │   └── 10 (Black)
    myrbtree.print_find(30); // should be: Found node: 30
    myrbtree.print_find(22); // should be: Cannot find the 22 node in the RBTree.
    myrbtree.print_delete(50); // should be: Found node: 50, deleting.
    myrbtree.print_tree();
    // should be:
        ┌── 40 (Black)
        │   └── 30 (Red)
    ┌── 20 (Black)
    │   └── 10 (Black)
    myrbtree.print_delete(50); // should be: Cannot find the 50 node in the tree.
    let mut mytree = AVLTree::new();
    mytree.print_find(30); // should be: Cannot find the 30 node in the tree.
    mytree.print_is_tree_empty(); // should be: true
    mytree.insert(20);
    mytree.print_is_tree_empty(); // should be: false
    mytree.insert(10);
    mytree.insert(30);
    mytree.insert(40);
    mytree.insert(50);
    mytree.print_count_number_of_leaves(); // should be: 6
    mytree.print_get_height_of_tree(); // should be: 3
    mytree.print_in_order_traversal(); // should be: 10 20 30 40 50
    mytree.print_is_tree_empty(); // should be: false
    mytree.print_pre_order_traversal(); // should be: 20 10 40 30 50 
    mytree.print_tree();
    // should be:
    /*
            ┌── 50(1)
        ┌── 40(2)
        │   └── 30(1)
    ┌── 20(3)
    │   └── 10(1)
        */
    mytree.print_find(30); // should be: Found node: 30
    mytree.print_find(22); // should be: Cannot find the 22 node in the tree.
    mytree.print_delete(50); // should be: Found node: 50, deleting.
    mytree.print_tree();
    // should be:
    /*
        ┌── 40(2)
        │   └── 30(1)
    ┌── 20(3)
    │   └── 10(1)
        */
    mytree.print_delete(50); // should be: Cannot find the 50 node in the tree.

## Program-based Tree Tester – Users can optionally execute our ‘trees_tester.exe’ file to get program to test both trees. The following will show how to test crate structs and functions through a demo executable:
1. Copy the crate executable file (‘trees_ tester.exe’) to folder location of your choice. 
2. Navigate/Open a terminal to location of ‘trees_tester.exe’ and execute file by running ‘./trees_ tester’ or ‘trees_ tester’ (whatever command your OS uses to run executables).
3. The program will prompt you to chose to build a Red-Black tree or AVL tree.
4. After selecting, you can build/modify your tree with various commands (and there are prompts for exiting too). The commands are:
•	insert <value> : Inserts the <value> into the tree; (duplicates are skipped).
•	find <value> : Finds the <value> from tree; (queried values that were not present in the tree will print a message stating so).
•	delete <value> : Deletes the <value> from tree; (queried values that were not present in the tree will print a message stating so).
•	leaves : Counts the number of leaves (NULL nodes) in the tree.
•	height : Counts the longest strip of nodes (from root to ends).
•	inorder : Prints the in-order traversal (from left most child of whole tree, to right most child of whole tree) of tree’s node values; this print out is essentially the same as printing all tree’s values in an ascending sort.
•	preorder : Prints the pre-order traversal, from root downward to left child to right child, of tree’s node values; this print out is essentially the same as printing all tree’s values from top row downward (grabbing the left most subtrees first).
•	ifempty : This checks if tree is empty (i.e. has no nodes; is just a root pointer).
•	print : Prints tree in structured format, where the printout shows the root pointer, the connected node’s values (and other attributes like colour and parent value), and line connections between nodes.
•	exit : Exits the program.
