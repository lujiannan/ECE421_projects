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
* 10,000 insert result:
    Finished bench [optimized] target(s) in 54.40s
     Running unittests src\lib.rs (target\release\deps\trees-07f23b5942000055.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\main.rs (target\release\deps\trees-1ff8a5ef67b321fb.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches\my_benchmark.rs (target\release\deps\my_benchmark-69c07faf983ee104.exe)
Gnuplot not found, using plotters backend
RBTree insert:          time:   [2.5256 ms 2.5342 ms 2.5442 ms]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe

* 40,000 insert result:
     Running unittests src\lib.rs (target\release\deps\trees-07f23b5942000055.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\main.rs (target\release\deps\trees-1ff8a5ef67b321fb.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches\my_benchmark.rs (target\release\deps\my_benchmark-69c07faf983ee104.exe)
Gnuplot not found, using plotters backend
RBTree insert:          time:   [15.025 ms 15.199 ms 15.410 ms]
                        change: [+491.93% +499.76% +508.45%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe

* 70,000 insert result:
     Running unittests src\lib.rs (target\release\deps\trees-07f23b5942000055.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\main.rs (target\release\deps\trees-1ff8a5ef67b321fb.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches\my_benchmark.rs (target\release\deps\my_benchmark-69c07faf983ee104.exe)
Gnuplot not found, using plotters backend
RBTree insert:          time:   [30.567 ms 30.778 ms 31.021 ms]
                        change: [+99.365% +102.49% +105.27%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

* 100,000 insert result:
     Running unittests src\lib.rs (target\release\deps\trees-07f23b5942000055.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\main.rs (target\release\deps\trees-1ff8a5ef67b321fb.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches\my_benchmark.rs (target\release\deps\my_benchmark-69c07faf983ee104.exe)
Gnuplot not found, using plotters backend
Benchmarking RBTree insert: : Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.3s, or reduce sample count to 90.
RBTree insert:          time:   [52.220 ms 52.766 ms 53.350 ms]
                        change: [+69.220% +71.444% +73.732%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild

* 130,000 insert result:
     Running unittests src\lib.rs (target\release\deps\trees-07f23b5942000055.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\main.rs (target\release\deps\trees-1ff8a5ef67b321fb.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches\my_benchmark.rs (target\release\deps\my_benchmark-69c07faf983ee104.exe)
Gnuplot not found, using plotters backend
Benchmarking RBTree insert: : Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 7.5s, or reduce sample count to 60.
RBTree insert:          time:   [73.962 ms 74.450 ms 74.991 ms]
                        change: [+39.321% +41.094% +42.888%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe

*/
