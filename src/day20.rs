use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::{matrix::directions, prelude::bfs, utils::move_in_direction};

type Position = (usize, usize);

#[derive(Debug, PartialEq, Eq)]
pub struct Maze {
    start: Position,
    end: Position,
    map: Vec<Vec<u8>>,
}

impl Maze {
    fn get(&self, r: usize, c: usize) -> Option<u8> {
        self.map.get(r).and_then(|row| row.get(c)).copied()
    }

    fn get_pos(&self, pos: (usize, usize)) -> Option<u8> {
        self.get(pos.0, pos.1)
    }
}

#[aoc_generator(day20)]
pub fn generator(input: &str) -> Maze {
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

    Maze { start, end, map }
}

fn solve<const CHEAT: usize, const LIMIT: usize>(maze: &Maze) -> usize {
    let path = bfs(
        &maze.start,
        |&pos| {
            directions::DIRECTIONS_4
                .into_iter()
                .filter_map(move |dir| {
                    move_in_direction(pos, dir, (maze.map.len(), maze.map[0].len()))
                })
                .filter(|new_pos| maze.get_pos(*new_pos) != Some(b'#'))
        },
        |pos| pos.0 == maze.end.0 && pos.1 == maze.end.1,
    )
    .unwrap();

    let mut count = 0;

    for (i, first) in path.iter().enumerate() {
        for (j, second) in path.iter().enumerate().skip(i + LIMIT + 2) {
            let dist = first.0.abs_diff(second.0) + first.1.abs_diff(second.1);
            if dist <= CHEAT && (j - i) - dist >= LIMIT {
                count += 1;
            }
        }
    }

    count
}

#[aoc(day20, part1)]
pub fn part1(maze: &Maze) -> usize {
    solve::<2, 100>(maze)
}

#[aoc(day20, part2)]
pub fn part2(maze: &Maze) -> usize {
    solve::<20, 100>(maze)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(solve::<2, 2>(&generator(SAMPLE)), 44);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(solve::<20, 50>(&generator(SAMPLE)), 285);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day20.txt");
        const ANSWERS: (usize, usize) = (1485, 1027501);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
