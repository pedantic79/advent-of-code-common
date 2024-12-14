use aoc_runner_derive::{aoc, aoc_generator};
use nom::{bytes::complete::tag, sequence::separated_pair, IResult};
use pathfinding::{matrix::directions::DIRECTIONS_4, utils::move_in_direction};

use crate::common::nom::{nom_isize, nom_lines, process_input};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Robot {
    pos: (isize, isize),
    vol: (isize, isize),
}

const HEIGHT_INPUT: isize = 103;
const WIDTH_INPUT: isize = 101;

const HEIGHT_SAMPLE: isize = 7;
const WIDTH_SAMPLE: isize = 11;

fn parse_robot(s: &str) -> IResult<&str, Robot> {
    let (s, _) = tag("p=")(s)?;
    let (s, (px, py)) = separated_pair(nom_isize, tag(","), nom_isize)(s)?;
    let (s, _) = tag(" v=")(s)?;
    let (s, (vx, vy)) = separated_pair(nom_isize, tag(","), nom_isize)(s)?;

    Ok((
        s,
        Robot {
            pos: (py, px),
            vol: (vy, vx),
        },
    ))
}

impl Robot {
    fn simulate(&mut self, times: isize, max: (isize, isize)) {
        let mut p = (
            self.pos.0 + self.vol.0 * times,
            self.pos.1 + self.vol.1 * times,
        );

        p.0 = p.0.rem_euclid(max.0);
        p.1 = p.1.rem_euclid(max.1);

        self.pos = p;
    }
}

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Vec<Robot> {
    process_input(nom_lines(parse_robot))(input)
}

#[aoc(day14, part1)]
pub fn part1(inputs: &[Robot]) -> usize {
    // const WIDTH: isize = WIDTH_SAMPLE;
    // const HEIGHT: isize = HEIGHT_SAMPLE;

    const WIDTH: isize = WIDTH_INPUT;
    const HEIGHT: isize = HEIGHT_INPUT;
    let mut robots = inputs.to_vec();

    robots
        .iter_mut()
        .for_each(|r| r.simulate(100, (HEIGHT, WIDTH)));

    const WIDTH_MID: isize = WIDTH / 2;
    const HEIGHT_MID: isize = HEIGHT / 2;
    const WIDTH_MID1: isize = WIDTH_MID + 1;
    const HEIGHT_MID1: isize = HEIGHT_MID + 1;

    let mut counts = [0; 4];
    for r in robots.iter() {
        #[allow(non_contiguous_range_endpoints)]
        match r.pos {
            (0..HEIGHT_MID, 0..WIDTH_MID) => counts[0] += 1,
            (HEIGHT_MID1..HEIGHT, 0..WIDTH_MID) => counts[1] += 1,
            (HEIGHT_MID1..HEIGHT, WIDTH_MID1..WIDTH) => counts[2] += 1,
            (0..HEIGHT, WIDTH_MID1..WIDTH) => counts[3] += 1,
            _ => {}
        }
    }

    counts.iter().product()
}

#[aoc(day14, part2)]
pub fn part2(inputs: &[Robot]) -> usize {
    const WIDTH: isize = WIDTH_INPUT;
    const HEIGHT: isize = HEIGHT_INPUT;
    let mut robots = inputs.to_vec();
    let mut count: usize = 0;

    loop {
        count += 1;
        let mut grid = [[b'.'; WIDTH as usize]; HEIGHT as usize];
        robots.iter_mut().for_each(|r| {
            r.simulate(1, (HEIGHT, WIDTH));
            grid[r.pos.0 as usize][r.pos.1 as usize] = b'#';
        });

        let mut connected = 0;

        for (r, row) in grid.iter().enumerate() {
            for (c, _) in row.iter().enumerate() {
                if grid[r][c] == b'#'
                    && DIRECTIONS_4
                        .iter()
                        .filter(|&&dir| {
                            move_in_direction((r, c), dir, (HEIGHT as usize, WIDTH as usize))
                                .and_then(|(x, y)| grid.get(x).and_then(|row| row.get(y)))
                                == Some(&b'#')
                        })
                        .count()
                        == 4
                {
                    connected += 1;
                }
            }
        }

        if connected > 10 {
            for row in grid.iter() {
                println!("{}", unsafe { std::str::from_utf8_unchecked(&row[..]) });
            }
            println!("{count}\n\n");
            break count;
        }

        if count > 50000 {
            break 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        // assert_eq!(part1(&generator(SAMPLE)), 12);
    }

    #[test]
    pub fn part2_test() {}

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day14.txt");
        const ANSWERS: (usize, usize) = (232589280, 7569);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
