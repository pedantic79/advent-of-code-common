use std::fmt::Write;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{bytes::complete::tag, sequence::preceded, IResult, Parser};

use crate::common::{nom::nom_i64, parse::parse_split};

type IntType = i64;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Computer {
    a: IntType,
    b: IntType,
    c: IntType,
    output: Vec<IntType>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Instruction {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl Computer {
    fn new(a: IntType) -> Self {
        Self {
            a,
            b: 0,
            c: 0,
            output: Vec::new(),
        }
    }
    fn run_instruction(&mut self, ins: IntType, operand: IntType) -> Option<usize> {
        let combo = self.get_combo(operand);

        let ins = unsafe { std::mem::transmute::<u8, Instruction>(ins as u8) };

        match ins {
            Instruction::Adv => {
                self.a >>= combo;
            }
            Instruction::Bxl => {
                self.b ^= operand;
            }
            Instruction::Bst => {
                self.b = combo % 8;
            }
            Instruction::Jnz => {
                if self.a != 0 {
                    return Some(operand as usize);
                }
            }
            Instruction::Bxc => {
                self.b ^= self.c;
            }
            Instruction::Out => {
                self.output.push(combo % 8);
            }
            Instruction::Bdv => {
                self.b = self.a >> combo;
            }
            Instruction::Cdv => {
                self.c = self.a >> combo;
            }
        }

        None
    }

    fn get_combo(&self, operand: IntType) -> IntType {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("invalid operand"),
        }
    }
}

fn parse_a(s: &str) -> IResult<&str, IntType> {
    let (s, a) = preceded(tag("Register A: "), nom_i64).parse(s)?;

    Ok((s, a))
}

#[aoc_generator(day17)]
pub fn generator(input: &str) -> (IntType, Vec<IntType>) {
    let (reg, pro) = input.split_once("\n\n").unwrap();
    let a = parse_a.parse(reg).unwrap().1;
    let nums = pro.split_once(": ").unwrap().1;
    let program = parse_split(nums, ',');

    (a, program)
}

fn run_computer(a: IntType, ins: &[IntType], one_shot: bool) -> Vec<IntType> {
    let mut computer = Computer::new(a);

    let mut pc = 0;
    while pc + 1 < ins.len() {
        let i = ins[pc];
        let j = ins[pc + 1];

        // println!(
        //     "{pc} {} {} {}\n {computer:?}",
        //     i as u8,
        //     j as u8,
        //     computer.get_combo(j)
        // );
        if let Some(jump) = computer.run_instruction(i, j) {
            if one_shot {
                return computer.output;
            }
            pc = jump;
        } else {
            pc += 2;
        }
    }

    computer.output
}

#[aoc(day17, part1)]
pub fn part1(inputs: &(IntType, Vec<IntType>)) -> String {
    let output = run_computer(inputs.0, &inputs.1, false);

    // This avoids allocating a vec, and extra strings.
    let mut buf = String::with_capacity(output.len() * 2 - 1);
    buf.write_fmt(format_args!("{}", output[0])).unwrap();
    for n in &output[1..] {
        buf.write_fmt(format_args!(",{n}")).unwrap();
    }
    buf
}

#[aoc(day17, part2)]
pub fn part2(inputs: &(IntType, Vec<IntType>)) -> IntType {
    let ins = &inputs.1;

    // See: https://en.wikipedia.org/wiki/Quine_(computing)
    let mut quines = vec![0];
    let mut new_quines = Vec::new();

    for num in ins.iter().rev() {
        let num = *num as IntType;
        new_quines.clear();
        for curr in quines.iter() {
            for i in 0..8 {
                let a = (curr << 3) + i;
                if run_computer(a, ins, true)[0] == num {
                    new_quines.push(a);
                    // println!("{new_quines:?}");
                }
            }
        }
        std::mem::swap(&mut quines, &mut new_quines);
    }

    // using stable sort, because the quines may already be sorted.
    quines.sort();

    quines
        .iter()
        .copied()
        .find(|&x| &run_computer(x, ins, false) == ins)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    pub fn part2_test() {
        // assert_eq!(part2(&generator(SAMPLE)), 336);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day17.txt");
        const ANSWERS: (&str, IntType) = ("7,0,7,3,4,1,3,0,1", 156985331222018);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
