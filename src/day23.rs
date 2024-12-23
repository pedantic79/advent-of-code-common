use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use smallstr::SmallString;

type SStr = SmallString<[u8; 2]>;

#[derive(Debug, PartialEq, Eq)]
pub struct Object {
    connections: HashMap<SStr, HashSet<SStr>>,
    keys: Vec<SStr>,
}

#[aoc_generator(day23)]
pub fn generator(input: &str) -> Object {
    let mut connections = HashMap::new();

    for (a, b) in input.lines().map(|l| {
        l.split_once('-')
            .map(|(x, y)| (SStr::from(x), SStr::from(y)))
            .unwrap()
    }) {
        connections
            .entry(a.clone())
            .or_insert_with(HashSet::new)
            .insert(b.clone());
        connections.entry(b).or_insert_with(HashSet::new).insert(a);
    }

    let keys = connections.keys().cloned().collect();

    Object { connections, keys }
}

#[aoc(day23, part1)]
pub fn part1(inputs: &Object) -> usize {
    let mut triangles = Vec::new();
    let nodes = &inputs.keys;
    let graph = &inputs.connections;
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            for k in j + 1..nodes.len() {
                let n1 = nodes[i].clone();
                let n2 = nodes[j].clone();
                let n3 = nodes[k].clone();

                if graph.get(&n1).unwrap_or(&HashSet::new()).contains(&n2)
                    && graph.get(&n1).unwrap_or(&HashSet::new()).contains(&n3)
                    && graph.get(&n2).unwrap_or(&HashSet::new()).contains(&n3)
                {
                    triangles.push([n1, n2, n3]);
                }
            }
        }
    }

    triangles
        .iter()
        .filter(|v| v.iter().any(|s| s.starts_with('t')))
        .count()
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
        r.insert(node.clone());
        let neighbors = graph.get(node).cloned().unwrap_or(HashSet::new());
        let mut new_p: HashSet<SStr> = p.intersection(&neighbors).cloned().collect();
        let mut new_x: HashSet<SStr> = x.intersection(&neighbors).cloned().collect();

        bron_kerbosch(graph, r, &mut new_p, &mut new_x, cliques);

        r.remove(node);
        p.remove(node);
        x.insert(node.clone());
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

    cliques
        .into_iter()
        .max_by_key(|clique| clique.len())
        .unwrap()
        .into_iter()
        .sorted()
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
