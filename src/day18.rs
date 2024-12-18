use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::{matrix::directions, prelude::bfs, utils::move_in_direction};

use crate::common::parse::parse_split_once;

const PROBLEM_LIM: usize = 1024;
const PROBLEM_DIM: usize = 71;

#[derive(Debug)]
pub struct Corruptions {
    lookup: Vec<Option<usize>>,
    reverse: Vec<(usize, usize)>, // this is in (x,y)
    height: usize,
    width: usize,
}

impl Corruptions {
    fn get(&self, p: (usize, usize)) -> Option<usize> {
        self.lookup.get(p.0 * self.width + p.1).copied().unwrap()
    }

    fn contains(&self, p: (usize, usize), limit: usize) -> bool {
        // This returns true if there is a corruption at `p` and the corruption occurrs under the limit
        // This returns false if there is no corruption (unwrap_or()) or we are at our limit
        self.get(p).map(|i| i < limit).unwrap_or(false)
    }
}

// Build a lookup forwards mapping coordinates to corruption number, and rule number to coordinate
// note: that reverse is in (x, y) while everywhere else is in (y, x)
fn generic_generator<const H: usize, const W: usize>(input: &str) -> Corruptions {
    let mut lookup = vec![None; H * W];
    let mut reverse = Vec::new();

    for (i, l) in input.lines().enumerate() {
        let (x, y): (usize, usize) = parse_split_once(l, ',').unwrap();
        lookup[y * W + x] = Some(i);
        reverse.push((x, y));
    }

    Corruptions {
        lookup,
        reverse,
        height: H,
        width: W,
    }
}

#[aoc_generator(day18)]
pub fn generator(input: &str) -> Corruptions {
    generic_generator::<PROBLEM_DIM, PROBLEM_DIM>(input)
}

fn solve_part1(corruptions: &Corruptions, limit: usize) -> Option<Vec<(usize, usize)>> {
    bfs(
        &(0, 0),
        |&state| {
            directions::DIRECTIONS_4
                .into_iter()
                .filter_map(move |direction| {
                    move_in_direction(state, direction, (corruptions.height, corruptions.width))
                })
                .filter(|p| !corruptions.contains(*p, limit))
        },
        |&(y, x)| y + 1 == corruptions.height && x + 1 == corruptions.width,
    )
}

#[aoc(day18, part1)]
pub fn part1(corruptions: &Corruptions) -> usize {
    solve_part1(corruptions, PROBLEM_LIM).unwrap().len() - 1
}

fn solve_part2<const LIMIT: usize>(corruptions: &Corruptions) -> String {
    let mut min = LIMIT + 1;
    let mut max = corruptions.reverse.len();

    while min != max {
        let mid = min + (max - min) / 2;
        if solve_part1(corruptions, mid).is_some() {
            min = mid + 1;
        } else {
            max = mid;
        }
    }

    let ans = corruptions.reverse[max - 1];
    format!("{},{}", ans.0, ans.1)
}

#[aoc(day18, part2)]
pub fn part2(corruptions: &Corruptions) -> String {
    solve_part2::<PROBLEM_LIM>(corruptions)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DIM: usize = 7;
    const SAMPLE_LIM: usize = 12;
    const SAMPLE: &str = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    pub fn input_test() {
        // println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(
            solve_part1(
                &generic_generator::<SAMPLE_DIM, SAMPLE_DIM>(SAMPLE),
                SAMPLE_LIM
            )
            .unwrap()
            .len()
                - 1,
            22
        );
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(
            solve_part2::<SAMPLE_LIM>(&generic_generator::<SAMPLE_DIM, SAMPLE_DIM>(SAMPLE)),
            "6,1"
        );
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day18.txt");
        const ANSWERS: (usize, &str) = (316, "45,18");

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
