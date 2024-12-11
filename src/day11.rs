use std::mem;

use ahash::{HashMap, HashMapExt};
use aoc_runner_derive::{aoc, aoc_generator};
use num::Integer;

#[derive(Debug, PartialEq, Eq)]
pub struct Object {}

type INT = usize;

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Vec<String> {
    input.split(' ').map(|s| s.to_string()).collect()
}

fn rule(input: &str) -> (String, Option<String>) {
    if input == "0" {
        ("1".into(), None)
    } else if input.len().is_even() {
        let mid = input.len() / 2;
        let r = input[mid..].trim_start_matches('0');

        (
            input[0..mid].to_string(),
            Some((if !r.is_empty() { r } else { "0" }).into()),
        )
    } else {
        let i: INT = input.parse().unwrap();
        (format!("{}", i * 2024), None)
    }
}

#[aoc(day11, part1)]
pub fn part1(inputs: &[String]) -> usize {
    let mut inputs = inputs.to_vec();
    let mut other = Vec::new();
    for _ in 0..25 {
        for i in &inputs {
            let (a, b) = rule(i);
            other.push(a);
            if let Some(b) = b {
                other.push(b);
            }
        }
        mem::swap(&mut inputs, &mut other);
        other.clear();
    }

    inputs.len()
}

#[aoc(day11, part2)]
pub fn part2(inputs: &[String]) -> usize {
    let mut data: HashMap<String, usize> = HashMap::new();
    for i in inputs {
        data.insert(i.to_string(), 1);
    }
    for _ in 0..75 {
        let mut counts = HashMap::new();
        for (i, local_count) in data.iter() {
            let (a, b) = rule(i);
            *counts.entry(a.clone()).or_insert(0) += local_count;
            if let Some(b) = b {
                assert!(!b.is_empty());
                *counts.entry(b.clone()).or_insert(0) += local_count;
            }
        }
        data = counts;
    }

    data.values().sum()
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
