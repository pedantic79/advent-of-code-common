use std::iter;

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iterate, Itertools};

type Point = (usize, usize);

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    map: HashMap<char, Vec<Point>>,
    size: Point,
}

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Input {
    let mut map = HashMap::new();
    for (y, row) in input.lines().enumerate() {
        for (x, cell) in row.bytes().enumerate() {
            if cell == b'.' || cell == b' ' {
                continue;
            }

            map.entry(cell as char).or_insert(Vec::new()).push((y, x));
        }
    }

    Input {
        map,
        size: (input.lines().count(), input.lines().next().unwrap().len()),
    }
}

fn checked_add_pos(v: Option<Point>, x: Point, y: Point) -> Option<Point> {
    let (v0, v1) = v?;
    let a = v0.checked_add(x.0)?.checked_sub(y.0)?;
    let b = v1.checked_add(x.1)?.checked_sub(y.1)?;

    Some((a, b))
}

fn generate_resonance(
    size: Point,
    iter: impl Iterator<Item = Option<Point>>,
) -> impl Iterator<Item = Point> {
    iter.map_while(|o| o)
        .take_while(move |p| p.0 < size.0 && p.1 < size.1)
}

#[aoc(day8, part1)]
pub fn part1(Input { map, size }: &Input) -> usize {
    let mut points = HashSet::new();
    for pos in map.values() {
        for x in pos.iter().combinations(2) {
            let (a, b) = (*x[0], *x[1]);

            points.extend(generate_resonance(
                *size,
                iter::once(checked_add_pos(Some(a), a, b)),
            ));

            points.extend(generate_resonance(
                *size,
                iter::once(checked_add_pos(Some(b), b, a)),
            ));
        }
    }

    points.len()
}

#[aoc(day8, part2)]
pub fn part2(Input { map, size }: &Input) -> usize {
    let mut points = HashSet::new();
    for pos in map.values() {
        for x in pos.iter().combinations(2) {
            let (a, b) = (*x[0], *x[1]);
            points.extend(generate_resonance(
                *size,
                iterate(Some(b), |&v| checked_add_pos(v, b, a)),
            ));

            points.extend(generate_resonance(
                *size,
                iterate(Some(a), |&v| checked_add_pos(v, a, b)),
            ));
        }
    }

    points.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 14);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 34);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day8.txt");
        const ANSWERS: Point = (413, 1417);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
