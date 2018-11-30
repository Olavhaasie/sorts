/// Sorts a slice in-place using
/// [insertion sort](https://en.wikipedia.org/wiki/Insertion_sort).
///
/// This sorting algorithm is very efficient when used on small data sets.
/// This is because insertion sort has constant space complexity and works
/// very fast when used on partially sorted data.
///
/// # Examples
/// ```
/// let mut vec = vec![-4, -5, 7, 45, 0];
/// sorts::insertion_sort(&mut vec);
/// assert_eq!(vec, &[-5, -4, 0, 7, 45]);
/// ```
pub fn insertion_sort<T: PartialOrd>(s: &mut [T]) {
    for i in 1..s.len() {
        let mut j = i;
        while j > 0 && s[j - 1] > s[j] {
            s.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_sort() {
        let test = &mut [9, 2, 5, 1, 3, 4, 6, 8, 7];
        insertion_sort(test);
        assert_eq!(test, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn sort_empty_set() {
        let test: &mut [u8] = &mut [];
        insertion_sort(test);
        assert_eq!(test, &[]);
    }

}
