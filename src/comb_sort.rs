pub fn comb_sort<T: PartialOrd>(s: &mut [T]) {
    let len = s.len();
    let inv_shrink: f32 = 1.0 / 1.3;

    let mut gap = len;
    let mut sorted = len < 2;

    while !sorted {
        gap = (gap as f32 * inv_shrink).floor() as usize;

        if gap <= 1 {
            gap = 1;
            sorted = true;
        }

        for i in 0..len - gap {
            if s[i] > s[i + gap] {
                s.swap(i, i + gap);
                sorted = false;
            }
        }
    }
}
