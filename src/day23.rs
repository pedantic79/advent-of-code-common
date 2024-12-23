use ahash::{HashMapExt, HashSetExt};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use petgraph::graph::{NodeIndex, UnGraph};
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

type SStr = [u8; 2];

fn from_s(s: &str) -> SStr {
    s.as_bytes().try_into().unwrap()
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
                triangle.sort_unstable();
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
    mut p: HashSet<NodeIndex>,
    mut x: HashSet<NodeIndex>,
    max_clique: &mut HashSet<NodeIndex>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > max_clique.len() {
            max_clique.clear();
            max_clique.extend(r.iter());
        }
        return;
    }

    let pivot = p.union(&x).next().copied().unwrap();
    let pivot_neighbors: HashSet<_> = graph.neighbors(pivot).collect();
    let p_minus_neighbors: HashSet<_> = p.difference(&pivot_neighbors).copied().collect();

    for v in p_minus_neighbors {
        r.insert(v);

        let neighbors: HashSet<_> = graph.neighbors(v).collect();
        let p_new = p.intersection(&neighbors).copied().collect();
        let x_new = x.intersection(&neighbors).copied().collect();

        bron_kerbosch(graph, r, p_new, x_new, max_clique);

        r.remove(&v);
        p.remove(&v);
        x.insert(v);
    }
}

#[aoc(day23, part2)]
pub fn part2(graph: &UnGraph<SStr, ()>) -> String {
    let mut max_clique = HashSet::new();

    bron_kerbosch(
        graph,
        &mut Default::default(),
        graph.node_indices().collect(),
        Default::default(),
        &mut max_clique,
    );

    max_clique
        .into_iter()
        .map(|idx| unsafe {
            // SAFETY: node_weight's are all [u8; 2] and unicode
            std::str::from_utf8_unchecked(graph.node_weight(idx).unwrap().as_slice())
        })
        .sorted_unstable()
        .join(",")
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
