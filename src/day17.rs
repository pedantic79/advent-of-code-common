use std::fmt::Write;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{bytes::complete::tag, sequence::preceded, IResult, Parser};

use crate::common::parse::parse_split;

type IntType = u64;

fn nom_int_type(s: &str) -> IResult<&str, IntType> {
    crate::common::nom::nom_u64.parse(s)
}

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
            output: Vec::with_capacity(16),
        }
    }

    fn run_instruction(&mut self, ins: IntType, operand: IntType) -> Option<usize> {
        let combo = self.get_combo(operand);

        // SAFETY: transmute is safe because each Instruction is represented by a u8
        // the input is guaranteed to be a u8 between 0 and 7
        let ins = unsafe { std::mem::transmute::<u8, Instruction>(ins as u8) };

        match ins {
            Instruction::Adv => {
                self.a >>= combo;
            }
            Instruction::Bxl => {
                self.b ^= operand;
            }
            Instruction::Bst => {
                self.b = combo & 7;
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
                self.output.push(combo & 7);
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
            0..4 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("invalid operand"),
        }
    }
}

fn parse_a(s: &str) -> IResult<&str, IntType> {
    let (s, a) = preceded(tag("Register A: "), nom_int_type).parse(s)?;

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

        if let Some(jump) = computer.run_instruction(i, j) {
            // if we are set to loop only once, then immediately return the output when we get to the jump
            if one_shot {
                assert_eq!(computer.output.len(), 1);
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
pub fn part1((a, program): &(IntType, Vec<IntType>)) -> String {
    let output = run_computer(*a, program, false);

    // This avoids allocating a vec, and extra strings.
    let mut buf = String::with_capacity(output.len() * 2 - 1);
    buf.write_fmt(format_args!("{}", output[0])).unwrap();
    for n in &output[1..] {
        buf.write_fmt(format_args!(",{n}")).unwrap();
    }
    buf
}

fn solve_p2(a: IntType, target: &[IntType], ins: &[IntType]) -> Option<IntType> {
    if target.is_empty() {
        // I think most people don't need to do this.
        // I get the wrong output if I don't check.
        return Some(a).filter(|_| run_computer(a, ins, false) == ins);
    }

    let last = target.len() - 1;
    let last_ins = target[last];

    for i in 0..8 {
        let candidate = (a << 3) + i;
        if run_computer(candidate, ins, true)[0] == last_ins {
            if let Some(res) = solve_p2(candidate, &target[0..last], ins) {
                return Some(res);
            }
        }
    }

    None
}

#[aoc(day17, part2)]
pub fn part2((_, program): &(IntType, Vec<IntType>)) -> IntType {
    // the program must contain only one ADV, and that must be ADV 3
    assert_eq!(program.chunks(2).filter(|&w| w[0] == 0).count(), 1);
    assert_eq!(program.chunks(2).filter(|&w| w == [0, 3]).count(), 1);

    // the program must contain only one Out
    assert_eq!(program.chunks(2).filter(|&w| w[0] == 5).count(), 1);

    // the program must end with JNZ 0
    assert_eq!(program[program.len() - 2..], [3, 0]);

    // This only works because the previous asserts are true.
    //
    // The instruction set only has one way of modifying A, via the ADV ins.
    // All other instructions modifying registers modify B or C.
    // So at somepoint we will modify the A register with ADV 3 this shifts A
    // to the right 3
    //
    // There is only one Out instruction, we only output one number per loop
    //
    // then at the end it will jump to the beginning (JNZ 0)
    //
    // so in psuedo-code all programs must look something like this
    //
    // do {
    //    b = (a % 8) ^ 4;
    //    c = a >> b;
    //    b = b ^ c ^ 4;
    //    output(b % 8);   // OUT 5
    //    a = a >> 3;  // ADV 3
    // } while a != 0; // JNZ 0
    //
    // as long as ADV 3 is done, and we output B or C. The ordering isn't important.
    //
    // since we're in a 3-bit machine, we're essentially always outputing one digit
    // based on one 3 bit segment of A
    //
    // to solve this, we will loop through backwards. Looking for an A that
    // produces the value in the program we want. To get that A, we will do A << 3
    // and check all variants 0-7
    solve_p2(0, program, program).unwrap_or(0)
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
        assert_eq!(part2(&(0, vec![0, 3, 5, 4, 3, 0])), 117440);
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
