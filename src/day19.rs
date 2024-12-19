use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::prelude::{bfs, count_paths};

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    towels: Vec<String>,
    designs: Vec<String>,
}

impl Input {
    fn solve(&self, towels: String) -> Option<Vec<String>> {
        bfs(
            &towels,
            |s: &String| {
                self.towels
                    .iter()
                    .filter_map(move |t| match_towels(t, s).map(|s| s.to_string()))
                    .collect_vec()
            },
            |s: &String| s.is_empty(),
        )
    }

    fn solve_2(&self, towels: String) -> usize {
        count_paths(
            towels,
            |s: &String| {
                self.towels
                    .iter()
                    .filter_map(move |t| match_towels(t, s).map(|s| s.to_string()))
                    .collect_vec()
            },
            |s: &String| s.is_empty(),
        )
    }
}

#[aoc_generator(day19)]
pub fn generator(input: &str) -> Input {
    let (a, b) = input.split_once("\n\n").unwrap();

    let towels = a.split(", ").map(|s| s.to_string()).collect();
    let designs = b.lines().map(|s| s.to_string()).collect();

    Input { towels, designs }
}

fn match_towels<'a>(towels: &'a str, design: &'a str) -> Option<&'a str> {
    design.strip_prefix(towels)
}

#[aoc(day19, part1)]
pub fn part1(inputs: &Input) -> usize {
    inputs
        .designs
        .iter()
        .filter(|s| inputs.solve(s.to_string()).is_some())
        .count()
}

#[aoc(day19, part2)]
pub fn part2(inputs: &Input) -> usize {
    inputs
        .designs
        .iter()
        .map(|s| inputs.solve_2(s.to_string()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 6);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 16);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day19.txt");
        const ANSWERS: (usize, usize) = (0, 0);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            // let output = generator(input);

            // assert_eq!(part1(&output), ANSWERS.0);
            // assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
