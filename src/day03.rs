use aoc_runner_derive::{aoc, aoc_generator};
use nom::{bytes::complete::tag, IResult};

use crate::common::nom::nom_usize;

#[derive(Debug, PartialEq, Eq)]
pub enum Symbols {
    Mul(usize),
    Do,
    Dont,
}

fn parse_digit(s: &str) -> IResult<&str, usize> {
    let (rest, a) = nom_usize(s)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, b) = nom_usize(rest)?;

    Ok((rest, a * b))
}

fn parse(s: &str) -> Option<(&str, Symbols)> {
    let b = s.as_bytes();
    let mut i = 0;
    while i < s.len() {
        match b[i] {
            b'd' if b[i..].starts_with(b"don't()") => return Some((&s[i + 7..], Symbols::Dont)),
            b'd' if b[i..].starts_with(b"do()") => return Some((&s[i + 4..], Symbols::Do)),
            b'm' if b[i..].starts_with(b"mul(") => {
                i += 4;
                let rest = &s[i..];
                if let Ok((rest, m)) = parse_digit(rest) {
                    if rest.as_bytes()[0] == b')' {
                        return Some((&rest[1..], Symbols::Mul(m)));
                    }
                }
                continue;
            }
            _ => {}
        }
        i += 1;
    }

    None
}

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<Symbols> {
    let mut input = input;
    let mut v = Vec::new();
    while let Some((i, m)) = parse(input) {
        v.push(m);
        input = i;
    }

    v
}

#[aoc(day3, part1)]
pub fn part1(inputs: &[Symbols]) -> usize {
    inputs
        .iter()
        .map(|i| if let Symbols::Mul(x) = i { *x } else { 0 })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(inputs: &[Symbols]) -> usize {
    let mut enabled = true;
    inputs
        .iter()
        .map(|i| {
            match i {
                Symbols::Mul(x) if enabled => return *x,
                Symbols::Do => enabled = true,
                Symbols::Dont => enabled = false,
                _ => {}
            }
            0
        })
        .sum()
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
        assert_eq!(part2(&generator(SAMPLE2)), 48);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day3.txt");
        const ANSWERS: (usize, usize) = (182780583, 90772405);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
