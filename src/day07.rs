use aoc_runner_derive::{aoc, aoc_generator};

use crate::common::parse::parse_split;

#[derive(Debug, PartialEq, Eq)]
pub struct Math {
    total: usize,
    numbers: Vec<usize>,
}

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<Math> {
    input
        .lines()
        .map(|line| {
            let (total, rest) = line.split_once(": ").unwrap();
            let numbers = parse_split(rest, ' ');
            let total = total.parse().unwrap();
            Math { total, numbers }
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(inputs: &[Math]) -> usize {
    inputs
        .iter()
        .map(|line| {
            if let Some(v) = pathfinding::directed::dfs::dfs(
                (line.numbers[0], &line.numbers[1..]),
                |(total, rest)| {
                    if rest.is_empty() {
                        vec![]
                    } else {
                        vec![(total + rest[0], &rest[1..]), (total * rest[0], &rest[1..])]
                    }
                },
                |(total, rest)| *total == line.total && rest.is_empty(),
            ) {
                line.total
            } else {
                0
            }
        })
        .sum()
}

fn concat(a: usize, b: usize) -> usize {
    str::parse::<usize>(&format!("{a}{b}")).unwrap()
}

#[aoc(day7, part2)]
pub fn part2(inputs: &[Math]) -> usize {
    inputs
        .iter()
        .map(|line| {
            if pathfinding::directed::dfs::dfs(
                (line.numbers[0], &line.numbers[1..]),
                |(total, rest)| {
                    if rest.is_empty() {
                        vec![]
                    } else {
                        vec![
                            (total + rest[0], &rest[1..]),
                            (total * rest[0], &rest[1..]),
                            (concat(*total, rest[0]), &rest[1..]),
                        ]
                    }
                },
                |(total, rest)| *total == line.total && rest.is_empty(),
            )
            .is_some()
            {
                line.total
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Vec<Math>());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 3749);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 336);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day7.txt");
        const ANSWERS: (usize, usize) = (3245122495150, 105517128211543);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
