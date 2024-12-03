use aoc_runner_derive::{aoc, aoc_generator};
use nom::{bytes::complete::tag, IResult};
use regex::Regex;

use crate::common::nom::nom_usize;

#[derive(Debug, PartialEq, Eq)]
pub struct Mul(usize, usize);

fn parse_digit(s: &str) -> IResult<&str, Mul> {
    let (rest, a) = nom_usize(s)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, b) = nom_usize(rest)?;

    Ok((rest, Mul(a, b)))
}

fn parse_mul(s: &str) -> IResult<&str, Mul> {
    let (s, _) = tag("mul(")(s)?;
    let (s, a) = nom_usize(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, b) = nom_usize(s)?;
    let (s, _) = tag(")")(s)?;

    Ok((s, Mul(a, b)))
}

fn parse(s: &str) -> Option<(&str, Mul)> {
    if let Some(idx) = s.find("mul(") {
        let rest = &s[(idx + 4)..];
        if let Ok((rest, m)) = parse_digit(rest) {
            if rest.as_bytes()[0] == b')' {
                Some((&rest[1..], m))
            } else {
                parse(&rest[1..])
            }
        } else {
            parse(rest)
        }
    } else {
        None
    }
}

#[aoc_generator(day3, part1)]
pub fn generator(input: &str) -> Vec<Mul> {
    let mut input = input;
    let mut v = Vec::new();
    while let Some((i, m)) = parse(input) {
        v.push(m);
        input = i;
    }

    v
}

#[aoc_generator(day3, part2)]
pub fn generator2(input: &str) -> Vec<Mul> {
    let r = Regex::new(r"(do\(\)|don't\(\)|mul\(\d+,\d+\))").unwrap();
    let mut include = true;
    let mut v = Vec::new();
    for m in r.find_iter(input) {
        if m.as_str() == "do()" {
            include = true;
        } else if m.as_str() == "don't()" {
            include = false;
        } else if include {
            v.push(parse_mul(m.as_str()).unwrap().1)
        }
    }

    v
}

#[aoc(day3, part1)]
pub fn part1(inputs: &[Mul]) -> usize {
    // println!("{}", inputs.len());
    inputs.iter().map(|Mul(a, b)| a * b).sum()
}

#[aoc(day3, part2)]
pub fn part2(inputs: &[Mul]) -> usize {
    inputs.iter().map(|Mul(a, b)| a * b).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const SAMPLE2: &str =
        r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 161);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE2)), 161);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day3.txt");
        const ANSWERS: (usize, usize) = (182780583, 90772405);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            let output2 = generator2(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output2), ANSWERS.1);
        }
    }
}
