//! # Rust sort collection
//! This is the documentation for `sorts`. This crate contains a
//! collection of various sorting algorithms.
pub mod bubble_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod selection_sort;

pub use bubble_sort::bubble_sort;
pub use insertion_sort::insertion_sort;
pub use merge_sort::merge_sort;
pub use selection_sort::selection_sort;
