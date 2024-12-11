use std::{iter, mem};

use ahash::{HashMap, HashMapExt};
use aoc_runner_derive::{aoc, aoc_generator};
use num::Integer;

use crate::common::parse::parse_split;

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Vec<usize> {
    parse_split(input, ' ')
}

fn one_or_two_elements<T>(first: T, second: Option<T>) -> impl Iterator<Item = T> {
    iter::once(first).chain(iter::once(second).flatten())
}

fn rule2(stone: usize) -> impl Iterator<Item = usize> {
    if stone == 0 {
        one_or_two_elements(1, None)
    } else {
        let s = format!("{}", stone);
        if s.len().is_even() {
            let mid = s.len() / 2;
            let (l, r) = s.split_at(mid);
            one_or_two_elements(l.parse().unwrap(), Some(r.parse().unwrap()))
        } else {
            one_or_two_elements(stone * 2024, None)
        }
    }
}

fn solve<const T: usize>(stones: &[usize]) -> usize {
    let mut data = HashMap::new();
    let mut counts = HashMap::new();
    for &stone in stones {
        data.insert(stone, 1);
    }
    for _ in 0..T {
        for (&stone, &stone_count) in data.iter() {
            for a in rule2(stone) {
                *counts.entry(a).or_insert(0) += stone_count;
            }
        }
        mem::swap(&mut data, &mut counts);
        counts.clear();
    }

    data.values().sum()
}

#[aoc(day11, part1)]
pub fn part1(inputs: &[usize]) -> usize {
    solve::<25>(inputs)
}

#[aoc(day11, part2)]
pub fn part2(inputs: &[usize]) -> usize {
    solve::<75>(inputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"125 17";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 55312);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 65601038650482);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day11.txt");
        const ANSWERS: (usize, usize) = (183248, 218811774248729);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
