//! # Rust sort collection
//! This is the documentation for `sorts`. This crate contains a
//! collection of various sorting algorithms.
pub mod bubble_sort;
pub mod cocktail_sort;
pub mod comb_sort;
pub mod gnome_sort;
pub mod heap_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod selection_sort;
pub mod shell_sort;

pub use self::bubble_sort::bubble_sort;
pub use self::cocktail_sort::cocktail_sort;
pub use self::comb_sort::comb_sort;
pub use self::gnome_sort::gnome_sort;
pub use self::heap_sort::heap_sort;
pub use self::insertion_sort::insertion_sort;
pub use self::merge_sort::merge_sort;
pub use self::quick_sort::quick_sort;
pub use self::selection_sort::selection_sort;
pub use self::shell_sort::shell_sort;
