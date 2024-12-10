use ahash::{HashSet, HashSetExt};
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::{count_paths, dfs_reach};

use crate::common::utils::neighbors;

#[derive(Debug, PartialEq, Eq)]
pub struct Object {}

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<Vec<u8>> {
    let mut res = Vec::new();

    for row in input.lines() {
        let mut output_row = Vec::new();
        for cell in row.bytes() {
            if cell == b'.' {
                output_row.push(255);
            } else {
                output_row.push(cell - b'0');
            }
        }
        res.push(output_row);
    }

    res
}

#[aoc(day10, part1)]
pub fn part1(inputs: &[Vec<u8>]) -> usize {
    let r_max = inputs.len();
    let c_max = inputs[0].len();
    let mut count = 0;

    for (r, row) in inputs.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == 0 {
                let mut endpoints = HashSet::new();

                endpoints.extend(
                    dfs_reach((r, c), |&curr| {
                        neighbors(curr.0, curr.1, r_max, c_max)
                            .filter(move |&(r, c)| inputs[r][c] == inputs[curr.0][curr.1] + 1)
                    })
                    .filter(|x| inputs[x.0][x.1] == 9),
                );
                count += endpoints.len();
            }
        }
    }
    count
}

#[aoc(day10, part2)]
pub fn part2(inputs: &[Vec<u8>]) -> usize {
    let r_max = inputs.len();
    let c_max = inputs[0].len();
    let mut count = 0;

    for (r, row) in inputs.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == 0 {
                count += count_paths(
                    (r, c),
                    |&curr| {
                        neighbors(curr.0, curr.1, r_max, c_max)
                            .filter(move |&(r, c)| inputs[r][c] == inputs[curr.0][curr.1] + 1)
                    },
                    |&curr| inputs[curr.0][curr.1] == 9,
                );
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = r"..90..9
...1.98
...2..7
6543456
765.987
876....
987....
";

    const SAMPLE0: &str = r"...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";

    const SAMPLE2: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE1));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE0)), 2);
        assert_eq!(part1(&generator(SAMPLE1)), 4);
        assert_eq!(part1(&generator(SAMPLE2)), 36);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE2)), 81);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day10.txt");
        const ANSWERS: (usize, usize) = (778, 1925);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
