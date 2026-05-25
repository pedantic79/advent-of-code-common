/// Max-Heap
pub fn accumulate_max_n<const N: usize, T: Ord>(mut heap: [T; N], x: T) -> [T; N] {
    for i in 0..N {
        assert!(i < heap.len());
        if x > heap[i] {
            // SAFETY: `i` will always be within the array bounds.
            // We will always make an empty slot. So at most we will need to
            // copy (N - 1) elements (i.e. creating slot at the 0th element)
            // If this were the last, the (N - 1)th element, then 0 elements
            // are copied (N - 1 - (N - 1) == 0).
            // see: https://doc.rust-lang.org/1.65.0/src/alloc/vec/mod.rs.html#1392-1406
            unsafe {
                // Drop the last element
                std::ptr::drop_in_place(heap.as_mut_ptr().add(N - 1));

                // Get ptr to first element
                let p = heap.as_mut_ptr().add(i);

                // Copy one less than the elements on the right, on spot over
                std::ptr::copy(p, p.add(1), N - 1 - i);
                std::ptr::write(p, x);
            }
            break;
        }
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
