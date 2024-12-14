use aoc_runner_derive::{aoc, aoc_generator};
use nom::{bytes::complete::tag, sequence::separated_pair, IResult};

use crate::common::{
    nom::{nom_isize, nom_lines, nom_usize, process_input},
    utils::chinese_remainder_theorem,
};

const HEIGHT: usize = 103;
const WIDTH: usize = 101;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Robot {
    pos: (usize, usize),
    vol: (isize, isize),
}

fn parse_robot(s: &str) -> IResult<&str, Robot> {
    let (s, _) = tag("p=")(s)?;
    let (s, (px, py)) = separated_pair(nom_usize, tag(","), nom_usize)(s)?;
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
    fn simulate_coord(p: usize, v: isize, times: usize, max: usize) -> usize {
        let times = times as isize;
        let max = max as isize;
        let mut p = p as isize;
        p += v * times;
        p = p.rem_euclid(max);
        p.try_into().unwrap()
    }

    fn simulate(&mut self, times: usize, max: (usize, usize)) {
        self.pos = (
            Self::simulate_coord(self.pos.0, self.vol.0, times, max.0),
            Self::simulate_coord(self.pos.1, self.vol.1, times, max.1),
        )
    }
}

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Vec<Robot> {
    process_input(nom_lines(parse_robot))(input)
}

#[allow(dead_code)]
fn print_robots<const H: usize, const W: usize>(robots: &[Robot]) {
    let mut grid = [[b'.'; W]; H];

    for r in robots.iter() {
        grid[r.pos.0][r.pos.1] = b'#';
    }

    for row in grid.iter() {
        println!("{}", unsafe { std::str::from_utf8_unchecked(&row[..]) });
    }
}

fn solve_part1<const H: usize, const W: usize>(mut robots: Vec<Robot>) -> usize {
    robots.iter_mut().for_each(|r| r.simulate(100, (H, W)));

    let mid = (H / 2, W / 2);

    let mut counts = [0; 4];
    for r in robots.iter() {
        if r.pos.0 == mid.0 || r.pos.1 == mid.1 {
            continue;
        }
        let y: usize = (r.pos.0 < mid.0).into();
        let x: usize = (r.pos.1 < mid.1).into();

        counts[y << 1 | x] += 1;
    }
    counts.iter().product()
}

#[aoc(day14, part1)]
pub fn part1(robots: &[Robot]) -> usize {
    solve_part1::<HEIGHT, WIDTH>(robots.to_vec())
}

fn solve_part2<const H: usize, const W: usize>(mut robots: Vec<Robot>) -> usize {
    let mut fpos = Vec::with_capacity(robots.len());

    // Simulate the robot coordinates independently from 1 to the max of the coordinate
    // storing the coordinates as f64 in fpos
    // then calculate the statistical variance of all the positions
    // find the number of steps for the minimal variance
    let min_xstep = (0..W)
        .map(|i| {
            fpos.clear();
            robots.iter_mut().for_each(|r| {
                r.pos.1 = Robot::simulate_coord(r.pos.1, r.vol.1, 1, W);
                fpos.push(r.pos.1 as f64);
            });

            (i + 1, statistical::variance(&fpos, None))
        })
        .reduce(|min, x| if x.1 < min.1 { x } else { min })
        .unwrap()
        .0;

    // repeat for other coordinate
    let min_ystep = (0..H)
        .map(|i| {
            fpos.clear();
            robots.iter_mut().for_each(|r| {
                r.pos.0 = Robot::simulate_coord(r.pos.0, r.vol.0, 1, H);
                fpos.push(r.pos.0 as f64);
            });

            (i + 1, statistical::variance(&fpos, None))
        })
        .reduce(|min, x| if x.1 < min.1 { x } else { min })
        .unwrap()
        .0;

    // chinese remainder theorem
    chinese_remainder_theorem([(min_xstep, W), (min_ystep, H)].into_iter())
}

#[aoc(day14, part2)]
pub fn part2(robots: &[Robot]) -> usize {
    solve_part2::<HEIGHT, WIDTH>(robots.to_vec())
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

    const S_HEIGHT: usize = 7;
    const S_WIDTH: usize = 11;

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(solve_part1::<S_HEIGHT, S_WIDTH>(generator(SAMPLE)), 12);
    }

    #[test]
    pub fn part2_test() {
        let mut robots = generator(SAMPLE);
        let steps = solve_part2::<S_HEIGHT, S_WIDTH>(robots.clone());
        assert_eq!(steps, 24);

        for r in robots.iter_mut() {
            r.simulate(steps, (S_HEIGHT, S_WIDTH));
        }
        println!();
        print_robots::<S_HEIGHT, S_WIDTH>(&robots);
    }

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

            let mut robots = output;
            for r in robots.iter_mut() {
                r.simulate(ANSWERS.1, (HEIGHT, WIDTH));
            }
            println!();
            print_robots::<HEIGHT, WIDTH>(&robots);
        }
    }
}
