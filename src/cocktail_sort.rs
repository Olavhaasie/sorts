/// Sorts a slice in-place using
/// [cocktail sort](https://en.wikipedia.org/wiki/Cocktail_shaker_sort).
///
/// Cocktail sort is a variation of bubble sort. The difference is that bubble
/// sort only makes forward passes, whereas cocktail sort goes back and forth,
/// like a cocktail shaker. In practice cocktail sort is faster than bubble
/// most of the time.  A small optimization can be made where the last swap
/// position is remembered, since all elements beyond that point are already
/// sorted.
///
/// # Examples
/// ```
/// let mut slice = vec![2,3,4,5,1];
/// sorts::cocktail_sort(&mut slice);
/// assert_eq!(slice, &[1,2,3,4,5]);
/// ```
pub fn cocktail_sort<T: PartialOrd>(s: &mut [T]) {
    let len = s.len();
    // Check if slice is shorter than 2, because if it's empty we get
    // substraction with overflow.
    if len < 2 {
        return;
    }

    let mut begin = 0;
    let mut end = len - 1;
    while begin < end {
        let range = begin..end;
        end = begin;
        for i in range {
            if s[i] > s[i + 1] {
                s.swap(i, i + 1);
                end = i;
            }
        }

        let range = (begin..end).rev();
        begin = end;
        for i in range {
            if s[i] > s[i + 1] {
                s.swap(i, i + 1);
                begin = i;
            }
        }
    }
}
