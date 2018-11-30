/// Sorts a slice in-place using
/// [bubble sort](https://en.wikipedia.org/wiki/Bubble_sort).
///
/// # Examples
/// To sort a vector
/// ```
/// let mut slice = vec![3,2,1,4];
/// sorts::bubble_sort(&mut slice);
/// assert_eq!(slice, &[1,2,3,4]);
/// ```
/// All kinds of slices can be sorted as long as they implement
/// [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
/// ```
/// let mut strings = vec!["rustc", "cargo", "rustup"];
/// sorts::bubble_sort(&mut strings);
/// assert_eq!(strings, &["cargo", "rustc", "rustup"]);
/// ```
pub fn bubble_sort<T: PartialOrd>(sequence: &mut [T]) {
    let len = sequence.len();

    for i in (0..len).rev() {
        let mut has_swapped = false;
        for j in 0..i {
            if sequence[j] > sequence[j + 1] {
                sequence.swap(j, j + 1);
                has_swapped = true;
            }
        }
        if !has_swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_sort() {
        let test = &mut [4,3,1,2];
        bubble_sort(test);
        assert_eq!(test, &[1,2,3,4]);
    }

    #[test]
    fn sort_signed() {
        let test = &mut [-4,3,1,2,-8];
        bubble_sort(test);
        assert_eq!(test, &[-8,-4,1,2,3]);
    }

    #[test]
    fn sort_strings() {
        let test = &mut ["bubble", "aardvark", "compiler"];
        bubble_sort(test);
        assert_eq!(test, &["aardvark", "bubble", "compiler"]);
    }

    #[test]
    fn sort_empty() {
        let test: &mut [u8] = &mut [];
        bubble_sort(test);
        assert_eq!(test, &[]);
    }
}
