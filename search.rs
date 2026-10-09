use std::ops::Range;

use num::{PrimInt, Unsigned};

/// Find the first integer in a half-open range for which `predicate` is false.
/// Return `range.end` when every value matches. Like slice `partition_point`,
/// the predicate must be true for a prefix and false for the remaining suffix.
/// This searches without allocating or enumerating the range.
pub fn partition_point<T: PrimInt + Unsigned>(
    range: Range<T>,
    mut predicate: impl FnMut(T) -> bool,
) -> T {
    let mut low = range.start;
    let mut high = range.end;
    let two = T::one() + T::one();

    while low < high {
        let middle = low + (high - low) / two;
        if predicate(middle) {
            low = middle + T::one();
        } else {
            high = middle;
        }
    }

    low
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_linear_search_for_every_small_partition() {
        for start in 0_u32..=20 {
            for end in start..=20 {
                for split in 0..=21 {
                    assert_eq!(
                        partition_point(start..end, |value| value < split),
                        split.clamp(start, end)
                    );
                }
            }
        }
    }

    #[test]
    fn handles_full_width_bounds_without_overflow() {
        assert_eq!(
            partition_point(0..u64::MAX, |value| value < u64::MAX - 1),
            u64::MAX - 1
        );
        assert_eq!(partition_point(0..usize::MAX, |_| true), usize::MAX);
    }
}
