struct GapSequence {
    gap: usize,
}

impl GapSequence {
    fn new(n: usize) -> Self {
        Self { gap: n }
    }
}

impl Iterator for GapSequence {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.gap /= 2;

        if self.gap > 0 {
            Some(self.gap)
        } else {
            None
        }
    }
}

pub fn shell_sort<T: PartialOrd>(s: &mut [T]) {
    let len = s.len();
    let gaps = GapSequence::new(len);

    for gap in gaps {
        for i in gap..len {
            let mut j = i;

            while j >= gap && s[j - gap] > s[j] {
                s.swap(j - gap, j);

                j -= gap;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_gap_sequence() {
        let gaps: Vec<_> = GapSequence::new(10).collect();
        assert_eq!(gaps, &[5, 2, 1]);
    }

}
