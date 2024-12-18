use ahash::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::{matrix::directions, prelude::bfs, utils::move_in_direction};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::common::parse::parse_split_once;

#[aoc_generator(day18)]
pub fn generator(input: &str) -> HashMap<(usize, usize), usize> {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| (parse_split_once(l, ',').unwrap(), i))
        .collect()
}

fn solve_part1<const H: usize, const W: usize>(
    corruptions: &HashMap<(usize, usize), usize>,
    limit: usize,
) -> Option<Vec<(usize, usize)>> {
    bfs(
        &(0, 0),
        |&state| {
            directions::DIRECTIONS_4
                .into_iter()
                .filter_map(move |direction| move_in_direction(state, direction, (H, W)))
                .filter(|p| !corruptions.get(p).map(|x| *x < limit).unwrap_or(false))
        },
        |&(y, x)| y + 1 == H && x + 1 == W,
    )
}

#[aoc(day18, part1)]
pub fn part1(corruptions: &HashMap<(usize, usize), usize>) -> usize {
    solve_part1::<71, 71>(corruptions, 1024).unwrap().len() - 1
}

#[aoc(day18, part2)]
pub fn part2(corruptions: &HashMap<(usize, usize), usize>) -> String {
    let ans = (1025..corruptions.len())
        .into_par_iter()
        .find_first(|&cand| solve_part1::<71, 71>(corruptions, cand).is_none())
        .map(|cand| corruptions.iter().find(|x| *x.1 == cand - 1).unwrap().0)
        .copied()
        .unwrap();

    format!("{},{}", ans.0, ans.1)
}

#[cfg(test)]
mod tests {
    use super::*;

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
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(
            solve_part1::<7, 7>(&generator(SAMPLE), 12).unwrap().len() - 1,
            22
        );
    }

    #[test]
    pub fn part2_test() {

        // assert_eq!(part2(&generator(SAMPLE)), 336);
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
