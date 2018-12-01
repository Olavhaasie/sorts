/// Sorts a slice in-place using
/// [selction sort](https://en.wikipedia.org/wiki/Selection_sort).
///
/// # Examples
/// ```
/// let mut vec = vec![56, 32, 78, 16];
/// sorts::insertion_sort(&mut vec);
/// assert_eq!(vec, &[16, 32, 56, 78]);
/// ```
pub fn selection_sort<T: PartialOrd>(s: &mut [T]) {
    for i in 0..s.len() {
        let swap = {
            let mut min = &s[i];
            let mut min_index = i;
            for j in i + 1..s.len() {
                if s[j] < *min {
                    min = &s[j];
                    min_index = j;
                }
            }
            min_index
        };

        if i != swap {
            s.swap(i, swap);
        }
    }
}
