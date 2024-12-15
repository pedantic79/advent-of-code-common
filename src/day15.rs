use std::fmt;

use ahash::{HashMap, HashMapExt};
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::{matrix::directions, utils::move_in_direction};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Input {
    warehouse: HashMap<(usize, usize), Block>,
    robot_pos: (usize, usize),
    moves: Option<Vec<(isize, isize)>>,
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
        _ => panic!("unknown direction"),
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.dim.0 {
            for c in 0..self.dim.1 {
                let ch = if let Some(block) = self.warehouse.get(&(r, c)) {
                    match block {
                        Block::Wall => '#',
                        Block::Box => 'O',
                        Block::Robot => '@',
                        Block::LBox => '[',
                        Block::RBox => ']',
                    }
                } else {
                    '.'
                };
                write!(f, "{ch}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Input {
    /// Removes the `moves` from the `Input`. This is useful to allow us
    /// loop over the `moves`, while modifying the `warehouse` at the same time.
    fn take_moves(&mut self) -> Vec<(isize, isize)> {
        self.moves.take().expect("moves already taken")
    }

    fn widen_warehouse(&self) -> Self {
        let mut warehouse = HashMap::with_capacity(self.warehouse.len() * 2 - 1);
        for r in 0..self.dim.0 {
            for c in 0..self.dim.1 {
                if let Some(&block) = self.warehouse.get(&(r, c)) {
                    match block {
                        Block::Wall => {
                            warehouse.insert((r, c * 2), Block::Wall);
                            warehouse.insert((r, c * 2 + 1), Block::Wall);
                        }
                        Block::Box => {
                            warehouse.insert((r, c * 2), Block::LBox);
                            warehouse.insert((r, c * 2 + 1), Block::RBox);
                        }
                        Block::Robot => {
                            warehouse.insert((r, c * 2), Block::Robot);
                        }
                        _ => panic!("can't pass expanded input"),
                    }
                }
            }
        }

        Input {
            warehouse,
            robot_pos: (self.robot_pos.0, self.robot_pos.1 * 2),
            moves: self.moves.clone(),
            dim: (self.dim.0, self.dim.1 * 2),
        }
    }

    /// Swaps the `pos` and `new_pos`, if `sim_only` is true, then we won't modify the warehouse.
    fn raw_move_block(&mut self, pos: (usize, usize), new_pos: (usize, usize), sim_only: bool) {
        assert!(self.warehouse.contains_key(&pos));
        assert!(!self.warehouse.contains_key(&new_pos));

        if sim_only {
            return;
        }

        if let Some(block) = self.warehouse.remove(&pos) {
            self.warehouse.insert(new_pos, block);
        }
    }

    /// Moves a block at `pos` in the direction `dir`. If `sim_only` is true then
    /// only simulate the move, don't modify the state.
    ///
    /// Returns the new position if the move is possible, and returns `None` if
    /// the move fails.
    fn move_block(
        &mut self,
        pos: (usize, usize),
        dir: (isize, isize),
        sim_only: bool, // if true, this doesn't move any blocks
    ) -> Option<(usize, usize)> {
        let target_pos = move_in_direction(pos, dir, self.dim)?;
        let target_block = self.warehouse.get(&target_pos).copied();

        match target_block {
            None => {
                self.raw_move_block(pos, target_pos, sim_only);
            }
            Some(Block::Wall) => return None,
            Some(Block::Robot) => {
                panic!("you're not supposed to move a robot")
            }
            Some(Block::Box) => {
                self.move_block(target_pos, dir, sim_only)?;
                self.raw_move_block(pos, target_pos, sim_only);
            }
            Some(Block::LBox) | Some(Block::RBox) => {
                if dir == directions::E || dir == directions::W {
                    // treat this like part1
                    self.move_block(target_pos, dir, sim_only)?;
                    self.raw_move_block(pos, target_pos, sim_only);
                } else {
                    // Get the direction and expected_linked_block based on the target_block we're looking for
                    let (direction, expected_linked_block) = if target_block == Some(Block::LBox) {
                        (directions::E, Block::RBox)
                    } else {
                        (directions::W, Block::LBox)
                    };

                    // get the linked_block and linked_block_pos of our Box
                    let linked_block_pos =
                        move_in_direction(target_pos, direction, self.dim).unwrap();
                    let linked_block = self.warehouse.get(&linked_block_pos).copied();

                    // the position to our left or right should be match expected_linked_block
                    assert_eq!(linked_block, Some(expected_linked_block));

                    // try to move the object both blocks up, if either fails we will exit quickly
                    self.move_block(linked_block_pos, dir, true)?;
                    self.move_block(target_pos, dir, true)?;
                    if !sim_only {
                        // if we get here, that means the simulation of moves was successful
                        // now do the real moves
                        self.move_block(target_pos, dir, false)?;
                        self.move_block(linked_block_pos, dir, false)?;

                        // only move our current target_block, the target_block will be moved through recursion
                        self.raw_move_block(pos, target_pos, sim_only);
                    }
                }
            }
        }
        Some(target_pos)
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
        warehouse: hm,
        robot_pos,
        moves: Some(moves),
        dim,
    }
}

#[aoc(day15, part1)]
pub fn part1(inputs: &Input) -> usize {
    let mut inputs = inputs.clone();
    let moves = inputs.take_moves();
    let mut robot_pos = inputs.robot_pos;

    for &m in moves.iter() {
        if let Some(rpos) = inputs.move_block(robot_pos, m, false) {
            robot_pos = rpos;
        }
        // print!("{}[2J", 27 as char);
        // println!("Move {:?}:", dir_to_char(m));
        // println!("{}\n", inputs);
        // std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // println!("{}\n", inputs);
    inputs
        .warehouse
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
    let mut inputs = inputs.widen_warehouse();
    let moves = inputs.take_moves();
    let mut robot_pos = inputs.robot_pos;

    for &m in moves.iter() {
        if let Some(rpos) = inputs.move_block(robot_pos, m, false) {
            robot_pos = rpos;
        }
        // print!("{}[2J", 27 as char);
        // println!("Move {:?}:", dir_to_char(m));
        // println!("{}\n", inputs);
        // std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // println!("{}\n", inputs);
    inputs
        .warehouse
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
        assert_eq!(part1(&generator(SAMPLE1)), 908);
        assert_eq!(part1(&generator(SAMPLE0)), 2028);
        assert_eq!(part1(&generator(SAMPLE)), 10092);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE1)), 618);
        assert_eq!(part2(&generator(SAMPLE0)), 1751);
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
