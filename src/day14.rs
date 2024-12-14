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

#[aoc(day14, part1)]
pub fn part1(inputs: &[Robot]) -> usize {
    let mut robots = inputs.to_vec();

    robots
        .iter_mut()
        .for_each(|r| r.simulate(100, (HEIGHT, WIDTH)));

    const WIDTH_MID: usize = WIDTH / 2;
    const HEIGHT_MID: usize = HEIGHT / 2;
    const WIDTH_MID1: usize = WIDTH_MID + 1;
    const HEIGHT_MID1: usize = HEIGHT_MID + 1;

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
    const W: isize = WIDTH as isize;
    const H: isize = HEIGHT as isize;

    let mut robots = inputs.to_vec();
    let mut fpos = Vec::with_capacity(robots.len());

    // Simulate the robot coordinates independently from 1 to the max of the coordinate
    // storing the coordinates as f64 in fpos
    // then calculate the statistical variance of all the positions
    // find the number of steps for the minimal variance
    let min_xstep = (1..=W)
        .map(|i| {
            fpos.clear();
            robots.iter_mut().for_each(|r| {
                r.pos.1 = Robot::simulate_coord(r.pos.1, r.vol.1, 1, WIDTH);
                fpos.push(r.pos.1 as f64);
            });

            (i, statistical::variance(&fpos, None))
        })
        .reduce(|min, x| if x.1 < min.1 { x } else { min })
        .unwrap()
        .0;

    // repeat for other coordinate
    let min_ystep = (1..=H)
        .map(|i| {
            fpos.clear();
            robots.iter_mut().for_each(|r| {
                r.pos.0 = Robot::simulate_coord(r.pos.0, r.vol.0, 1, HEIGHT);
                fpos.push(r.pos.0 as f64);
            });

            (i, statistical::variance(&fpos, None))
        })
        .reduce(|min, x| if x.1 < min.1 { x } else { min })
        .unwrap()
        .0;

    // chinese remainder theorem
    let ans = chinese_remainder_theorem([(min_xstep, W), (min_ystep, H)].into_iter());
    ans.try_into().unwrap()
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
