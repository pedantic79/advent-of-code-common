use ahash::{HashMap, HashMapExt};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iterate, Itertools};

use crate::common::parse::parse_lines;

#[aoc_generator(day22)]
pub fn generator(input: &str) -> Vec<i64> {
    parse_lines(input)
}

fn secret_number(mut s: i64) -> i64 {
    s = ((s * 64) ^ s) % 16777216;
    s = ((s / 32) ^ s) % 16777216;
    s = ((s * 2048) ^ s) % 16777216;
    s
}

#[aoc(day22, part1)]
pub fn part1(inputs: &[i64]) -> i64 {
    inputs
        .iter()
        .map(|n| {
            iterate(*n, |s| secret_number(*s))
                .take(2001)
                .last()
                .unwrap()
        })
        .inspect(|x| println!("{x}"))
        .sum()
}

#[aoc(day22, part2)]
pub fn part2(inputs: &[i64]) -> i64 {
    let v = inputs
        .iter()
        .map(|n| {
            iterate(*n, |s| secret_number(*s))
                .take(2001)
                .map(|n| n % 10)
                .collect_vec()
        })
        .collect_vec();

    let mut scores = HashMap::new();
    for (j, w) in v.iter().enumerate() {
        let mut ans = HashMap::new();
        let diff = w.iter().tuple_windows().map(|(x, y)| y - x).collect_vec();
        for (i, pattern) in diff.windows(4).enumerate() {
            if !ans.contains_key(pattern) {
                ans.insert(pattern, w[i + 4]);
            }
        }

        for (k, v) in ans.iter() {
            *scores.entry(k.to_vec()).or_insert(0) += v;
        }
    }

    scores.values().max().copied().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = r"1
10
100
2024";

    const SAMPLE2: &str = r"1
2
3
2024";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE1));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE1)), 37327623);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE2)), 23);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day22.txt");
        const ANSWERS: (i64, i64) = (16619522798, 1854);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
