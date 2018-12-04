#[macro_use]
extern crate criterion;
extern crate rand;
extern crate sorts;

use criterion::{Bencher, Criterion, ParameterizedBenchmark};
use rand::prelude::*;

/// Creates a vec with random order.
/// Vectors of the same size will have the same order.
fn get_random_vec(n: usize) -> Vec<usize> {
    let mut rng: StdRng = StdRng::seed_from_u64(42);
    let mut vec: Vec<usize> = (0..n).collect();
    vec.shuffle(&mut rng);
    vec
}

/// Writes the bench function for any sort function.
macro_rules! create_bench_fun {
    ($x:ident) => {
        |b: &mut Bencher, n: &usize| {
            let s = get_random_vec(*n);
            b.iter(|| sorts::$x(&mut s.clone()));
        }
    };
}

/// Writes the benchmark for given set sizes and sort functions.
macro_rules! create_bench {
    ($p:expr, $f:ident, $($s:ident),*) => {
        ParameterizedBenchmark::new(stringify!($f), create_bench_fun!($f), $p)
            $(
                .with_function(stringify!($s), create_bench_fun!($s))
             )*
    }
}

fn bench(c: &mut Criterion) {
    let sizes: Vec<usize> = vec![50, 100, 250, 500, 750, 1000, 1500, 2000, 2500];

    let benchmark = create_bench!(
        sizes,
        bubble_sort,
        insertion_sort,
        selection_sort,
        cocktail_sort,
        merge_sort,
        heap_sort,
        comb_sort,
        quick_sort
    );

    c.bench("sort_bench", benchmark);
}

criterion_group!(benches, bench);
criterion_main!(benches);
