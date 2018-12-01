#[macro_use]
extern crate criterion;
extern crate rand;
extern crate sorts;

use criterion::{Bencher, Criterion, ParameterizedBenchmark};
use rand::prelude::*;

fn get_random_vec(n: usize) -> Vec<usize> {
    let mut rng: StdRng = StdRng::seed_from_u64(42);
    let mut vec: Vec<usize> = (0..n).collect();
    vec.shuffle(&mut rng);
    vec
}

macro_rules! create_bench {
    ($x:ident) => {
        |b: &mut Bencher, n: &usize| {
            let s = get_random_vec(*n);
            b.iter(|| sorts::$x(&mut s.clone()));
        }
    }
}

fn bench(c: &mut Criterion) {
    let params: Vec<usize> = vec![
        10,
        50,
        100,
        500,
        1000,
    ];

    let bubble = create_bench!(bubble_sort);
    let insertion = create_bench!(insertion_sort);
    let selection = create_bench!(selection_sort);
    let cocktail = create_bench!(cocktail_sort);
    let merge = create_bench!(merge_sort);

    let benchmark = ParameterizedBenchmark::new("bubble_sort", bubble, params)
        .with_function("insertion_sort", insertion)
        .with_function("selection_sort", selection)
        .with_function("cocktail_sort", cocktail)
        .with_function("merge_sort", merge);

    c.bench("sort_bench", benchmark);
}

criterion_group!(benches, bench);
criterion_main!(benches);
