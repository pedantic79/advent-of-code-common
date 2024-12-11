use ahash::{HashMap, HashMapExt};
use aoc_runner_derive::{aoc, aoc_generator};
use num::Integer;

use crate::common::parse::parse_split;

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Vec<usize> {
    parse_split(input, ' ')
}

fn rule(input: usize) -> (usize, Option<usize>) {
    if input == 0 {
        (1, None)
    } else {
        let s = format!("{}", input);
        if s.len().is_even() {
            let mid = s.len() / 2;
            let (l, r) = s.split_at(mid);
            (l.parse().unwrap(), Some(r.parse().unwrap()))
        } else {
            (input * 2024, None)
        }
    }
}

fn solve<const T: usize>(inputs: &[usize]) -> usize {
    let mut data: HashMap<usize, usize> = HashMap::new();
    for &i in inputs {
        data.insert(i, 1);
    }
    for _ in 0..T {
        let mut counts = HashMap::new();
        for (&i, &local_count) in data.iter() {
            let (a, b) = rule(i);
            *counts.entry(a).or_insert(0) += local_count;
            if let Some(b) = b {
                *counts.entry(b).or_insert(0) += local_count;
            }
        }
        data = counts;
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
        // assert_eq!(part2(&generator(SAMPLE)), 336);
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
