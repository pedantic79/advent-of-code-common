use std::fmt::Write;

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type SStr = [u8; 2];

#[derive(Debug, PartialEq, Eq)]
pub struct Object {
    connections: HashMap<SStr, HashSet<SStr>>,
    keys: Vec<SStr>,
}

fn from_s(s: &str) -> [u8; 2] {
    assert_eq!(s.len(), 2);

    let mut res = [b' '; 2];
    res.clone_from_slice(s.as_bytes());

    res
}

#[aoc_generator(day23)]
pub fn generator(input: &str) -> Object {
    let mut connections = HashMap::new();

    for (a, b) in input.lines().map(|l| {
        l.split_once('-')
            .map(|(x, y)| (from_s(x), from_s(y)))
            .unwrap()
    }) {
        connections.entry(a).or_insert_with(HashSet::new).insert(b);
        connections.entry(b).or_insert_with(HashSet::new).insert(a);
    }

    let keys = connections.keys().cloned().collect();

    Object { connections, keys }
}

#[aoc(day23, part1)]
pub fn part1(inputs: &Object) -> usize {
    let mut count = 0;
    let graph = &inputs.connections;
    for (n1, n2, n3) in inputs.keys.iter().tuple_combinations() {
        if graph.get(n1).unwrap_or(&HashSet::new()).contains(n2)
            && graph.get(n1).unwrap_or(&HashSet::new()).contains(n3)
            && graph.get(n2).unwrap_or(&HashSet::new()).contains(n3)
            && [n1, n2, n3].iter().any(|s| s[0] == b't')
        {
            count += 1;
        }
    }

    count
}

fn bron_kerbosch(
    graph: &HashMap<SStr, HashSet<SStr>>,
    r: &mut HashSet<SStr>,
    p: &mut HashSet<SStr>,
    x: &mut HashSet<SStr>,
    cliques: &mut Vec<HashSet<SStr>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
        return;
    }

    let p_clone = p.clone();
    for node in p_clone.iter() {
        r.insert(*node);
        let neighbors = graph.get(node).cloned().unwrap_or(HashSet::new());
        let mut new_p = p.intersection(&neighbors).cloned().collect();
        let mut new_x = x.intersection(&neighbors).cloned().collect();

        bron_kerbosch(graph, r, &mut new_p, &mut new_x, cliques);

        r.remove(node);
        p.remove(node);
        x.insert(*node);
    }
}

#[aoc(day23, part2)]
pub fn part2(inputs: &Object) -> String {
    let graph = &inputs.connections;

    let mut cliques = Vec::new();
    let mut r = HashSet::new();
    let mut p = graph.keys().cloned().collect();
    let mut x = HashSet::new();
    bron_kerbosch(graph, &mut r, &mut p, &mut x, &mut cliques);

    let mut max = cliques
        .into_iter()
        .max_by_key(|clique| clique.len())
        .unwrap()
        .into_iter()
        .collect_vec();

    max.sort_unstable();

    // This avoids allocating a vec, and extra strings.
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
