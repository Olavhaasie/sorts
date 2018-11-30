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
    fn correct_bubble_sort() {
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
    fn empty_bubble_sort() {
        let test: &mut [u8] = &mut [];
        bubble_sort(test);
        assert_eq!(test, &[]);
    }
}
