use bench_test::{
    sort_arr,
    sorting_trait::{self, combinator},
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn sort_arr_benchmark(c: &mut Criterion) {
    let mut arr = black_box([6, 2, 4, 1, 9, -2, 5]);

    c.bench_function("sorting algorithm", |b| b.iter(|| sort_arr(&mut arr)));
}

fn sort_arr_benchmark_stuct_impl(c: &mut Criterion) {
    let mut file = include_str!("../../data.txt");
    // let mut arr2 = black_box(combinator(file));

    c.bench_function("trait_sort__sorting algorithm", |b| {
        b.iter(|| combinator(file))
    });
}

criterion_group!(benches, sort_arr_benchmark, sort_arr_benchmark_stuct_impl);
criterion_main!(benches);
