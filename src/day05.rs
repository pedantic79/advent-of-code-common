use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};
use bit_set::BitSet;

use crate::common::parse::{parse_split, parse_split_once};

#[derive(Debug, PartialEq, Eq)]
pub struct Object {
    rules: BitSet,
    pages: Vec<Vec<usize>>,
}

fn pair(a: usize, b: usize) -> usize {
    a * 100 + b
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Object {
    let (top, bot) = input.split_once("\n\n").unwrap();
    let rules = top
        .trim_end()
        .lines()
        .map(|s| {
            let (a, b) = parse_split_once(s, '|').unwrap();
            pair(a, b)
        })
        .collect();
    let pages = bot.lines().map(|s| parse_split(s, ',')).collect();

    Object { rules, pages }
}

fn check_page(rules: &BitSet, page: &[usize]) -> bool {
    page.windows(2)
        .all(|win| rules.contains(pair(win[0], win[1])))
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
        if check_page(&inputs.rules, page) {
            continue;
        }

        let mut p = page.to_vec();

        p.sort_by(|&a, &b| {
            if inputs.rules.contains(pair(a, b)) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        if &p != page {
            let middle = p.len() / 2;
            count += p[middle];
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
