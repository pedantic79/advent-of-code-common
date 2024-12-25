use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day25)]
pub fn generator(input: &str) -> (Vec<[i8; 5]>, Vec<[i8; 5]>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    input.split("\n\n").for_each(|group| {
        let array = group.lines().fold([-1; 5], |mut acc, line| {
            for (cell, b) in acc.iter_mut().zip(line.bytes()) {
                if b == b'#' {
                    *cell += 1;
                }
            }

            acc
        });

        if group.starts_with("#####") {
            locks.push(array)
        } else {
            keys.push(array)
        }
    });

    (locks, keys)
}

#[aoc(day25, part1)]
pub fn part1((locks, keys): &(Vec<[i8; 5]>, Vec<[i8; 5]>)) -> usize {
    locks
        .iter()
        .cartesian_product(keys.iter())
        .filter(|(l, k)| l.iter().zip(k.iter()).all(|(a, b)| a + b <= 5))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 3);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day25.txt");
        const ANSWERS: (usize, usize) = (3466, 0);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
        }
    }
}
