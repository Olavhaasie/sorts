#![feature(test)]
extern crate rand;
extern crate sorts;
extern crate test;

use rand::prelude::*;
use sorts::merge_sort;
use test::Bencher;

#[bench]
fn bench_sort_small(b: &mut Bencher) {
    let test = vec![6, 7, 3, 4, 5, 1, 8, 9, 2];
    b.iter(|| {
        merge_sort(&mut test.clone());
    });
}

#[bench]
fn bench_sort_big(b: &mut Bencher) {
    let mut test: Vec<_> = (0..10_000).collect();
    rand::thread_rng().shuffle(&mut test);
    b.iter(|| {
        merge_sort(&mut test.clone());
    });
}
