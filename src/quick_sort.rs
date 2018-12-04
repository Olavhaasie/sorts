/// Sorts a slice in-place using
/// [quicksort](https://en.wikipedia.org/wiki/Quicksort).
///
/// Quicksort can be compared to merge sort as it also is a divide-and-conquer
/// algorithm. However, quicksort does all the heavy work before the recursive
/// calls, so it could also be called a conquer-and-divide algorithm. This
/// implementation uses the [Lomuto partition scheme](https://en.wikipedia.org/wiki/Quicksort#Lomuto_partition_scheme).
///
/// # Examples
/// ```
/// let mut vec = vec![0, -1, -2, -3,];
/// sorts::quick_sort(&mut vec);
/// assert_eq!(vec, &[-3, -2, -1, 0]);
/// ```
pub fn quick_sort<T: PartialOrd>(s: &mut [T]) {
    if s.len() > 1 {
        let pivot = lomuto_partition(s);
        quick_sort(&mut s[..pivot]);
        quick_sort(&mut s[pivot + 1..]);
    }
}

/// Partitions a slice according to the Lomuta partition scheme.
pub fn lomuto_partition<T: PartialOrd>(s: &mut [T]) -> usize {
    let pivot = s.len() - 1;
    let mut swap = 0;
    for i in 0..pivot {
        if s[i] < s[pivot] {
            if swap != i {
                s.swap(swap, i);
            }
            swap += 1;
        }
    }

    if swap != pivot {
        s.swap(swap, pivot);
    }
    swap
}
