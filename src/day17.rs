use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::newline, combinator::all_consuming,
    sequence::preceded, IResult, Parser,
};

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
    fn run_instruction(&mut self, ins: Instruction, operand: Instruction) -> Option<usize> {
        let combo = self.get_combo(operand);
        let operand = operand as u8 as IntType;

        match ins {
            Instruction::Adv => {
                self.a /= 2_i64.pow(combo as u32);
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
                self.b = self.a / 2_i64.pow(combo as u32);
            }
            Instruction::Cdv => {
                self.c = self.a / 2_i64.pow(combo as u32);
            }
        }

        None
    }

    fn get_combo(&self, operand: Instruction) -> IntType {
        match operand {
            Instruction::Adv => 0,
            Instruction::Bxl => 1,
            Instruction::Bst => 2,
            Instruction::Jnz => 3,
            Instruction::Bxc => self.a,
            Instruction::Out => self.b,
            Instruction::Bdv => self.c,
            Instruction::Cdv => panic!("invalid operand"),
        }
    }
}

fn parse_computer(s: &str) -> IResult<&str, Computer> {
    let (s, a) = preceded(tag("Register A: "), nom_i64).parse(s)?;
    let (s, _) = newline.parse(s)?;
    let (s, b) = preceded(tag("Register B: "), nom_i64).parse(s)?;
    let (s, _) = newline.parse(s)?;
    let (s, c) = preceded(tag("Register C: "), nom_i64).parse(s)?;

    Ok((
        s,
        Computer {
            a,
            b,
            c,
            output: Vec::new(),
        },
    ))
}

#[aoc_generator(day17)]
pub fn generator(input: &str) -> (Computer, Vec<Instruction>) {
    let (reg, pro) = input.split_once("\n\n").unwrap();
    let computer = all_consuming(parse_computer).parse(reg).unwrap().1;

    let nums = pro.split_once(": ").unwrap().1;
    let program: Vec<u8> = parse_split(nums, ',');

    (computer, unsafe {
        std::mem::transmute::<std::vec::Vec<u8>, std::vec::Vec<Instruction>>(program)
    })
}

fn run_computer(a: IntType, ins: &[Instruction]) -> Vec<IntType> {
    let mut computer = Computer {
        a,
        b: 0,
        c: 0,
        output: Vec::new(),
    };

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
            pc = jump;
        } else {
            pc += 2;
        }
    }

    computer.output
}

#[aoc(day17, part1)]
pub fn part1(inputs: &(Computer, Vec<Instruction>)) -> String {
    let output = run_computer(inputs.0.a, &inputs.1);
    let v = output.into_iter().map(|n| n.to_string()).collect_vec();
    v.join(",")
}

#[aoc(day17, part2)]
pub fn part2(inputs: &(Computer, Vec<Instruction>)) -> IntType {
    // 2,4 = BST 4: B = A % 8
    // 1,4 = BXL 4: B = B ^ 4
    // 7,5 = CDV 5: C = A >> B
    // 4,1 = BXC 1: B = B ^ C
    // 1,4 = BXL 4: B = B ^ 4
    // 5,5 = OUT 5: PRINT B%8
    // 0,3 = ADV 3: A = A >> 3
    // 3,0 = JMP 0: JUMP 0

    fn get_out(a: IntType) -> IntType {
        let b = (a % 8) ^ 4;
        let c = a >> b;
        (b ^ c ^ 4) % 8
    }

    let ins = &inputs.1;
    let ins_inttype: Vec<_> = ins.iter().map(|&x| x as IntType).collect();

    let mut quines = vec![0];
    let mut new_quines = Vec::new();

    for num in ins.iter().rev() {
        let num = *num as IntType;
        new_quines.clear();
        for curr in quines.iter() {
            for i in 0..8 {
                let a = (curr << 3) + i;
                if get_out(a) == num {
                    new_quines.push(a);
                    // println!("{new_quines:?}");
                }
            }
        }
        std::mem::swap(&mut quines, &mut new_quines);
    }

    quines.sort_unstable();

    // println!("{quines:?}");
    quines
        .iter()
        .copied()
        .find(|&x| run_computer(x, ins) == ins_inttype)
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
