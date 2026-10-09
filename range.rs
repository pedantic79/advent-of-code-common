use std::{
    cmp::{max, min},
    ops::Range,
};

/// Inclusive overlap of two endpoint pairs, independent of endpoint order.
pub fn overlap_bounds<T: Ord + Copy>((a, b): (T, T), (c, d): (T, T)) -> Option<(T, T)> {
    let low = a.min(b).max(c.min(d));
    let high = a.max(b).min(c.max(d));

    (low <= high).then_some((low, high))
}

#[cfg(test)]
mod overlap_tests {
    use super::overlap_bounds;

    #[test]
    fn handles_reversed_disjoint_and_touching_intervals() {
        assert_eq!(overlap_bounds((5_u64, 1), (7, 3)), Some((3, 5)));
        assert_eq!(overlap_bounds((1_i32, 3), (3, 5)), Some((3, 3)));
        assert_eq!(overlap_bounds((1, 2), (3, 4)), None);
    }
}

/// range_intersect takes range `range` and range `cutter`. This calculates the three possible
/// overlaps of `range` with respects to `cutter`
///
/// ```ignore
/// range  [         )
/// cutter     [   )
///        [   )       <- before
///            [   )   <- inter
///                [ ) <- after
/// ```
///
/// before starts with range.start, ends with min of cutter.start and range.end
/// inter starts with max cutter.start and range.start, ends with min of range.end, cutter.end
/// after starts with max cutter.end and range.start, ends with range.end
///
/// ```ignore
/// # use advent_of_code_2024::common::range_intersect;
/// // x does not intersect, two possible ways, before or after
/// assert_eq!(range_intersect(10..15, &(0..7)), [None, None, Some(10..15)]);
/// assert_eq!(range_intersect(10..15, &(17..30)), [Some(10..15), None, None]);
///
/// // x intersects completely
/// assert_eq!(range_intersect(10..15, &(10..15)), [None, Some(10..15), None]);
///
/// // x is within both the start and end of range
/// assert_eq!(range_intersect(10..15, &(12..13)), [Some(10..12), Some(12..13), Some(13..15)]);
///
/// // x intersects to the beginning or end of range
/// assert_eq!(range_intersect(10..15, &(0..13)), [None, Some(10..13), Some(13..15)]);
/// assert_eq!(range_intersect(10..15, &(13..20)), [Some(10..13), Some(13..15), None]);
/// ```
///
pub fn range_intersect<T: Ord + Copy>(range: Range<T>, cutter: &Range<T>) -> [Option<Range<T>>; 3] {
    let before = range.start..min(range.end, cutter.start);
    let inter = max(range.start, cutter.start)..min(cutter.end, range.end);
    let after = max(cutter.end, range.start)..range.end;

    [
        (before.end > before.start).then_some(before),
        (inter.end > inter.start).then_some(inter),
        (after.end > after.start).then_some(after),
    ]
}

#[cfg(feature = "common_test")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_intersect() {
        // x does not intersect, two possible ways, before or after
        assert_eq!(range_intersect(10..15, &(0..7)), [None, None, Some(10..15)]);
        assert_eq!(
            range_intersect(10..15, &(17..30)),
            [Some(10..15), None, None]
        );

        // x intersects completely
        assert_eq!(
            range_intersect(10..15, &(10..15)),
            [None, Some(10..15), None]
        );

        // x is within both the start and end of range
        assert_eq!(
            range_intersect(10..15, &(12..13)),
            [Some(10..12), Some(12..13), Some(13..15)]
        );

        // x intersects to the beginning or end of range
        assert_eq!(
            range_intersect(10..15, &(0..13)),
            [None, Some(10..13), Some(13..15)]
        );
        assert_eq!(
            range_intersect(10..15, &(13..20)),
            [Some(10..13), Some(13..15), None]
        );
    }
}
