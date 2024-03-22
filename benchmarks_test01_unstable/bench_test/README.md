# Result

```bash

$ cargo bench
warning: `bench_test` (bench "sorting_benchmark") generated 2 warnings (run `cargo fix --bench "sorting_benchmark"` to apply 2 suggestions)
    Finished bench [optimized] target(s) in 0.91s
     Running unittests src/lib.rs (target/release/deps/bench_test-4fda61c62655144c)

running 2 tests
test sorting::tests::test_bubble_sort ... ignored
test sorting::tests::test_selection_sort ... ignored

test result: ok. 0 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/sorting_benchmark.rs (target/release/deps/sorting_benchmark-7a040d163772e8d6)
trait_sort__sorting algorithm
                        time:   [146.10 ns 146.20 ns 146.30 ns]
                        change: [-0.0521% +0.1501% +0.3689%] (p = 0.16 > 0.05)
                        No change in performance detected.
Found 14 outliers among 100 measurements (14.00%)
  3 (3.00%) low severe
  5 (5.00%) high mild
  6 (6.00%) high severe

trait_FromStr__sorting algorithm
                        time:   [310.45 ps 310.52 ps 310.59 ps]
                        change: [-0.1904% -0.0010% +0.1857%] (p = 1.00 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe

trait_FromStr__unstable_sorting algorithm
                        time:   [310.39 ps 310.47 ps 310.56 ps]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
```

