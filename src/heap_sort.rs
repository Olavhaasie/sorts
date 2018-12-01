pub fn heap_sort<T: PartialOrd>(s: &mut [T]) {
    build_max_heap(s);
    for i in (0..s.len()).rev() {
        s.swap(0, i);
        max_heapify(s, 0, i);
    }
}

/// Creates an in-place max-heap of given slice.
/// The largest value will be at the first index.
fn build_max_heap<T: PartialOrd>(s: &mut [T]) {
    let len = s.len();
    for i in (0..=len / 2).rev() {
        max_heapify(s, i, len);
    }
}

/// Max heapifies an embedded heap from given index.
fn max_heapify<T: PartialOrd>(s: &mut [T], i: usize, heap_len: usize) {
    let left = 2 * i + 1;
    let right = left + 1;

    let mut largest = i;
    if left < heap_len && s[left] > s[largest] {
        largest = left;
    }
    if right < heap_len && s[right] > s[largest] {
        largest = right;
    }

    if largest != i {
        s.swap(i, largest);
        max_heapify(s, largest, heap_len);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_correct_max_heap() {
        let mut heap = vec![1, 2, 3, 4, 5];
        build_max_heap(&mut heap);
        assert_eq!(heap, &[5, 4, 3, 1, 2]);
    }

    #[test]
    fn correct_max_heapify_right() {
        let mut heap = vec![2, 1, 3];
        max_heapify(&mut heap, 0, 3);
        assert_eq!(heap, &[3, 1, 2]);
    }

    #[test]
    fn correct_max_heapify_left() {
        let mut heap = vec![2, 4, 3];
        max_heapify(&mut heap, 0, 3);
        assert_eq!(heap, &[4, 2, 3]);
    }

    #[test]
    fn correct_max_heapify_none() {
        let mut heap = vec![5, 4, 3];
        max_heapify(&mut heap, 0, 3);
        assert_eq!(heap, &[5, 4, 3]);
    }
}
