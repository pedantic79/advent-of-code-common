use std::{collections::VecDeque, ops::ControlFlow};

use bit_set::BitSet;

/// Reusable BFS buffers for nodes with dense, unique indices in `0..node_count`.
/// Avoids hashing and storing paths when only visits or distances are needed.
pub struct DenseBfs<N> {
    seen: Vec<bool>,
    queue: VecDeque<(N, usize)>,
}

impl<N> DenseBfs<N> {
    pub fn new(node_count: usize) -> Self {
        Self {
            seen: vec![false; node_count],
            queue: VecDeque::new(),
        }
    }

    /// Visit each reachable node once in distance order. Return `Break(value)` to
    /// stop, `Continue(false)` to prune a node, or `Continue(true)` to expand it.
    /// The mapper must assign distinct indices to distinct graph nodes.
    #[inline(always)]
    pub fn visit<B, IN: IntoIterator<Item = N>>(
        &mut self,
        start: N,
        mut successors: impl FnMut(&N) -> IN,
        mut mapper: impl FnMut(&N) -> usize,
        mut visitor: impl FnMut(&N, usize) -> ControlFlow<B, bool>,
    ) -> Option<B> {
        self.seen.fill(false);
        self.queue.clear();
        self.seen[mapper(&start)] = true;
        self.queue.push_back((start, 0));
        while let Some((node, distance)) = self.queue.pop_front() {
            match visitor(&node, distance) {
                ControlFlow::Break(value) => return Some(value),
                ControlFlow::Continue(false) => continue,
                ControlFlow::Continue(true) => {}
            }
            for next in successors(&node) {
                let seen = &mut self.seen[mapper(&next)];
                if !*seen {
                    *seen = true;
                    self.queue.push_back((next, distance + 1));
                }
            }
        }
        None
    }
}

/// Result of a dense breadth-first search.
pub struct BfsResult {
    pub goal_distance: Option<usize>,
    /// Maximum distance visited, or the graph's eccentricity if no goal matched.
    pub max_distance: usize,
}

/// Search with edge distances and dense visitation storage.
///
/// `index` must be an injective mapping into `0..node_count`. This needs
/// neither hashing nor path reconstruction for indexed graphs. To measure
/// the farthest reachable distance, use a goal predicate that always returns false.
#[inline]
pub fn bfs_indexed<N, IN>(
    node_count: usize,
    start: N,
    successors: impl FnMut(&N) -> IN,
    index: impl FnMut(&N) -> usize,
    mut goal: impl FnMut(&N) -> bool,
) -> BfsResult
where
    IN: IntoIterator<Item = N>,
{
    let mut max_distance = 0;
    let goal_distance =
        DenseBfs::new(node_count).visit(start, successors, index, |node, distance| {
            max_distance = distance;
            if goal(node) {
                ControlFlow::Break(distance)
            } else {
                ControlFlow::Continue(true)
            }
        });

    BfsResult {
        goal_distance,
        max_distance,
    }
}

/// Orthogonal neighbors of a row-major rectangular grid, without row wrapping.
/// `position < len`, `width > 0`, and `len` must be a multiple of `width`.
#[inline]
pub fn adjacent_indices(position: usize, width: usize, len: usize) -> [Option<usize>; 4] {
    let column = position % width;
    [
        position.checked_sub(width),
        (position < len - width).then(|| position + width),
        (column > 0).then(|| position - 1),
        (column + 1 < width).then_some(position + 1),
    ]
}

/// All-pairs shortest distances for a dense, nonnegative weighted graph.
///
/// `edges(source)` yields `(target, weight)` pairs with `target < node_count`.
/// `T::max_value()` represents unreachable nodes and saturated path lengths.
/// Keeping the matrix avoids petgraph's final conversion to a hash map.
pub fn all_pairs_distances<T, IN>(
    node_count: usize,
    mut edges: impl FnMut(usize) -> IN,
) -> Vec<Vec<T>>
where
    T: num::PrimInt + num::Unsigned,
    IN: IntoIterator<Item = (usize, T)>,
{
    let mut distances = vec![vec![T::max_value(); node_count]; node_count];
    for (source, row) in distances.iter_mut().enumerate() {
        row[source] = T::zero();
        for (target, weight) in edges(source) {
            row[target] = row[target].min(weight);
        }
    }

    for middle in 0..node_count {
        for source in 0..node_count {
            if distances[source][middle] == T::max_value() {
                continue;
            }
            for target in 0..node_count {
                let through_middle = distances[source][middle]
                    .checked_add(&distances[middle][target])
                    .unwrap_or(T::max_value());
                distances[source][target] = distances[source][target].min(through_middle);
            }
        }
    }

    distances
}

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
        return Some(0);
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
            if seen.insert(v) {
                queue.push_back((successor, depth));
            }
        }
    }

    None
}

#[cfg(test)]
mod dense_tests {
    use super::*;

    #[test]
    fn handles_cycles_pruning_early_exit_and_buffer_reuse() {
        let edges = [vec![1, 1, 2], vec![0, 3], vec![3], vec![4], vec![]];
        let mut bfs = DenseBfs::new(edges.len());
        let found = bfs.visit(
            0,
            |&n| edges[n].iter().copied(),
            |&n| n,
            |&n, d| {
                if n == 3 {
                    ControlFlow::Break(d)
                } else {
                    ControlFlow::Continue(true)
                }
            },
        );
        assert_eq!(found, Some(2));
        let mut visits = Vec::new();
        assert_eq!(
            bfs.visit(
                0,
                |&n| edges[n].iter().copied(),
                |&n| n,
                |&n, d| {
                    visits.push((n, d));
                    ControlFlow::<(), bool>::Continue(n != 3)
                }
            ),
            None
        );
        assert_eq!(visits, [(0, 0), (1, 1), (2, 1), (3, 2)]);
        assert_eq!(
            bfs.visit(
                4,
                |&n| edges[n].iter().copied(),
                |&n| n,
                |_, d| ControlFlow::Break(d)
            ),
            Some(0)
        );
    }

    #[test]
    fn neighbors_handle_edges_and_single_rows_or_columns() {
        assert_eq!(adjacent_indices(0, 3, 6), [None, Some(3), None, Some(1)]);
        assert_eq!(adjacent_indices(5, 3, 6), [Some(2), None, Some(4), None]);
        assert_eq!(adjacent_indices(0, 1, 1), [None; 4]);
        assert_eq!(adjacent_indices(1, 1, 3), [Some(0), Some(2), None, None]);
    }
}

#[cfg(test)]
mod indexed_tests {
    use super::*;

    #[test]
    fn breadth_first_distances_include_start_and_visit_cycles_once() {
        let graph = [vec![1, 2], vec![0, 3], vec![3], vec![1], vec![]];
        for (target, expected) in [(0, Some(0)), (3, Some(2)), (4, None)] {
            let result = bfs_indexed(
                5,
                0,
                |&node| graph[node].iter().copied(),
                |&node| node,
                |&node| node == target,
            );
            assert_eq!(result.goal_distance, expected);
        }
        let result = bfs_indexed(
            5,
            0,
            |&node| graph[node].iter().copied(),
            |&node| node,
            |_| false,
        );
        assert_eq!(result.max_distance, 2);
        let isolated = bfs_indexed(
            5,
            4,
            |&node| graph[node].iter().copied(),
            |&node| node,
            |_| false,
        );
        assert_eq!(isolated.max_distance, 0);
    }

    #[test]
    fn grid_neighbors_do_not_wrap_rows_or_cross_edges() {
        assert_eq!(adjacent_indices(0, 3, 6), [None, Some(3), None, Some(1)]);
        assert_eq!(adjacent_indices(2, 3, 6), [None, Some(5), Some(1), None]);
        assert_eq!(adjacent_indices(5, 3, 6), [Some(2), None, Some(4), None]);
        assert_eq!(adjacent_indices(0, 1, 1), [None; 4]);
    }

    #[test]
    fn shortest_distances_handle_parallel_edges_and_saturated_paths() {
        let graph = [
            vec![(1, 250_u8), (1, 2), (2, 20)],
            vec![(2, 3)],
            vec![(3, 254)],
            vec![],
        ];
        let distances = all_pairs_distances(4, |node| graph[node].iter().copied());
        assert_eq!(distances[0], [0, 2, 5, u8::MAX]);
        assert_eq!(distances[2][0], u8::MAX);
        assert_eq!(distances[3][3], 0);
        assert!(all_pairs_distances::<u32, _>(0, |_| []).is_empty());
    }
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
        assert_eq!(res, Some(0));
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
