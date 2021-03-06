//! Enriches ranges

use crate::option::BoolExt;

// BEGIN SNIPPET range DEPENDS ON option

pub trait UsizeRangeBoundsExt {
    /// Gets a range on a sequential collection.
    ///
    /// When `range.start_end(len)` returns `Some(start, end)`,
    /// It is guaranteed that `slice[start..end]` does not panics
    /// by out-of-range error if `slice` has at least `len` length.
    ///
    /// As usages, see implementation of SegmentTree or Table.
    fn to_range(&self, len: usize) -> Option<std::ops::Range<usize>>;
}

impl<T: std::ops::RangeBounds<usize>> UsizeRangeBoundsExt for T {
    #[allow(unstable_name_collisions)]
    fn to_range(&self, len: usize) -> Option<std::ops::Range<usize>> {
        use std::ops::Bound::*;

        let start = match self.start_bound() {
            Included(&i) => i,
            Excluded(&i) => i+1,
            Unbounded => 0
        };

        let end = match self.end_bound() {
            Included(&i) => i+1,
            Excluded(&i) => i,
            Unbounded => len,
        };

        (start <= end && end <= len).then(start..end)
    }
}

pub trait BoundCloned<T> {
    fn cloned(self) -> std::ops::Bound<T>;
}

impl<T: Clone> BoundCloned<T> for std::ops::Bound<&T> {
    fn cloned(self) -> std::ops::Bound<T> {
        match self {
            std::ops::Bound::Included(x) => std::ops::Bound::Included(x.clone()),
            std::ops::Bound::Excluded(x) => std::ops::Bound::Excluded(x.clone()),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded
        }
    }
}

pub trait BoundExt<T> {
    fn map<U>(self, f: impl FnOnce(T) -> U) -> std::ops::Bound<U>;
}

impl<T> BoundExt<T> for std::ops::Bound<T> {
    fn map<U>(self, f: impl FnOnce(T) -> U) -> std::ops::Bound<U> {
        match self {
            std::ops::Bound::Included(x) => std::ops::Bound::Included(f(x)),
            std::ops::Bound::Excluded(x) => std::ops::Bound::Excluded(f(x)),
            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded
        }
    }
}

// END SNIPPET

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Bound::*;

    #[test]
    fn test_reversed_range() {
        let range = 3..2;
        assert_eq!(range.to_range(5), None);
    }

    #[test]
    fn test_zero_length() {
        let slice = [0, 1, 2, 3, 4];

        let range1 = (Included(3), Included(2));
        assert_eq!(range1.to_range(5), Some(3..3));
        assert_eq!(slice[range1.to_range(5).unwrap()], []);

        let range2 = (Excluded(2), Included(2));
        assert_eq!(range2.to_range(5), Some(3..3));
        assert_eq!(slice[range2.to_range(5).unwrap()], []);

        let range3 = (Included(3), Excluded(3));
        assert_eq!(range3.to_range(5), Some(3..3));
        assert_eq!(slice[range3.to_range(5).unwrap()], []);

        let range4 = (Excluded(2), Excluded(3));
        assert_eq!(range4.to_range(5), Some(3..3));
        assert_eq!(slice[range4.to_range(5).unwrap()], []);
    }

    #[test]
    fn test_right_edge() {
        let slice = [0, 1, 2, 3, 4];

        let range1 = 4..5;
        assert_eq!(range1.to_range(5), Some(4..5));
        assert_eq!(slice[range1.to_range(5).unwrap()], [4]);

        let range2 = 5..5;
        assert_eq!(range2.to_range(5), Some(5..5));
        assert_eq!(slice[range2.to_range(5).unwrap()], []);

        let range3 = 4..6;
        assert_eq!(range3.to_range(5), None);
        assert_eq!(slice.get(range3), None);

        let range4 = 5..6;
        assert_eq!(range4.to_range(5), None);
        assert_eq!(slice.get(range4), None);

        let range5 = 6..6;
        assert_eq!(range5.to_range(5), None);
        assert_eq!(slice.get(range5), None);
    }

    /*
    #[test]
    fn test_left_edge() {
    }

    #[test]
    fn test_unbounded() {
    }
    */
}
