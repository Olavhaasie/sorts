extern crate sorts;

/// Writes a test that tests sorting input.
macro_rules! sort_test {
    ($test:ident, $sort:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $test() {
            let vec: &mut [usize] = &mut $input;
            sorts::$sort(vec);
            assert_eq!(vec, &$expected);
        }
    };
}

macro_rules! sort_tests {
    ($($sort:ident),*) => {
        $(
        #[cfg(test)]
        mod $sort {
            sort_test!(sort_even, $sort, [2, 4, 3, 1], [1, 2, 3, 4]);
            sort_test!(sort_odd, $sort, [2, 4, 3, 1, 0], [0, 1, 2, 3, 4]);
            sort_test!(sort_sorted, $sort, [3, 4, 5, 6, 7], [3, 4, 5, 6, 7]);
            sort_test!(sort_reversed, $sort, [9, 8, 7, 6, 5], [5, 6, 7, 8, 9]);
            sort_test!(sort_empty, $sort, [], []);
            sort_test!(sort_one, $sort, [100], [100]);
            sort_test!(sort_two, $sort, [9, 8], [8, 9]);
        }
        )*
    }
}

sort_tests! {
    bubble_sort,
    cocktail_sort,
    gnome_sort,
    insertion_sort,
    selection_sort,
    merge_sort,
    heap_sort,
    comb_sort
}
