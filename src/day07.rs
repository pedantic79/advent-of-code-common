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

fn depth_first_p1(total: usize, target: usize, numbers: &[usize]) -> bool {
    if numbers.is_empty() {
        return total == target;
    }

    if total > target {
        return false;
    }

    let (&first, rest) = numbers.split_first().unwrap();
    depth_first_p1(total * first, target, rest) || depth_first_p1(total + first, target, rest)
}

fn depth_first_p2(total: usize, target: usize, numbers: &[usize]) -> bool {
    if numbers.is_empty() {
        return total == target;
    }

    if total > target {
        return false;
    }

    let (&first, rest) = numbers.split_first().unwrap();
    depth_first_p2(total * first, target, rest)
        || depth_first_p2(total + first, target, rest)
        || depth_first_p2(concat(total, first), target, rest)
}

#[aoc(day7, part1)]
pub fn part1(inputs: &[Math]) -> usize {
    inputs
        .iter()
        .filter(|Math { total, numbers }| depth_first_p1(numbers[0], *total, &numbers[1..]))
        .map(|m| m.total)
        .sum()
}

fn concat(a: usize, b: usize) -> usize {
    let mut shift = 1;

    while shift <= b {
        shift *= 10;
    }

    a * shift + b
}

#[aoc(day7, part2)]
pub fn part2(inputs: &[Math]) -> usize {
    inputs
        .iter()
        .filter(|Math { total, numbers }| depth_first_p2(numbers[0], *total, &numbers[1..]))
        .map(|m| m.total)
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
        assert_eq!(part2(&generator(SAMPLE)), 11387);
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
