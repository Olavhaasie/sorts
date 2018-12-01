/// Sorts a slice using
/// [merge sort](https://en.wikipedia.org/wiki/Merge_sort).
///
/// # Examples
/// ```
/// let mut slice = vec![3,2,1,4];
/// sorts::merge_sort(&mut slice);
/// assert_eq!(slice, &[1,2,3,4]);
/// ```
pub fn merge_sort<T: PartialOrd + Copy>(s: &mut [T]) {
    let len = s.len();
    if len < 2 {
        return;
    }

    let mid = len / 2;
    merge_sort(&mut s[..mid]);
    merge_sort(&mut s[mid..]);

    let mut tmp = Vec::with_capacity(len);
    let mut i = 0;
    let mut j = mid;

    while i < mid && j < len {
        if s[i] < s[j] {
            tmp.push(s[i]);
            i += 1;
        } else {
            tmp.push(s[j]);
            j += 1;
        }
    }
    if i < mid {
        tmp.extend_from_slice(&s[i..mid]);
    } else if j < len {
        tmp.extend_from_slice(&s[j..len]);
    }

    s.copy_from_slice(&tmp[..]);
}
