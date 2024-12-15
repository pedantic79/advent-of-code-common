use core::fmt;

use ahash::{HashMap, HashMapExt};
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::{matrix::directions, utils::move_in_direction};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Input {
    state: HashMap<(usize, usize), Block>,
    robot_pos: (usize, usize),
    moves: Vec<(isize, isize)>,
    dim: (usize, usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Block {
    Wall,
    Box,
    LBox,
    RBox,
    Robot,
}

#[allow(dead_code)]
fn dir_to_char(dir: (isize, isize)) -> char {
    match dir {
        directions::E => '>',
        directions::N => '^',
        directions::W => '<',
        directions::S => 'v',
        _ => '*',
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.dim.0 {
            for c in 0..self.dim.1 {
                if let Some(block) = self.state.get(&(r, c)) {
                    write!(
                        f,
                        "{}",
                        match block {
                            Block::Wall => "#",
                            Block::Box => "O",
                            Block::Robot => "@",
                            Block::LBox => "[",
                            Block::RBox => "]",
                        }
                    )?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Input {
    fn raw_move_object(&mut self, pos: (usize, usize), new_pos: (usize, usize)) {
        assert!(self.state.contains_key(&pos));
        // assert!(!self.state.contains_key(&new_pos));

        if let Some(block) = self.state.remove(&pos) {
            self.state.insert(new_pos, block);
        }
    }

    fn move_object_p1(
        &mut self,
        pos: (usize, usize),
        dir: (isize, isize),
    ) -> Option<(usize, usize)> {
        let new_pos = move_in_direction(pos, dir, self.dim)?;
        let target_block = self.state.get(&new_pos).copied();

        match target_block {
            None => {
                self.raw_move_object(pos, new_pos);
                Some(new_pos)
            }
            Some(Block::Wall) => None,
            Some(Block::Robot) => {
                unreachable!()
            }
            Some(Block::Box) => {
                self.move_object_p1(new_pos, dir)?;
                self.raw_move_object(pos, new_pos);
                Some(new_pos)
            }
            _ => panic!("this is meant for part 1 only"),
        }
    }

    fn double_wide(&self) -> Self {
        let mut state = HashMap::new();
        for r in 0..self.dim.0 {
            for c in 0..self.dim.1 {
                if let Some(&block) = self.state.get(&(r, c)) {
                    match block {
                        Block::Wall => {
                            state.insert((r, c * 2), Block::Wall);
                            state.insert((r, c * 2 + 1), Block::Wall);
                        }
                        Block::Box => {
                            state.insert((r, c * 2), Block::LBox);
                            state.insert((r, c * 2 + 1), Block::RBox);
                        }
                        Block::Robot => {
                            state.insert((r, c * 2), Block::Robot);
                        }
                        _ => panic!("can't pass expanded input"),
                    }
                }
            }
        }

        Input {
            state,
            robot_pos: (self.robot_pos.0, self.robot_pos.1 * 2),
            moves: self.moves.clone(),
            dim: (self.dim.0, self.dim.1 * 2),
        }
    }

    fn move_object_p2(
        &mut self,
        pos: (usize, usize),
        dir: (isize, isize),
        dryrun: bool,
    ) -> Option<(usize, usize)> {
        let new_pos = move_in_direction(pos, dir, self.dim)?;
        let target_block = self.state.get(&new_pos).copied();

        match target_block {
            None => {
                if !dryrun {
                    self.raw_move_object(pos, new_pos)
                };
                Some(new_pos)
            }
            Some(Block::Wall) => None,
            Some(Block::Robot) => {
                unreachable!()
            }
            Some(Block::LBox) => {
                if dir == directions::E || dir == directions::W {
                    self.move_object_p2(new_pos, dir, dryrun)?;
                    if !dryrun {
                        self.raw_move_object(pos, new_pos);
                    }
                } else {
                    let rbox_pos = move_in_direction(new_pos, directions::E, self.dim).unwrap();
                    let right = self.state.get(&rbox_pos).copied();

                    // the position to our right should be an rbox
                    assert_eq!(right, Some(Block::RBox));

                    // try to move the object up
                    self.move_object_p2(rbox_pos, dir, true)?;
                    self.move_object_p2(new_pos, dir, true)?;
                    if !dryrun {
                        self.move_object_p2(new_pos, dir, false)?;
                        self.move_object_p2(rbox_pos, dir, false)?;
                        self.raw_move_object(pos, new_pos);
                    }
                }

                Some(new_pos)
            }
            Some(Block::RBox) => {
                if dir == directions::E || dir == directions::W {
                    self.move_object_p2(new_pos, dir, dryrun)?;
                    if !dryrun {
                        self.raw_move_object(pos, new_pos);
                    }
                } else {
                    let lbox_pos = move_in_direction(new_pos, directions::W, self.dim).unwrap();
                    let right = self.state.get(&lbox_pos).copied();

                    // the position to our right should be an rbox
                    assert_eq!(right, Some(Block::LBox));

                    // try to move the object up
                    self.move_object_p2(lbox_pos, dir, true)?;
                    self.move_object_p2(new_pos, dir, true)?;
                    if !dryrun {
                        self.move_object_p2(lbox_pos, dir, false)?;
                        self.move_object_p2(new_pos, dir, false)?;
                        self.raw_move_object(pos, new_pos);
                    }
                }

                Some(new_pos)
            }
            _ => panic!("this is meant for part 2 only"),
        }
    }
}

#[aoc_generator(day15)]
pub fn generator(input: &str) -> Input {
    let (warehouse, moves) = input.split_once("\n\n").unwrap();
    let mut hm = HashMap::new();
    let mut robot_pos = (0, 0);
    let mut dim = (0, 0);

    for (r, line) in warehouse.lines().enumerate() {
        for (c, cell) in line.bytes().enumerate() {
            match cell {
                b'#' => {
                    hm.insert((r, c), Block::Wall);
                }
                b'O' => {
                    hm.insert((r, c), Block::Box);
                }
                b'@' => {
                    robot_pos = (r, c);
                    hm.insert((r, c), Block::Robot);
                }
                _ => {}
            }
            dim.1 = c;
        }
        dim.0 = r;
    }
    dim.0 += 1;
    dim.1 += 1;

    let moves = moves
        .bytes()
        .filter(|b| *b != b'\n')
        .map(|b| match b {
            b'^' => directions::N,
            b'>' => directions::E,
            b'v' => directions::S,
            b'<' => directions::W,
            _ => panic!("unknown move"),
        })
        .collect();

    Input {
        state: hm,
        robot_pos,
        moves,
        dim,
    }
}

#[aoc(day15, part1)]
pub fn part1(inputs: &Input) -> usize {
    let moves = &inputs.moves;
    let mut inputs = inputs.clone();
    let mut robot_pos = inputs.robot_pos;

    for &m in moves {
        if let Some(rpos) = inputs.move_object_p1(robot_pos, m) {
            robot_pos = rpos;
        }
        // println!("Move {:?}:", dir_to_char(m));
        // println!("{}\n", inputs);
    }

    // println!("{}\n", inputs);
    inputs
        .state
        .iter()
        .map(|(pos, block)| {
            if block == &Block::Box {
                pos.0 * 100 + pos.1
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day15, part2)]
pub fn part2(inputs: &Input) -> usize {
    let inputs = inputs.double_wide();

    let moves = &inputs.moves;
    let mut inputs = inputs.clone();
    let mut robot_pos = inputs.robot_pos;

    for &m in moves {
        // println!("Move {:?}:", dir_to_char(m));
        if let Some(rpos) = inputs.move_object_p2(robot_pos, m, false) {
            robot_pos = rpos;
        }
        // println!("{}\n", inputs);
    }

    // println!("{}\n", inputs);
    inputs
        .state
        .iter()
        .map(|(pos, block)| {
            if block == &Block::LBox {
                pos.0 * 100 + pos.1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE0: &str = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const SAMPLE: &str = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const SAMPLE1: &str = r"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE0));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE0)), 2028);
        assert_eq!(part1(&generator(SAMPLE)), 10092);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 9021);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day15.txt");
        const ANSWERS: (usize, usize) = (1436690, 1482350);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
