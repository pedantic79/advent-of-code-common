use ahash::{HashMap, HashMapExt};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::{
    matrix::directions::DIRECTIONS_4,
    prelude::{astar, astar_bag_collect},
    utils::move_in_direction,
};

type Pad<'a> = &'a [&'a [u8]];
type Position = (usize, usize);
type Cache = HashMap<(Position, Position, usize), usize>;

const NUM_PAD: Pad = &[b"789", b"456", b"123", b".0A"];
const DIR_PAD: Pad = &[b".^A", b"<v>"];
const NUM_POS: &[Position] = &[
    (3, 1),
    (2, 0),
    (2, 1),
    (2, 2),
    (1, 0),
    (1, 1),
    (1, 2),
    (0, 0),
    (0, 1),
    (0, 2),
    (3, 2),
];
const DIR_POS: &[Position] = &[(0, 1), (0, 2), (1, 0), (1, 1), (1, 2)];

fn char2pos(pad: Pad, c: u8) -> Position {
    if pad.len() == 4 {
        NUM_POS[if c.is_ascii_digit() {
            usize::from(c - b'0')
        } else {
            10
        }]
    } else {
        DIR_POS[match c {
            b'^' => 0,
            b'A' => 1,
            b'<' => 2,
            b'v' => 3,
            b'>' => 4,
            _ => panic!("unknown char"),
        }]
    }
}

fn calculate_minimum(
    pad: Pad,
    a: Position,
    b: Position,
    remain: usize,
    memo: &mut Cache,
) -> (Vec<u8>, usize) {
    // Get all paths from a to b
    let paths = astar(
        &a,
        |&pos| {
            DIRECTIONS_4
                .into_iter()
                .filter_map(move |dir| move_in_direction(pos, dir, (pad.len(), pad[0].len())))
                .filter(|new_pos| pad[new_pos.0][new_pos.1] != b'.')
                .map(|new_pos| {
                    (
                        new_pos,
                        if remain > 0 {
                            get_cost(pad, pos, new_pos, remain - 1, memo)
                        } else {
                            1
                        },
                    )
                })
                .collect_vec()
        },
        |_| 0,
        |&pos| pos == b,
    )
    .unwrap();

    memo.insert((a, b, remain), paths.1);

    // convert the positions to directions and cost
    // return the minimum cost
    (pos2dir(&paths.0), paths.1)
}

fn input_to_move(pad: Pad, input: &[u8], remain: usize, memo: &mut Cache) -> Vec<u8> {
    std::iter::once(b'A')
        .chain(input.iter().copied())
        .tuple_windows()
        .flat_map(|(w, u)| {
            calculate_minimum(pad, char2pos(pad, w), char2pos(pad, u), remain, memo).0
        })
        .collect()
}

fn pos2dir(positions: &[Position]) -> Vec<u8> {
    let mut dirs: Vec<_> = positions
        .windows(2)
        .map(|x| {
            let (a0, a1) = (x[0].0 as isize, x[0].1 as isize);
            let (b0, b1) = (x[1].0 as isize, x[1].1 as isize);

            (b0 - a0, b1 - a1)
        })
        .map(|dir| match dir {
            (0, 1) => b'>',
            (1, 0) => b'v',
            (0, -1) => b'<',
            (-1, 0) => b'^',
            _ => panic!("unknown direction"),
        })
        .collect();

    dirs.push(b'A');
    dirs
}

fn get_cost(pad: Pad, a: Position, b: Position, remain: usize, memo: &mut Cache) -> usize {
    if let Some(&cost) = memo.get(&(a, b, remain)) {
        return cost;
    }

    let mut cost = 1;

    if remain > 0 {
        let a = calculate_minimum(pad, a, b, remain - 1, memo);
        cost += a.1;
    }
    cost
}

#[aoc_generator(day21)]
pub fn generator(input: &str) -> Vec<String> {
    input.lines().map(ToOwned::to_owned).collect()
}

#[aoc(day21, part1)]
pub fn part1(inputs: &[String]) -> usize {
    // let mut num_cache = HashMap::new();
    let mut dir_cache = HashMap::new();

    let mut total = 0;
    for s in inputs.iter() {
        let n = s[..s.len() - 1].parse::<usize>().unwrap();
        let s = input_to_move(NUM_PAD, s.as_bytes(), 3, &mut dir_cache);
        let s = input_to_move(DIR_PAD, &s, 2, &mut dir_cache);
        let s = input_to_move(DIR_PAD, &s, 1, &mut dir_cache);

        total += n * s.len();
    }

    total
}

#[aoc(day21, part2)]
pub fn part2(inputs: &[String]) -> usize {
    unimplemented!()
}

pub fn debug_output(v: &[u8]) -> &str {
    unsafe { std::str::from_utf8_unchecked(v) }
}

#[cfg(test)]
mod tests {

    use super::*;

    const SAMPLE: &str = r"029A
980A
179A
456A
379A";

    #[test]
    pub fn input_test() {
        // println!("{:?}", generator(SAMPLE));

        let mut num_cache = HashMap::new();
        let mut dir_cache = HashMap::new();

        for _ in 0..10 {
            println!(
                "{:?}",
                debug_output(&input_to_move(
                    NUM_PAD,
                    "279A".as_bytes(),
                    1,
                    &mut num_cache
                ))
            );
        }

        for _ in 0..10 {
            println!(
                "{:?}",
                debug_output(&input_to_move(
                    DIR_PAD,
                    "<^A<^^A>>AvvvA".as_bytes(),
                    1,
                    &mut dir_cache
                ))
            );
        }

        // println!(
        //     "{:?}",
        //     debug_output(&input_to_move(
        //         DIR_PAD,
        //         "v<<A>^A>Av<<A>^AA>AvAA^A<vAAA>^A".as_bytes()
        //     ))
        // );
        //<vA<AA>^>AvA^<A>AvA^A<vA<AA>^>AvA^<A>AAvA^A<vA^>AA<A>A<v<A>A^>AAA<Av>A^A
        //v<<A>>^Av<A<A>>^AvAA^<A>Av<<A>>^AAv<A<A>>^AvAA^<A>Av<A>^AA<A>Av<A<A>>^AAA<A>vA^A

        // assert_eq!(
        //     solve_direction_pad("<A^A^>^AvvvA"),
        //     "v<<A>>^A<A>AvA<^AA>A<vAAA>^A"
        // )
        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        //2: 279 v<A <AA>>^AvA^<A>AvA^Av<A<AA>>^AvA^<A>AAvA^Av<A>^AA<A>A<vA<A>>^AAA<Av>A^A
        //2: 279 v<<A >>^Av<A<A>>^AvAA<^A>Av<A<AA>>^AvA<^A>AAvA^Av<A>^AA<A>Av<<A>A^>AAAvA^<A>A
        part1(&["279A".to_string()]);
        // part1(&["279A".to_string()]);
        // assert_eq!(part1(&["279A".to_string()]), 279 * 72);
        // assert_eq!(part1(&["341A".to_string()]), 341 * 72);
        assert_eq!(part1(&["459A".to_string()]), 459 * 74);

        // assert_eq!(part1(&generator(SAMPLE)), 126384);
    }

    #[test]
    pub fn part2_test() {
        // assert_eq!(part2(&generator(SAMPLE)), 336);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day21.txt");
        const ANSWERS: (usize, usize) = (0, 0);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            // let output = generator(input);

            // assert_eq!(part1(&output), ANSWERS.0);
            // assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
