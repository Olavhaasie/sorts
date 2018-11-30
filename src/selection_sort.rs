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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_sort() {
        let test = &mut [9, 2, 5, 1, 3, 4, 6, 8, 7];
        selection_sort(test);
        assert_eq!(test, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn sort_empty_set() {
        let test: &mut [u8] = &mut [];
        selection_sort(test);
        assert_eq!(test, &[]);
    }

}
