use std::collections::VecDeque;

use ahash::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::{
    matrix::directions::{self, DIRECTIONS_4},
    utils::move_in_direction,
};

use crate::common::utils::build_array;

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn check_perimeter(inputs: &[Vec<char>], r: usize, c: usize, cell: char) -> usize {
    let row_max = inputs.len();
    let col_max = inputs[0].len();

    DIRECTIONS_4
        .iter()
        .map(|&delta| {
            move_in_direction((r, c), delta, (row_max, col_max))
                .and_then(|(r, c)| inputs.get(r).and_then(|row| row.get(c)))
                .copied()
        })
        .filter(|&x| x != Some(cell))
        .count()
}

const CORNERS: [[(isize, isize); 3]; 4] = [
    [directions::E, directions::S, directions::SE],
    [directions::S, directions::W, directions::SW],
    [directions::W, directions::N, directions::NW],
    [directions::N, directions::E, directions::NE],
];

fn check_corner(inputs: &[Vec<char>], r: usize, c: usize, cell: char) -> usize {
    let row_max = inputs.len();
    let col_max = inputs[0].len();
    let cell = Some(cell);
    let mut count = 0;

    for corner in CORNERS.iter() {
        let neighbors = build_array::<Option<char>, _, 3>(corner.iter().map(|&delta| {
            move_in_direction((r, c), delta, (row_max, col_max))
                .and_then(|(r, c)| inputs.get(r).and_then(|row| row.get(c)))
                .copied()
        }));

        if neighbors[0] != cell && neighbors[1] != cell {
            count += 1;
        }

        if neighbors[0] == cell && neighbors[1] == cell && neighbors[2] != cell {
            count += 1;
        }
    }

    count
}

#[aoc(day12, part1)]
pub fn part1(inputs: &[Vec<char>]) -> usize {
    let mut seen = HashSet::default();
    let r_max = inputs.len();
    let c_max = inputs[0].len();
    let mut count = 0;

    // eprintln!("{inputs:?}");

    for (r, row) in inputs.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            // eprintln!("{cell}");

            if seen.contains(&(r, c)) {
                continue;
            }
            seen.insert((r, c));

            let (mut area, mut perim) = (1, check_perimeter(inputs, r, c, cell));
            let mut queue: VecDeque<(usize, usize)> = DIRECTIONS_4
                .iter()
                .filter_map(|&delta| move_in_direction((r, c), delta, (r_max, c_max)))
                .filter(|delta| inputs.get(delta.0).and_then(|row| row.get(delta.1)) == Some(&cell))
                .collect();

            while let Some((y, x)) = queue.pop_front() {
                if seen.contains(&(y, x)) {
                    continue;
                }
                // println!("{y},{x}={:?}", inputs.get(y).and_then(|row| row.get(x)));

                seen.insert((y, x));
                area += 1;
                perim += check_perimeter(inputs, y, x, cell);
                queue.extend(
                    DIRECTIONS_4
                        .iter()
                        .filter_map(|&delta| move_in_direction((y, x), delta, (r_max, c_max)))
                        .filter(|delta| {
                            inputs.get(delta.0).and_then(|row| row.get(delta.1)) == Some(&cell)
                        }),
                )
            }
            // eprintln!("{cell}: {area}*{perim}={}", area * perim);
            count += area * perim;
        }
    }

    count
}

#[aoc(day12, part2)]
pub fn part2(inputs: &[Vec<char>]) -> usize {
    let mut seen = HashSet::default();
    let r_max = inputs.len();
    let c_max = inputs[0].len();
    let mut count = 0;

    for (r, row) in inputs.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if seen.contains(&(r, c)) {
                continue;
            }
            seen.insert((r, c));

            let (mut area, mut perim) = (1, check_corner(inputs, r, c, cell));
            let mut queue: VecDeque<(usize, usize)> = DIRECTIONS_4
                .iter()
                .filter_map(|&delta| move_in_direction((r, c), delta, (r_max, c_max)))
                .filter(|delta| inputs.get(delta.0).and_then(|row| row.get(delta.1)) == Some(&cell))
                .collect();

            while let Some((y, x)) = queue.pop_front() {
                if seen.contains(&(y, x)) {
                    continue;
                }

                seen.insert((y, x));
                area += 1;
                perim += dbg!(check_corner(inputs, y, x, cell));
                queue.extend(
                    DIRECTIONS_4
                        .iter()
                        .filter_map(|&delta| move_in_direction((y, x), delta, (r_max, c_max)))
                        .filter(|delta| {
                            inputs.get(delta.0).and_then(|row| row.get(delta.1)) == Some(&cell)
                        }),
                )
            }
            count += area * perim;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 1930);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 1206);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day12.txt");
        const ANSWERS: (usize, usize) = (1451030, 859494);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
