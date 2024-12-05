use std::cmp::Ordering;

use ahash::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};

use crate::common::parse::{parse_split, parse_split_once};

#[derive(Debug, PartialEq, Eq)]
pub struct Object {
    rules: HashSet<(usize, usize)>,
    pages: Vec<Vec<usize>>,
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Object {
    let (top, bot) = input.split_once("\n\n").unwrap();
    let rules = top
        .trim_end()
        .lines()
        .map(|s| parse_split_once(s, '|').unwrap())
        .collect();
    let pages = bot.lines().map(|s| parse_split(s, ',')).collect();

    Object { rules, pages }
}

fn check_page(rules: &HashSet<(usize, usize)>, page: &[usize]) -> bool {
    for (i, &current) in page.iter().enumerate() {
        for &next_page in page[i + 1..].iter() {
            if !rules.contains(&(current, next_page)) {
                return false;
            }
        }
    }

    true
}

#[aoc(day5, part1)]
pub fn part1(inputs: &Object) -> usize {
    let mut count = 0;
    for page in &inputs.pages {
        if check_page(&inputs.rules, page) {
            // look for middle
            // println!("{page:?}")
            let middle = page.len() / 2;
            count += page[middle];
        }
    }

    count
}

#[aoc(day5, part2)]
pub fn part2(inputs: &Object) -> usize {
    let mut count = 0;
    for page in &inputs.pages {
        if !check_page(&inputs.rules, page) {
            let mut page = page.to_vec();

            page.sort_by(|&a, &b| {
                if inputs.rules.contains(&(a, b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });

            let middle = page.len() / 2;
            count += page[middle];
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 143);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 123);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day5.txt");
        const ANSWERS: (usize, usize) = (5275, 6191);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
