/// Insert a nondominated candidate, removing any entries it dominates.
///
/// `dominates(a, b)` must be a reflexive, transitive relation. Equivalent
/// candidates are rejected. The vector must already be a Pareto frontier.
#[inline(always)]
pub fn insert<T>(frontier: &mut Vec<T>, candidate: T, dominates: impl Fn(&T, &T) -> bool) -> bool {
    if frontier.iter().any(|old| dominates(old, &candidate)) {
        return false;
    }

    frontier.retain(|old| !dominates(&candidate, old));
    frontier.push(candidate);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_incomparable_entries_and_removes_dominated_entries() {
        let dominates = |a: &(u32, u32), b: &(u32, u32)| a.0 <= b.0 && a.1 <= b.1;
        let mut frontier = Vec::new();
        assert!(insert(&mut frontier, (1, 5), dominates));
        assert!(insert(&mut frontier, (5, 1), dominates));
        assert!(!insert(&mut frontier, (1, 5), dominates));
        assert!(!insert(&mut frontier, (6, 6), dominates));
        assert!(insert(&mut frontier, (1, 1), dominates));
        assert_eq!(frontier, [(1, 1)]);
    }
}
