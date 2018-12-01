/// Sorts a slice in-place using
/// [gnome sort](https://en.wikipedia.org/wiki/Gnome_sort).
///
/// Gnome sort is also known as stupid sort, because it is very easy to
/// understand and not very efficient. It is based on how a gnome would sort
/// flower pots.
///
/// # Examples
/// ```
/// let mut vec = vec![5,3,2,4];
/// sorts::gnome_sort(&mut vec);
/// assert_eq!(vec, &[2,3,4,5]);
/// ```
pub fn gnome_sort<T: PartialOrd>(s: &mut [T]) {
    let mut i = 0;
    while i < s.len() {
        if i == 0 || s[i - 1] < s[i] {
            i += 1;
        } else {
            s.swap(i - 1, i);
            i -= 1;
        }
    }
}
