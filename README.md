# Sorts

[![Crates.io](https://img.shields.io/crates/v/sorts.svg)](https://crates.io/crates/sorts)
![Crates.io](https://img.shields.io/crates/l/sorts.svg)
[![Crates.io](https://img.shields.io/crates/d/sorts.svg)](https://crates.io/crates/sorts)

A small sorting algorithms collection written in Rust _for learning purposes_.

## Algorithms
Below is a list of all implemented sorting algorithms. Algorithms that are
not yet crossed are on the to-do list.

- [X] Bubble sort
- [X] Merge sort
- [X] Insertion sort
- [X] Selection sort
- [X] Cocktail sort
- [ ] Gnome sort
- [ ] Bogosort
- [ ] Quicksort
- [ ] Heap sort
- [ ] Comb sort

## Benchmarking
The benchmark code is located in the `benches` directory. It can be run using

    $ cargo bench

The results are outputted to `target/criterion`. The benchmark tests the
different sorting algorithms against different problem sizes.
