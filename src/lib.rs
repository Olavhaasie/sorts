//! # Rust sort collection
//! This is the documentation for `sorts`. This crate contains a
//! collection of various sorting algorithms.
pub mod bubble_sort;
pub mod heap_sort;
pub mod cocktail_sort;
pub mod gnome_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod selection_sort;

pub use bubble_sort::bubble_sort;
pub use heap_sort::heap_sort;
pub use cocktail_sort::cocktail_sort;
pub use gnome_sort::gnome_sort;
pub use insertion_sort::insertion_sort;
pub use merge_sort::merge_sort;
pub use selection_sort::selection_sort;
