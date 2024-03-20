// * update with avl tree crate and functions

// * just copy this file to 'benches' folder and run 'cargo bench' to see the results
/*
the Cargo.toml file should have:
[dependencies]
rand = "0.8.4"

[dev-dependencies]
criterion = { version = "0.5.1", features = ["html_reports"] }

[[bench]]
name = "my_benchmark"
harness = false


 */

 use criterion::{black_box, criterion_group, criterion_main, Criterion};
 use rand::Rng;
 
 extern crate trees;
 use trees::rbtree::*;
 
 fn criterion_benchmark(c: &mut Criterion) {
     // change the num_limit to 10,000, 40,000, 70,000, 100,000, 130,000
     let num_limit = 10000; // 10,000, 40,000, 70,000, 100,000, 130,000
     let mut rng = rand::thread_rng();
     let mut test_values: Vec<u32> = (0..num_limit)
         .map(|_| rng.gen_range(1..num_limit))
         .collect();
 
     // test style [start]: insert (comment out the other test styles to test this one)
     // insert 'num_limit' of values into tree
     c.bench_function("RBTree insert: ", |b| {
         b.iter(|| {
             let mut rbtree = RBTree::new();
             for &item in &test_values {
                 rbtree.insert(item);
             }
         })
     });
     // test style [end]: insert
 
     // test style [start]: search known (comment out the other test styles to test this one)
     // search for 'num_limit' of known values in tree
     let mut rbtree = RBTree::new();
     for &item in &test_values {
         rbtree.insert(item);
     }
     // search for lowest 'num_limit/10' of values in tree
     // (option of known or unknown values:
     // 'unknown' means they may not all be in the tree,
     // 'known' means they are for sure in the tree)
     let mut lower_test_values_known = test_values.clone();
     lower_test_values_known.sort();
     let lower_test_values_known: Vec<_> = lower_test_values_known
         .iter()
         .take((num_limit / 10).try_into().unwrap())
         .cloned()
         .collect();
     c.bench_function("RBTree search known: ", |b| {
         b.iter(|| {
             for &item in &lower_test_values_known {
                 rbtree.find(item);
             }
         })
     });
     // test style [end]: search known
 
     // test style [start]: search unknown (comment out the other test styles to test this one)
     // search for 'num_limit' of unknown values in tree
     let mut rbtree = RBTree::new();
     for &item in &test_values {
         rbtree.insert(item);
     }
     // search for lowest 'num_limit/10' of values in tree
     // (option of known or unknown values:
     // 'unknown' means they may not all be in the tree,
     // 'known' means they are for sure in the tree)
     let mut lower_test_values_unknown: Vec<u32> = (0..(num_limit / 10))
         .map(|_| rng.gen_range(1..(num_limit / 10)))
         .collect();
     c.bench_function("RBTree search unknown: ", |b| {
         b.iter(|| {
             for &item in &lower_test_values_unknown {
                 rbtree.find(item);
             }
         })
     });
     // test style [end]: search unknown
 }
 
 criterion_group!(benches, criterion_benchmark);
 criterion_main!(benches);
 
 /*

 
 */
 