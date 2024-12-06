use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::matrix::directions;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Guard {
    y: usize,
    x: usize,
    dir: (isize, isize),
}

impl Guard {
    fn turn_right(&mut self) {
        self.dir = match self.dir {
            directions::N => directions::E,
            directions::E => directions::S,
            directions::S => directions::W,
            directions::W => directions::N,
            _ => panic!("unknown direction {:?}", self.dir),
        };
    }

    fn pre_increment(&self) -> Option<(usize, usize)> {
        Some((
            self.y.checked_add_signed(self.dir.0)?,
            self.x.checked_add_signed(self.dir.1)?,
        ))
    }

    fn increment(&mut self) -> Option<(usize, usize)> {
        let (y, x) = self.pre_increment()?;
        self.y = y;
        self.x = x;

        Some((y, x))
    }
}

#[aoc_generator(day6)]
pub fn generator(input: &str) -> (Vec<Vec<u8>>, Guard) {
    let mut y = 0;
    let mut x = 0;
    let mut output_grid = Vec::new();

    for (r, row) in input.lines().enumerate() {
        let mut output_row = Vec::with_capacity(row.len());
        for (c, cell) in row.bytes().enumerate() {
            if cell == b'^' {
                y = r;
                x = c;
                output_row.push(b'X');
            } else {
                output_row.push(cell);
            }
        }
        output_grid.push(output_row);
    }

    (
        output_grid,
        Guard {
            y,
            x,
            dir: directions::N,
        },
    )
}

#[aoc(day6, part1)]
pub fn part1(inputs: &(Vec<Vec<u8>>, Guard)) -> usize {
    let mut map = inputs.0.to_vec();
    let mut guard = inputs.1;

    while let Some((y, x)) = guard.pre_increment() {
        if let Some(&cell) = map.get(y).and_then(|row| row.get(x)) {
            if cell != b'#' {
                map[y][x] = b'X';
                guard.increment();
            } else {
                guard.turn_right();
            }
        } else {
            break;
        }
    }

    map.iter()
        .map(|row| row.iter().filter(|&&b| b == b'X').count())
        .sum()
}

#[aoc(day6, part2)]
pub fn part2(inputs: &(Vec<Vec<u8>>, Guard)) -> usize {
    let map = &inputs.0;
    let guard = inputs.1;
    let mut count = 0;

    for (o_y, o_row) in map.iter().enumerate() {
        for (o_x, &o_cell) in o_row.iter().enumerate() {
            let mut seen: HashSet<(usize, usize, (isize, isize))> = HashSet::new();
            let mut guard = guard;
            if o_cell == b'#' {
                continue;
            }
            while let Some((y, x)) = guard.pre_increment() {
                if let Some(&cell) = map.get(y).and_then(|row| row.get(x)) {
                    let cell = if o_y == y && o_x == x { b'#' } else { cell };
                    if cell != b'#' {
                        if !seen.insert((y, x, guard.dir)) {
                            count += 1;
                            break;
                        }
                        guard.increment();
                    } else {
                        guard.turn_right();
                    }
                } else {
                    break;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 41);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 6);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day6.txt");
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
