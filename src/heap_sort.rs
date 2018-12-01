pub fn heap_sort<T: PartialOrd>(s: &mut [T]) {
    build_max_heap(s);
    for i in (0..s.len()).rev() {
        s.swap(0, i);
        max_heapify(s, 0, i);
    }
}

fn build_max_heap<T: PartialOrd>(s: &mut [T]) {
    let len = s.len();
    for i in (0..=len / 2).rev() {
        max_heapify(s, i, len);
    }
}

fn max_heapify<T: PartialOrd>(s: &mut [T], i: usize, heap_len: usize) {
    let left = 2 * i;
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
