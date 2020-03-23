#[derive(Copy, Clone, Debug)]
pub struct RingIterator {
    pub curr: usize,
    pub size: usize,
}

impl RingIterator {
    pub fn new(curr: usize, size: usize) -> Self {
        assert!(curr < size);

        Self { curr, size }
    }
}
impl Iterator for RingIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr;
        self.curr = (curr + 1) % self.size;
        Some(curr)
    }
}

impl DoubleEndedIterator for RingIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        let curr = self.curr;
        self.curr = (curr + self.size - 1) % self.size;
        Some(curr)
    }
}

impl ExactSizeIterator for RingIterator {
    fn len(&self) -> usize {
        self.size - (self.curr + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_forwards_from_first() {
        let ring = RingIterator::new(0, 3);

        let check = vec![0, 1, 2, 0, 1, 2];

        let mut index = 0;
        for r in ring.take(6) {
            assert_eq!(check[index], r);
            index += 1;
        }
    }

    #[test]
    fn iter_forwards_from_mid() {
        let ring = RingIterator::new(1, 3);

        let check = vec![1, 2, 0, 1, 2, 0];

        let mut index = 0;
        for r in ring.take(6) {
            assert_eq!(check[index], r);
            index += 1;
        }
    }

    #[test]
    fn iter_forwards_from_last() {
        let ring = RingIterator::new(2, 3);

        let check = vec![2, 0, 1, 2, 0, 1];

        let mut index = 0;
        for r in ring.take(6) {
            assert_eq!(check[index], r);
            index += 1;
        }
    }

    #[test]
    fn iter_backwards_from_first() {
        let ring = RingIterator::new(0, 3);

        let check = vec![0, 2, 1, 0, 2, 1];

        let mut index = 0;
        // NOTE: here ring.rev().take(6) == ring.take(6).rev()
        for r in ring.rev().take(6) {
            assert_eq!(check[index], r);
            index += 1;
        }
    }

    #[test]
    fn iter_backwards_from_mid() {
        let ring = RingIterator::new(1, 3);

        let check = vec![1, 0, 2, 1, 0, 2];

        let mut index = 0;
        // NOTE: here ring.rev().take(6) == ring.take(6).rev()
        for r in ring.rev().take(6) {
            assert_eq!(check[index], r);
            index += 1;
        }
    }

    #[test]
    fn iter_backwards_from_last() {
        let ring = RingIterator::new(2, 3);

        let check = vec![2, 1, 0, 2, 1, 0];

        let mut index = 0;
        // NOTE: why is ring.rev().take(6) != ring.take(6).rev(), and
        // why does it only fail here, and not in the other `iter_backwards` tests
        for r in ring.rev().take(6) {
            assert_eq!(check[index], r);
            index += 1;
        }
    }
}
