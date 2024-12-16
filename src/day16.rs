use ahash::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::{
    matrix::directions,
    prelude::{astar_bag_collect, dijkstra},
    utils::move_in_direction,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Object {
    start: (usize, usize),
    end: (usize, usize),
    map: Vec<Vec<u8>>,
    dir: (isize, isize),
}

#[aoc_generator(day16)]
pub fn generator(input: &str) -> Object {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let map = input
        .lines()
        .enumerate()
        .map(|(r, line)| {
            if let Some(c) = line.bytes().position(|b| b == b'E') {
                end = (r, c)
            } else if let Some(c) = line.bytes().position(|b| b == b'S') {
                start = (r, c)
            }

            line.trim_end().to_string().into_bytes()
        })
        .collect();

    Object {
        start,
        end,
        map,
        dir: directions::E,
    }
}

fn choices(dir: (isize, isize)) -> [((isize, isize), usize); 4] {
    match dir {
        directions::N => [
            (directions::W, 1000),
            (directions::E, 1000),
            (directions::S, 2000),
            (directions::N, 1),
        ],
        directions::E => [
            (directions::N, 1000),
            (directions::S, 1000),
            (directions::W, 2000),
            (directions::E, 1),
        ],
        directions::S => [
            (directions::E, 1000),
            (directions::W, 1000),
            (directions::N, 2000),
            (directions::S, 1),
        ],
        directions::W => [
            (directions::N, 1000),
            (directions::S, 1000),
            (directions::E, 2000),
            (directions::W, 1),
        ],
        _ => panic!("unknown direction"),
    }
}

#[aoc(day16, part1)]
pub fn part1(inputs: &Object) -> usize {
    let ans = dijkstra(
        &(inputs.start, inputs.dir),
        |&(pos, dir)| {
            choices(dir)
                .iter()
                .filter_map(move |&(new_dir, cost)| {
                    if cost == 1 {
                        let new_pos =
                            move_in_direction(pos, dir, (inputs.map.len(), inputs.map[0].len()))?;
                        if inputs.map[new_pos.0][new_pos.1] != b'#' {
                            Some(((new_pos, dir), cost))
                        } else {
                            None
                        }
                    } else {
                        Some(((pos, new_dir), cost))
                    }
                })
                .collect::<Vec<_>>()
        },
        |(pos, _)| pos == &inputs.end,
    )
    .unwrap();

    // println!("{:?}", ans.0);

    ans.1
}

#[aoc(day16, part2)]
pub fn part2(inputs: &Object) -> usize {
    let ans = astar_bag_collect(
        &(inputs.start, inputs.dir),
        |&(pos, dir)| {
            choices(dir)
                .iter()
                .filter_map(move |&(new_dir, cost)| {
                    if cost == 1 {
                        let new_pos =
                            move_in_direction(pos, dir, (inputs.map.len(), inputs.map[0].len()))?;
                        if inputs.map[new_pos.0][new_pos.1] != b'#' {
                            Some(((new_pos, dir), cost))
                        } else {
                            None
                        }
                    } else {
                        Some(((pos, new_dir), cost))
                    }
                })
                .collect::<Vec<_>>()
        },
        |_| 0,
        |(pos, _)| pos == &inputs.end,
    )
    .unwrap();

    ans.0
        .iter()
        .flat_map(|v| v.iter().map(|(p, _)| p))
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const SAMPLE1: &str = r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 7036);
        assert_eq!(part1(&generator(SAMPLE1)), 11048);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 45);
        assert_eq!(part2(&generator(SAMPLE1)), 64);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day16.txt");
        const ANSWERS: (usize, usize) = (99460, 500);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
