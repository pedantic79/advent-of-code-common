use std::collections::VecDeque;

use bit_set::BitSet;

/// `bfs_count_bitset` returns the number of elements and using a bitset to keep track of seen nodes.
/// Based on [pathfinding](https://github.com/samueltardieu/pathfinding/blob/v4.0.0/src/directed/bfs.rs#L78)
/// `bfs` algorithm.
///
/// * `mapper`, must map to a usize. This is useful to map coordinates, but not hashing. Bits should be tightly
///   packed, otherwise the BitSet may grow too large.
pub fn bfs_count_bitset<N, FN, IN, FS, FM>(
    start: &N,
    mut successors: FN,
    mut success: FS,
    mut mapper: FM,
) -> Option<usize>
where
    N: Eq + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: FnMut(&N) -> bool,
    FM: FnMut(&N) -> usize,
{
    if success(start) {
        return Some(1);
    }
    let mut queue = VecDeque::new();
    let mut seen = BitSet::new();

    queue.push_back((start.clone(), 0));
    seen.insert(mapper(start));

    while let Some((node, depth)) = queue.pop_front() {
        let depth = depth + 1;
        for successor in successors(&node) {
            if success(&successor) {
                return Some(depth);
            }
            let v = mapper(&successor);
            if !seen.contains(v) {
                queue.push_back((successor, depth));
                seen.insert(v);
            }
        }
    }

    None
}

#[cfg(feature = "common_test")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_count_bitset_immediate() {
        // Start node matches success
        let start = 10;
        let res = bfs_count_bitset(&start, |_n| vec![], |&n| n == 10, |&n| n);
        assert_eq!(res, Some(1));
    }

    #[test]
    fn test_bfs_count_bitset_reachable() {
        // Standard BFS pathfinding on a simple number line
        // From 0 to 5, where from x we can transition to x + 1 or x + 2
        let start = 0;
        let res = bfs_count_bitset(&start, |&n| vec![n + 1, n + 2], |&n| n == 5, |&n| n);
        // Min steps: 0 -> 2 -> 4 -> 5 is 3 steps. Or 0 -> 2 -> 5 is 2 steps: 0 -> 2 (depth 1), 2 -> 5 (depth 2).
        // Let's trace it:
        // Start: 0
        // successors(0) -> [1, 2]. seen={0}. queue=[(1, 1), (2, 1)]. seen={0, 1, 2}.
        // pop 1 (depth 1): successors(1) -> [2, 3]. 2 is seen. queue=[(2, 1), (3, 2)]. seen={0,1,2,3}.
        // pop 2 (depth 1): successors(2) -> [3, 4]. 3 is seen. queue=[(3, 2), (4, 2)]. seen={0,1,2,3,4}.
        // pop 3 (depth 2): successors(3) -> [4, 5]. 5 is success! Returns depth = 3.
        // Wait, why didn't we do 0 -> 2 -> 5 (successors of 2 is 3 and 4, not 5!) Oh, successors(x) is x+1, x+2. So successors(2) is 3, 4.
        // What about 0 -> 2 -> 4 -> 6?
        // Let's trace: 0 -> 2 (depth 1), 2 -> 4 (depth 2), successors(4) -> [5, 6]. 5 is success! Returns depth = 3.
        // So yes, 3 is correct.
        assert_eq!(res, Some(3));
    }

    #[test]
    fn test_bfs_count_bitset_unreachable() {
        // Start at 0, successor always stays at 0 (or seen), target is 5
        let start = 0;
        let res = bfs_count_bitset(&start, |&n| vec![n], |&n| n == 5, |&n| n);
        assert_eq!(res, None);
    }
}
