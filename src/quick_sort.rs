pub fn quick_sort<T: PartialOrd>(s: &mut [T]) {
    if s.len() > 1 {
        let pivot = lomuto_partition(s);
        quick_sort(&mut s[..pivot]);
        quick_sort(&mut s[pivot + 1..]);
    }
}

/// Partitions a slice according to the
/// [Lomuto partition scheme](https://en.wikipedia.org/wiki/Quicksort#Lomuto_partition_scheme).
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
