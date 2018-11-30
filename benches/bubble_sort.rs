#![feature(test)]
extern crate test;
extern crate rust_sort;

use rust_sort::bubble_sort;
use test::Bencher;

#[bench]
fn bench_sorting_sorted(b: &mut Bencher) {
    let test = &mut [1,2,3,4,5,6,7,8,9];
    b.iter(|| {
        bubble_sort(test);
    });
}
