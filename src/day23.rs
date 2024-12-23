use std::fmt::Write;

use ahash::{HashMapExt, HashSetExt};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use petgraph::graph::{NodeIndex, UnGraph};
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

type SStr = [u8; 2];

fn from_s(s: &str) -> [u8; 2] {
    assert_eq!(s.len(), 2);

    let mut res = [b' '; 2];
    res.clone_from_slice(s.as_bytes());

    res
}

#[aoc_generator(day23)]
pub fn generator(input: &str) -> UnGraph<SStr, ()> {
    let mut graph = UnGraph::<SStr, ()>::new_undirected();
    let mut node_map = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        let a = from_s(a);
        let b = from_s(b);

        let node1 = *node_map.entry(a).or_insert_with(|| graph.add_node(a));
        let node2 = *node_map.entry(b).or_insert_with(|| graph.add_node(b));
        graph.add_edge(node1, node2, ());
    }

    graph
}

#[aoc(day23, part1)]
pub fn part1(graph: &UnGraph<SStr, ()>) -> usize {
    let mut triangles = HashSet::new();

    for edge in graph.edge_indices() {
        let (a, b) = graph.edge_endpoints(edge).unwrap();
        let neighbors_b: HashSet<_> = graph.neighbors(b).collect();

        for c in graph.neighbors(a) {
            if neighbors_b.contains(&c) {
                let mut triangle = [a, b, c];
                triangle.sort();
                triangles.insert(triangle);
            }
        }
    }

    triangles
        .into_iter()
        .filter(|v| v.iter().any(|x| graph.node_weight(*x).unwrap()[0] == b't'))
        .count()
}

fn bron_kerbosch(
    graph: &UnGraph<SStr, ()>,
    r: &mut HashSet<NodeIndex>,
    p: &mut HashSet<NodeIndex>,
    x: &mut HashSet<NodeIndex>,
    cliques: &mut Vec<HashSet<NodeIndex>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
        return;
    }

    for &v in p.clone().iter() {
        r.insert(v);

        let neighbors: HashSet<_> = graph.neighbors(v).collect();
        let mut p_new = p.intersection(&neighbors).copied().collect();
        let mut x_new = x.intersection(&neighbors).copied().collect();

        bron_kerbosch(graph, r, &mut p_new, &mut x_new, cliques);

        r.remove(&v);
        p.remove(&v);
        x.insert(v);
    }
}

#[aoc(day23, part2)]
pub fn part2(graph: &UnGraph<SStr, ()>) -> String {
    let mut cliques = Vec::new();
    {
        let mut r = HashSet::new();
        let mut p = graph.node_indices().collect();
        let mut x = HashSet::new();

        bron_kerbosch(graph, &mut r, &mut p, &mut x, &mut cliques);
    }

    let max = cliques
        .into_iter()
        .max_by_key(|clique| clique.len())
        .unwrap()
        .into_iter()
        .map(|idx| graph.node_weight(idx).unwrap())
        .sorted_unstable()
        .collect_vec();

    let mut buf = String::with_capacity(max.len() * 2);
    buf.write_fmt(format_args!(
        "{}{}",
        char::from(max[0][0]),
        char::from(max[0][1])
    ))
    .unwrap();
    for n in &max[1..] {
        buf.write_fmt(format_args!(",{}{}", char::from(n[0]), char::from(n[1])))
            .unwrap();
    }

    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 7);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), "co,de,ka,ta");
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day23.txt");
        const ANSWERS: (usize, &str) = (998, "cc,ff,fh,fr,ny,oa,pl,rg,uj,wd,xn,xs,zw");

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
