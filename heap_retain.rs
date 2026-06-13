/// Accumulates the largest `N` elements from a stream into a fixed-size sorted array.
///
/// This function expects `heap` to be sorted in descending order (largest to smallest).
/// It inserts the new element `x` into the correct sorted position and shifts subsequent
/// elements to the right, discarding the smallest element (`heap[N - 1]`).
///
/// # Preconditions
/// - The input `heap` must be sorted in descending order.
///
/// # Postconditions
/// - The returned array is sorted in descending order.
/// - The returned array contains the largest `N` elements from the union of `heap` and `{x}`.
///
/// # Performance
/// - **Fast Path**: If `x` is smaller than or equal to the smallest element in `heap`, the function
///   immediately returns `heap` in $O(1)$ time with a single comparison.
/// - **Slow Path**: If `x` is larger than the smallest element, it searches for the insertion index
///   and shifts elements to the right, taking $O(N)$ time.
pub fn accumulate_max_n<const N: usize, T: Ord>(mut heap: [T; N], x: T) -> [T; N] {
    if N == 0 {
        return heap;
    }
    if x <= heap[N - 1] {
        return heap;
    }
    let i = heap[..N - 1]
        .iter()
        .position(|val| *val < x)
        .unwrap_or(N - 1);
    // SAFETY:
    // 1. `N` is strictly greater than 0 due to the earlier check.
    // 2. The index `i` is guaranteed to be in the range `0..=N - 1` since
    //    `position` is called on a slice of length `N - 1` and defaults to `N - 1`.
    // 3. Dropping the element at `N - 1` is safe because `N - 1` is a valid index.
    // 4. `p` points to index `i`, which is a valid index in `heap`.
    // 5. The destination of the copy (`p + 1`) and the number of elements to copy
    //    (`N - 1 - i`) are within the array bounds because the copied range `i..N-1`
    //    moves to `i+1..N`, which does not exceed `N`.
    // 6. `ptr::copy` handles overlapping regions correctly (memmove).
    // 7. Writing `x` at `i` overwrites the logically uninitialized slot, ensuring
    //    all elements are initialized exactly once without double-dropping.
    unsafe {
        // Drop the last element
        std::ptr::drop_in_place(heap.as_mut_ptr().add(N - 1));

        // Get ptr to first element to move
        let p = heap.as_mut_ptr().add(i);

        // Copy elements after insertion point one spot to the right
        std::ptr::copy(p, p.add(1), N - 1 - i);
        std::ptr::write(p, x);
    }
    heap
}

#[cfg(feature = "common_test")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accumulate_max_n() {
        // Start with a sorted array representing the initial heap of largest elements
        let heap = [10, 8, 5, 2];

        // Insert smaller element - should not modify heap
        assert_eq!(accumulate_max_n(heap, 1), [10, 8, 5, 2]);

        // Insert element larger than the smallest element
        assert_eq!(accumulate_max_n(heap, 3), [10, 8, 5, 3]);

        // Insert element larger than all elements
        assert_eq!(accumulate_max_n(heap, 12), [12, 10, 8, 5]);

        // Insert element in the middle
        assert_eq!(accumulate_max_n(heap, 9), [10, 9, 8, 5]);

        // Accumulate a series of values into a fresh (initially zero) heap
        let mut h = [0; 3];
        h = accumulate_max_n(h, 5);
        h = accumulate_max_n(h, 3);
        h = accumulate_max_n(h, 8);
        h = accumulate_max_n(h, 2);
        h = accumulate_max_n(h, 9);
        assert_eq!(h, [9, 8, 5]);
    }
}
