use bench_test::{sort_arr, sorting_trait::combinator, trait_fromstr, trait_fromstr_unstable};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// fn sort_arr_benchmark(c: &mut Criterion) {
//     let mut arr = black_box([6, 2, 4, 1, 9, -2, 5]);

//     c.bench_function("sorting algorithm", |b| b.iter(|| sort_arr(&mut arr)));
// }

fn sort_arr_benchmark_stuct_impl(c: &mut Criterion) {
    let file = include_str!("../../data.txt");
    // let mut arr2 = black_box(combinator(file));

    c.bench_function("trait_sort__sorting algorithm", |b| {
        b.iter(|| combinator(file))
    });
}

fn sort_arr_benchmark_trait_fromstr(c: &mut Criterion) {
    use std::str::FromStr;
    let file = include_str!("../../data.txt");
    let elves = trait_fromstr::Elves::from_str(file).expect("Failed to parse file");
    let sum = elves.get_fat_sum(3);

    c.bench_function("trait_FromStr__sorting algorithm", |b| b.iter(|| sum));
}

fn sort_arr_benchmark_trait_fromstr_unstable(c: &mut Criterion) {
    use std::str::FromStr;
    let file = include_str!("../../data.txt");
    let elves = trait_fromstr_unstable::Elves::from_str(file).expect("Failed to parse file");
    let sum = elves.get_fat_sum(3);

    c.bench_function("trait_FromStr__unstable_sorting algorithm", |b| {
        b.iter(|| sum)
    });
}

criterion_group!(
    benches,
    // sort_arr_benchmark,
    sort_arr_benchmark_stuct_impl,
    sort_arr_benchmark_trait_fromstr,
    sort_arr_benchmark_trait_fromstr_unstable
);
criterion_main!(benches);
