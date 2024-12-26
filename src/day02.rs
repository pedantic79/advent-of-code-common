use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::common::parse::parse_split;

#[derive(Debug, PartialEq, Eq)]
pub struct Object {}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|line| parse_split(line, ' ')).collect()
}

fn is_good<'a>(v: impl Iterator<Item = &'a i32> + Clone) -> bool {
    let check = |(a, b)| (1..4).contains(&(b - a));
    v.clone().tuple_windows().all(check) || v.tuple_windows().map(|(x, y)| (y, x)).all(check)
}

fn remove_iter<T>(v: &[T], mid: usize) -> impl Iterator<Item = &'_ T> + Clone {
    let (a, b) = (&v[..mid], &v[mid + 1..]);
    a.iter().chain(b.iter())
}

#[aoc(day2, part1)]
pub fn part1(inputs: &[Vec<i32>]) -> usize {
    inputs.iter().filter(|line| is_good(line.iter())).count()
}

#[aoc(day2, part2)]
pub fn part2(inputs: &[Vec<i32>]) -> usize {
    inputs
        .iter()
        .filter(|line| {
            is_good(line.iter()) || (0..line.len()).any(|i| is_good(remove_iter(line, i)))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 2);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 4);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day2.txt");
        const ANSWERS: (usize, usize) = (670, 700);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
