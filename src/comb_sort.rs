/// Sorts a slice in-place using
/// [comb sort](https://en.wikipedia.org/wiki/Comb_sort).
///
/// Comb sort is a simple algorithms that improves on bubble sort. It does so
/// by eliminating small values at the end of the list quickly, since these
/// slow down bubble sort. This is solved by comparing elements at the
/// beginning of the list to the end with a so-called gap. As the algorithm
/// the gap size shrinks until it is finally 1, where it is the same as bubble
/// sort.
///
/// # Examples
/// ```
/// let mut vec = vec![9, 7, 8, 5, 1];
/// sorts::comb_sort(&mut vec);
/// assert_eq!(vec, &[1, 5, 7, 8, 9]);
/// ```
pub fn comb_sort<T: PartialOrd>(s: &mut [T]) {
    let len = s.len();
    let inv_shrink: f32 = 1.0 / 1.3;

    let mut gap = len;
    let mut sorted = len < 2;

    while !sorted {
        gap = (gap as f32 * inv_shrink).floor() as usize;

        if gap <= 1 {
            gap = 1;
            sorted = true;
        }

        for i in 0..len - gap {
            if s[i] > s[i + gap] {
                s.swap(i, i + gap);
                sorted = false;
            }
        }
    }
}
