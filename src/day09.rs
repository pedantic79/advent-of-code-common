use std::iter;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq)]
pub struct Object {}

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Vec<i64> {
    let mut output = Vec::new();
    let mut file_id = 0;
    for (c, is_file) in input.chars().zip([true, false].into_iter().cycle()) {
        let n = c.to_digit(10).unwrap() as usize;
        if is_file {
            output.extend(iter::repeat(file_id).take(n));
            file_id += 1;
        } else {
            output.extend(iter::repeat(-1).take(n));
        }
    }

    output
}

#[aoc(day9, part1)]
pub fn part1(inputs: &[i64]) -> usize {
    let mut inputs = inputs.to_vec();
    let mut len = inputs.len();
    let mut i = 0;

    while i < len {
        if inputs[i] == -1 {
            inputs.swap(i, len - 1);
            inputs.pop();

            while Some(-1) == inputs.last().copied() {
                inputs.pop();
            }
            len = inputs.len();
        }
        i += 1;
    }

    // println!(">> {:?}", inputs);

    inputs
        .iter()
        .enumerate()
        .map(|(i, c)| (*c as usize) * i)
        .sum()
}

fn variable_window(inputs: &[i64]) -> Vec<(i64, usize)> {
    let mut prev = None;
    let mut count = 0;
    let mut res = Vec::new();

    for &x in inputs.iter() {
        match prev {
            None => {
                prev = Some(x);
                count = 1;
            }
            Some(y) if x == y => {
                count += 1;
            }
            Some(y) => {
                res.push((y, count));
                prev = Some(x);
                count = 1;
            }
        }
    }

    if count > 0 {
        if let Some(prev) = prev {
            res.push((prev, count));
        }
    }

    res
}

pub fn print_window(inputs: &[(i64, usize)]) {
    for &(i, count) in inputs {
        for _ in 0..count {
            if i != -1 {
                print!("{}", i);
            } else {
                print!(".");
            }
        }
    }
}

#[aoc(day9, part2)]
pub fn part2(inputs: &[i64]) -> usize {
    let mut inputs = variable_window(inputs);
    let len = inputs.len();
    let mut i = 0;

    while i < len {
        if inputs[i].0 == -1 {
            for pos in ((i + 1)..len).rev() {
                if inputs[pos].0 != -1 && inputs[pos].1 <= inputs[i].1 {
                    inputs.swap(i, pos);

                    if inputs[pos].1 != inputs[i].1 {
                        // fix length
                        let new_len = inputs[pos].1 - inputs[i].1;
                        inputs[pos].1 = inputs[i].1;
                        inputs.insert(i + 1, (-1, new_len));
                    }
                    // print_window(&inputs);
                    // println!("");
                    // println!("swapping, {} and {}: ", i, pos);
                    break;
                }
            }
        }
        i += 1;
    }

    let mut sum = 0;
    let mut pos = 0;
    for (i, count) in inputs {
        for _ in 0..count {
            if i != -1 {
                sum += i * pos
            }
            pos += 1;
        }
    }

    sum as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"2333133121414131402";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 1928);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 2858);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day9.txt");
        const ANSWERS: (usize, usize) = (6288599492129, 6321896265143);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
