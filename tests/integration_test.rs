extern crate sorts;

/// Writes a test that tests sorting input.
macro_rules! sort_test {
    ($test:ident, $sort:ident, $input:expr, $expected:expr) => {

        #[test]
        fn $test() {
            let vec = &mut $input;
            sorts::$sort(vec);
            assert_eq!(vec, &$expected);
        }
    }
}

#[cfg(test)]
mod gnome_sort {
    sort_test!(correct_sort, gnome_sort, [2, 4, 3, 1], [1, 2, 3, 4]);
}

