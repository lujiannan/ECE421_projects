extern crate trees;

//
use trees::tree::*;
// use trees::tree::AVLTree;

// Assuming RBTree is structured properly and has an `insert` method.

fn main() {
    // passed_avl_example();
    main1007();
    // main2001();
    // main2002();
    // main2003();
    // main2004();
    // main2005();
    // main1005();
}

fn avl() {
    // Create the root of the tree with a specific key
    let mut root = AVLTree::new();
    root.insert(20);
    root.insert(10);
    root.insert(30);
    root.insert(25);
    root.insert(28);
    root.insert(27);
    root.insert(5);
    
    root.print_tree();
    root.delete(40);
    root.print_tree();
    root.delete(30);
    root.print_tree();
}

fn passed_avl_example() {
    // Create the root of the tree with a specific key
    let mut root = AVLTree::new();
    root.insert(20);
    root.insert(10);
    root.insert(30);
    root.insert(25);
    root.insert(28);
    root.insert(27);
    root.insert(5);
    
    root.print_tree();
}

fn passed_rb_example() {
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
    let mut root = AVLTree::new();
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
    // root.delete(20);
    // root.print_tree();
    root.delete(50);
    root.print_tree();
    // root.count_number_of_leaves();
    // root.get_height_of_tree();
    // root.is_tree_empty();
    // root.print_in_order_traversal();
    // root.print_pre_order_traversal();
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
    // myrbtree.find(30); // should be: Cannot find the 30 node, the RBTree is empty, no nodes in tree.
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
    // myrbtree.find(30); // should be: Found node: 30
    // myrbtree.find(22); // should be: Cannot find the 22 node in the RBTree.
}

fn main1006() {
    // use a blank main when running criterion benchmark tests
}

fn main1007() {
    // tester for CLI program with inputs
    println!("Welcome to our AVL or red-black tree tester program!");
    let mut program_on = true;
    while program_on {
        let mut while_stage_holder_1 = true;
        let mut tree_type_holder = String::new();
        while while_stage_holder_1 {
            println!("What tree type would you like to test, or, exit program (avl, rb, exit): ");
            let mut tree_type = String::new();
            let _ = std::io::stdin().read_line(&mut tree_type); // get keyboard input
            tree_type = tree_type.trim().to_lowercase(); // make it trimmed and lowercase
            println!("Your tree type: {}", tree_type);
            match tree_type.as_str() {
                "avl" => {
                    println!("You have chosen AVL tree.");
                    while_stage_holder_1 = false;
                    tree_type_holder = "avl".to_string();
                }
                "rb" => {
                    println!("You have chosen red-black tree.");
                    while_stage_holder_1 = false;
                    tree_type_holder = "rb".to_string();
                }
                "exit" => {
                    println!("Exiting program. Goodbye!");
                    return;
                }
                _ => {
                    println!("Invalid input, please try again.");
                }
            }
        }
        let mut while_stage_holder_2 = true;
        if tree_type_holder == "avl" {
            let mut myavltree = AVLTree::new();
            while while_stage_holder_2 {
                println!("What would you like to do with this AVL tree: ");
                println!(
                    "Commands are: insert <value>, find <value>, delete <value>, leaves (i.e. count leaves),"
                );
                println!("height (i.e. get tree height), inorder (i.e. prints in order traversal), preorder (i.e. prints pre order traversal),");
                println!("ifempty (i.e. check if tree is empty), print (i.e. print tree in structured format), exit");
                let mut input = String::new();
                let _ = std::io::stdin().read_line(&mut input); // get keyboard input
                input = input.trim().to_lowercase(); // make it trimmed and lowercase
                let parts: Vec<&str> = input.split_whitespace().collect();
                match parts[0] {
                    "insert" => {
                        if parts.len() < 2 {
                            println!("Expected a number after 'insert'");
                            continue;
                        }
                        let num = match parts[1].parse::<u32>() {
                            Ok(num) => num,
                            Err(_) => {
                                println!("Invalid input, please enter a number.");
                                continue;
                            }
                        };
                        myavltree.insert(num);
                        println!("Inserted {}", num);
                    }
                    "find" => {
                        if parts.len() < 2 {
                            println!("Expected a number after 'find'");
                            continue;
                        }
                        let num = match parts[1].parse::<u32>() {
                            Ok(num) => num,
                            Err(_) => {
                                println!("Invalid input, please enter a number.");
                                continue;
                            }
                        };
                        myavltree.print_find(num);
                    }
                    "delete" => {
                        if parts.len() < 2 {
                            println!("Expected a number after 'delete'");
                            continue;
                        }
                        let num = match parts[1].parse::<u32>() {
                            Ok(num) => num,
                            Err(_) => {
                                println!("Invalid input, please enter a number.");
                                continue;
                            }
                        };
                        myavltree.delete(num);
                        
                    }
                    "leaves" => {
                        myavltree.print_count_number_of_leaves();
                    }
                    "height" => {
                        myavltree.print_get_height_of_tree();
                    }
                    "inorder" => {
                        myavltree.print_in_order_traversal();
                    }
                    "preorder" => {
                        myavltree.print_pre_order_traversal();
                    }
                    "ifempty" => {
                        myavltree.print_is_tree_empty();
                    }
                    "print" => {
                        myavltree.print_tree();
                    }
                    "exit" => {
                        println!("Exiting program. Goodbye!");
                        program_on = false;
                        break;
                    }
                    _ => println!("Unknown command, please try again."),
                }
            }
        } else if tree_type_holder == "rb" {
            let mut myrbtree = RBTree::new();
            while while_stage_holder_2 {
                println!("What would you like to do with this Red-Black tree: ");
                println!(
                    "Commands are: insert <value>, find <value>, delete <value>, leaves (i.e. count leaves),"
                );
                println!("height (i.e. get tree height), inorder (i.e. prints in order traversal), preorder (i.e. prints pre order traversal),");
                println!("ifempty (i.e. check if tree is empty), print (i.e. print tree in structured format), exit");
                let mut input = String::new();
                let _ = std::io::stdin().read_line(&mut input); // get keyboard input
                input = input.trim().to_lowercase(); // make it trimmed and lowercase
                let parts: Vec<&str> = input.split_whitespace().collect();
                match parts[0] {
                    "insert" => {
                        if parts.len() < 2 {
                            println!("Expected a number after 'insert'");
                            continue;
                        }
                        let num = match parts[1].parse::<u32>() {
                            Ok(num) => num,
                            Err(_) => {
                                println!("Invalid input, please enter a number.");
                                continue;
                            }
                        };
                        myrbtree.insert(num);
                        println!("Inserted {}", num);
                    }
                    "find" => {
                        if parts.len() < 2 {
                            println!("Expected a number after 'find'");
                            continue;
                        }
                        let num = match parts[1].parse::<u32>() {
                            Ok(num) => num,
                            Err(_) => {
                                println!("Invalid input, please enter a number.");
                                continue;
                            }
                        };
                        myrbtree.print_find(num);
                    }
                    "delete" => {
                        if parts.len() < 2 {
                            println!("Expected a number after 'delete'");
                            continue;
                        }
                        let num = match parts[1].parse::<u32>() {
                            Ok(num) => num,
                            Err(_) => {
                                println!("Invalid input, please enter a number.");
                                continue;
                            }
                        };
                        let result = myrbtree.delete(num);
                        match result {
                            Some(ref node) => {
                                // if tree contains the key
                                println!("Found node: {:?}, deleting.", node.borrow().key);
                            },
                            None => {
                                // if tree doesn't contain the key
                                println!("Cannot find the {} node in the tree.", num);
                            }
                        }
                    }
                    "leaves" => {
                        myrbtree.print_count_number_of_leaves();
                    }
                    "height" => {
                        myrbtree.print_get_height_of_tree();
                    }
                    "inorder" => {
                        myrbtree.print_in_order_traversal();
                    }
                    "preorder" => {
                        myrbtree.print_pre_order_traversal();
                    }
                    "ifempty" => {
                        myrbtree.print_is_tree_empty();
                    }
                    "print" => {
                        myrbtree.print_tree();
                    }
                    "exit" => {
                        println!("Exiting program. Goodbye!");
                        program_on = false;
                        break;
                    }
                    _ => println!("Unknown command, please try again."),
                }
            }
        }
    }
}

fn main2001() {
    // tester for LL case , avl tree
    //works

    let mut mytree = AVLTree::new();
    mytree.insert(20);
    mytree.insert(10);
    mytree.insert(30);
    mytree.insert(5);
    mytree.print_tree();
    // should be:
    /*
        ┌── 30(1)
    ┌── 20(3)
    │   └── 10(2)
    │       └── 5(1)
            */
    mytree.insert(4);
    mytree.print_tree();
    // should be:
    /*
        ┌── 30(1)
    ┌── 20(3)
    │   │   ┌── 10(1)
    │   └── 5(2)
    │       └── 4(1)
            */
}

fn main2002() {
    // tester for RR case, avl tree
    //works

    let mut mytree = AVLTree::new();
    mytree.insert(20);
    mytree.insert(10);
    mytree.insert(30);
    mytree.insert(40);
    mytree.print_tree();
    // should be:
    /*
            ┌── 40(1)
        ┌── 30(2)
    ┌── 20(3)
    │   └── 10(1)
            */
    mytree.insert(50);
    mytree.print_tree();
    // should be:
    /*
            ┌── 50(1)
        ┌── 40(2)
        │   └── 30(1)
    ┌── 20(3)
    │   └── 10(1)
            */
}

fn main2003() {
    // tester for LR case, avl tree
    //works

    let mut mytree = AVLTree::new();
    mytree.insert(20);
    mytree.insert(10);
    mytree.insert(30);
    mytree.insert(5);
    mytree.print_tree();
    // should be:
    /*
        ┌── 30(1)
    ┌── 20(3)
    │   └── 10(2)
    │       └── 5(1)
            */
    mytree.insert(6);
    mytree.print_tree();
    // should be:
    /*
        ┌── 30(1)
    ┌── 20(3)
    │   │   ┌── 10(1)
    │   └── 6(2)
    │       └── 5(1)
            */
}

fn main2004() {
    // tester for RL case, avl tree
    //works

    let mut mytree = AVLTree::new();
    mytree.insert(20);
    mytree.insert(10);
    mytree.insert(30);
    mytree.insert(40);
    mytree.print_tree();
    // should be:
    /*
            ┌── 40(1)
        ┌── 30(2)
    ┌── 20(3)
    │   └── 10(1)
            */
    mytree.insert(35);
    mytree.print_tree();
    // should be:
    /*
            ┌── 40(1)
        ┌── 35(2)
        │   └── 30(1)
    ┌── 20(3)
    │   └── 10(1)
            */
}

fn main2005() {
    // avl tree //works
    // tester for meta info functions (leave count, empty, height, in order traversal, pre order traversal)
    
    // 3- Count the number of leaves in a tree.
    // 4- Return the height of a tree.
    // 5- Print In-order traversal of the tree.
    // 6- Check if the tree is empty.
    // 7- Print the tree showing its structure. (Using println!(“{:#?}”,tree); is NOT sufficient)

    let mut mytree = AVLTree::new();
    mytree.print_find(30); // should be: Cannot find the 30 node, the RBTree is empty, no nodes in tree.
    mytree.print_is_tree_empty(); // should be: true
    mytree.insert(20);
    mytree.print_is_tree_empty(); // should be: false
    mytree.insert(10);
    mytree.insert(30);
    mytree.insert(40);
    mytree.insert(50);

    // mytree.count_number_of_leaves(); // should be: 6
    // mytree.get_height_of_tree(); // should be: 3
    // mytree.print_in_order_traversal(); // should be: 10 20 30 40 50
    // mytree.is_tree_empty(); // should be: false
    // mytree.print_pre_order_traversal(); // should be: 20 10 40 30 50 // not needed (extra feature; prints root first, then left, then right)
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
    mytree.print_find(22); // should be: Cannot find the 22 node in the RBTree.
    mytree.insert(60);
    mytree.insert(70);
    mytree.insert(80);
    mytree.insert(90);
    mytree.insert(100);
    mytree.insert(110);
    mytree.print_in_order_traversal(); // should be: 10 20 30 40 50 60 70 80 90 100 110
    mytree.print_pre_order_traversal(); // should be: 40 20 10 30 80 60 50 70 100 90 110 // not needed (extra feature; prints root first, then left, then right)
    mytree.print_tree();
    // should be:
    /*
                ┌── 110(1)
            ┌── 100(2)
            │   └── 90(1)
        ┌── 80(3)
        │   │   ┌── 70(1)
        │   └── 60(2)
        │       └── 50(1)
    ┌── 40(4)
    │   │   ┌── 30(1)
    │   └── 20(2)
    │       └── 10(1)
        */
    mytree.insert(33);
    mytree.insert(34);
    mytree.insert(35);
    mytree.insert(36);
    mytree.insert(39);
    mytree.insert(38);
    mytree.insert(37);
    mytree.insert(49);
    mytree.insert(48);
    mytree.insert(47);
    mytree.insert(41);
    mytree.insert(42);
    mytree.insert(43);
    mytree.insert(44);
    mytree.print_in_order_traversal(); // should be: 10 20 30 33 34 35 36 37 38 39 40 41 42 43 44 47 48 49 50 60 70 80 90 100 110
    mytree.print_pre_order_traversal(); // should be: 40 33 20 10 30 36 35 34 38 37 39 49 43 42 41 47 44 48 80 60 50 70 100 90 110
    mytree.print_tree();
    // should be:
    /*
                    ┌── 110(1)
                ┌── 100(2)
                │   └── 90(1)
            ┌── 80(3)
            │   │   ┌── 70(1)
            │   └── 60(2)
            │       └── 50(1)
        ┌── 49(4)
        │   │       ┌── 48(1)
        │   │   ┌── 47(2)
        │   │   │   └── 44(1)
        │   └── 43(3)
        │       └── 42(2)
        │           └── 41(1)
    ┌── 40(5)
    │   │           ┌── 39(1)
    │   │       ┌── 38(2)
    │   │       │   └── 37(1)
    │   │   ┌── 36(3)
    │   │   │   └── 35(2)
    │   │   │       └── 34(1)
    │   └── 33(4)
    │       │   ┌── 30(1)
    │       └── 20(2)
    │           └── 10(1)
        */

}
