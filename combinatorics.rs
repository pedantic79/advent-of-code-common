/// Enumerate every subset of `bits` elements in binary-reflected Gray order.
/// Consecutive masks differ in exactly one bit. The empty subset comes first.
///
/// Panics when `2^bits` cannot be represented by `usize`.
pub fn gray_subsets(bits: u32) -> impl ExactSizeIterator<Item = usize> + DoubleEndedIterator {
    let count = 1_usize.checked_shl(bits).expect("too many subset elements");
    (0..count).map(|index| index ^ (index >> 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumerates_every_subset_with_one_change_between_neighbors() {
        for bits in 0..=10 {
            let masks: Vec<_> = gray_subsets(bits).collect();
            assert_eq!(masks[0], 0);
            assert!(
                masks
                    .array_windows()
                    .all(|[a, b]| (a ^ b).count_ones() == 1)
            );
            let mut sorted = masks;
            sorted.sort_unstable();
            assert!(sorted.into_iter().eq(0..1 << bits));
        }
    }

    #[test]
    #[should_panic(expected = "too many subset elements")]
    fn rejects_unrepresentable_subset_counts() {
        let _ = gray_subsets(usize::BITS);
    }
}
