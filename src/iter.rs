//! Iterator over bits.

use crate::BitSlice;
use core::iter::{FusedIterator, Iterator};

/// Iterator over the bits of a [`BitSlice`].
#[derive(Clone, Debug)]
pub struct Iter<'a> {
    slice: &'a BitSlice,
}

impl<'a> Iter<'a> {
    /// Create a new iterator over a `BitSlice`.
    #[must_use]
    pub fn new(slice: &'a BitSlice) -> Self {
        Self { slice }
    }
}

impl Iterator for Iter<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        let (bit, rest) = self.slice.split_first()?;
        self.slice = rest;
        Some(bit)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.slice.len();
        (len, Some(len))
    }

    fn count(self) -> usize {
        self.slice.len()
    }

    fn last(self) -> Option<bool> {
        self.slice.last()
    }

    fn nth(&mut self, n: usize) -> Option<bool> {
        self.slice = self.slice.get_slice(n..self.slice.len()).ok()?;
        self.next()
    }
}

impl DoubleEndedIterator for Iter<'_> {
    fn next_back(&mut self) -> Option<bool> {
        let (bit, rest) = self.slice.split_last()?;
        self.slice = rest;
        Some(bit)
    }

    fn nth_back(&mut self, n: usize) -> Option<bool> {
        let new_end = self.slice.len().checked_sub(n)?;
        self.slice = self.slice.get_slice(0..new_end).ok()?;
        self.next_back()
    }
}

impl ExactSizeIterator for Iter<'_> {
    fn len(&self) -> usize {
        self.slice.len()
    }
}

impl FusedIterator for Iter<'_> {}

#[cfg(test)]
mod tests {
    use crate::BitSlice;
    const BYTES: [u8; 2] = [0xa0, 0x0a];
    const BITS: [bool; 16] = [
        true, false, true, false, false, false, false, false, false, false, false, false, true,
        false, true, false,
    ];

    #[test]
    fn iter_count() {
        assert_eq!(BitSlice::EMPTY.iter().count(), 0);
        assert_eq!(BitSlice::new(&BYTES).iter().count(), 16);
    }

    #[test]
    #[allow(clippy::iter_nth_zero)]
    fn iter_nth() {
        let bits = BitSlice::new(&BYTES);
        assert_eq!(bits.iter().nth(0), Some(BITS[0]));
        assert_eq!(bits.iter().nth(7), Some(BITS[7]));
        assert_eq!(bits.iter().nth(15), Some(BITS[15]));
        assert_eq!(bits.iter().nth(16), None);
    }

    #[test]
    fn iter_last() {
        assert_eq!(BitSlice::new(&BYTES).iter().last(), Some(BITS[15]));
        assert_eq!(BitSlice::EMPTY.iter().last(), None);
    }

    #[test]
    fn iter_next_back() {
        let bits = BitSlice::new(&BYTES);
        assert!(bits.iter().rev().eq(BITS.iter().copied().rev()));
    }

    #[test]
    fn iter_nth_back() {
        let bits = BitSlice::new(&BYTES);
        assert_eq!(bits.iter().nth_back(0), Some(BITS[15]));
        assert_eq!(bits.iter().nth_back(7), Some(BITS[8]));
        assert_eq!(bits.iter().nth_back(15), Some(BITS[0]));
        assert_eq!(bits.iter().nth_back(16), None);
    }

    #[test]
    fn iter_exact_size() {
        let bits = BitSlice::new(&BYTES);
        let mut iter = bits.iter();
        assert_eq!(iter.len(), 16);
        assert!(iter.next().is_some());
        assert_eq!(iter.len(), 15);
        assert!(iter.next_back().is_some());
        assert_eq!(iter.len(), 14);
    }

    #[test]
    fn iter_double_ended() {
        let bits = BitSlice::new(&BYTES);
        let mut iter = bits.iter();
        assert_eq!(iter.next(), Some(BITS[0]));
        assert_eq!(iter.next_back(), Some(BITS[15]));
        assert_eq!(iter.next(), Some(BITS[1]));
        assert_eq!(iter.next_back(), Some(BITS[14]));
    }
}
