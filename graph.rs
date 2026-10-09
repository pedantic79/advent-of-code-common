use std::collections::VecDeque;

/// Kahn ordering of the subgraph reachable from `start`, with dense node IDs.
///
/// `successors` must return stable edges into `0..node_count` on every call.
/// Nodes precede their successors; parallel edges are allowed. An error gives
/// one reachable node left in a cycle. Unreachable nodes and cycles are ignored.
/// Dense storage avoids the hashing needed by the general crate algorithms.
pub fn topological_order<IN>(
    node_count: usize,
    start: usize,
    mut successors: impl FnMut(usize) -> IN,
) -> Result<Vec<usize>, usize>
where
    IN: IntoIterator<Item = usize>,
{
    let mut incoming = vec![0; node_count];
    let mut seen = vec![false; node_count];
    let mut queue = VecDeque::from([start]);
    seen[start] = true;

    while let Some(node) = queue.pop_front() {
        for next in successors(node) {
            incoming[next] += 1;
            if !seen[next] {
                seen[next] = true;
                queue.push_back(next);
            }
        }
    }

    let mut order = Vec::new();
    if incoming[start] == 0 {
        queue.push_back(start);
    }

    while let Some(node) = queue.pop_front() {
        order.push(node);
        for next in successors(node) {
            incoming[next] -= 1;
            if incoming[next] == 0 {
                queue.push_back(next);
            }
        }
    }

    if let Some(node) = (0..node_count).find(|&node| seen[node] && incoming[node] != 0) {
        Err(node)
    } else {
        Ok(order)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_dependencies_follow_all_parents_and_ignore_unreachable_cycles() {
        let graph = [vec![1, 2], vec![3, 3], vec![3], vec![], vec![4]];
        assert_eq!(
            topological_order(5, 0, |node| graph[node].iter().copied()),
            Ok(vec![0, 1, 2, 3])
        );
        assert_eq!(
            topological_order(5, 4, |node| graph[node].iter().copied()),
            Err(4)
        );
    }

    #[test]
    fn detects_cycles_that_return_to_the_root() {
        let graph = [vec![1], vec![0]];
        assert!(topological_order(2, 0, |node| graph[node].iter().copied()).is_err());
    }
}
