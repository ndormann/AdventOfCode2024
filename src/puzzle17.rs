use crate::prelude::Aoc2024;
use std::fmt::{Debug, Formatter};
use std::str::Lines;

type Literal = u8;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Adv(Combo),
    Bxl(Literal),
    Bst(Combo),
    Jnz(Literal),
    Bxc,
    Out(Combo),
    Bdv(Combo),
    Cdv(Combo),
}

#[derive(Debug, Clone, Copy)]
enum Combo {
    Literal(u8),
    RegA,
    RegB,
    RegC,
    Reserved,
}

impl From<u8> for Combo {
    fn from(value: u8) -> Self {
        match value {
            4 => Combo::RegA,
            5 => Combo::RegB,
            6 => Combo::RegC,
            7 => Combo::Reserved,
            lit => Combo::Literal(lit),
        }
    }
}

impl From<(u8, u8)> for Instruction {
    fn from(value: (u8, u8)) -> Self {
        let (opcode, operand) = value;
        let combo = Combo::from(operand);
        let literal = operand;
        match opcode {
            0 => Instruction::Adv(combo),
            1 => Instruction::Bxl(literal),
            2 => Instruction::Bst(combo),
            3 => Instruction::Jnz(literal),
            4 => Instruction::Bxc,
            5 => Instruction::Out(combo),
            6 => Instruction::Bdv(combo),
            7 => Instruction::Cdv(combo),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
struct Machine {
    a: i64,
    b: i64,
    c: i64,

    ip: usize,

    instructions: Vec<Instruction>,
}

impl Machine {
    fn resolve(&self, combo: &Combo) -> i64 {
        match combo {
            Combo::Literal(lit) => *lit as i64,
            Combo::RegA => self.a,
            Combo::RegB => self.b,
            Combo::RegC => self.c,
            Combo::Reserved => unreachable!(),
        }
    }

    fn step(&mut self, output: &mut Vec<u8>) -> bool {
        let instruction = &self.instructions[self.ip];
        match instruction {
            Instruction::Adv(combo) => {
                self.a >>= self.resolve(combo);
            }
            Instruction::Bxl(lit) => {
                self.b = self.b ^ *lit as i64;
            }
            Instruction::Bst(combo) => {
                self.b = self.resolve(combo) % 8;
            }
            Instruction::Jnz(lit) => {
                if self.a != 0 {
                    self.ip = *lit as usize;
                    return true;
                }
            }
            Instruction::Bxc => {
                self.b = self.b ^ self.c;
            }
            Instruction::Out(combo) => output.push((self.resolve(combo) % 8) as u8),
            Instruction::Bdv(combo) => {
                self.b = self.a >> self.resolve(combo);
            }
            Instruction::Cdv(combo) => {
                self.c = self.a >> self.resolve(combo);
            }
        }
        self.ip += 1;
        self.ip < self.instructions.len()
    }
    pub fn exec(&mut self) -> Vec<u8> {
        let mut output = Vec::new();
        while self.step(&mut output) {}
        output
    }
}

fn reg_line_to_value(lines: &mut Lines) -> i64 {
    lines
        .next()
        .unwrap()
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .parse::<i64>()
        .unwrap()
}

impl From<&String> for Machine {
    fn from(value: &String) -> Self {
        let mut lines = value.lines();
        let a = reg_line_to_value(&mut lines);
        let b = reg_line_to_value(&mut lines);
        let c = reg_line_to_value(&mut lines);
        lines.next();
        let mut target: Vec<u8> = lines
            .next()
            .unwrap()
            .split(",")
            .map(|el| el.strip_prefix("Program: ").unwrap_or(el))
            .filter_map(|el| el.parse::<u8>().ok())
            .collect();

        let mut instructions = Vec::new();
        for i in 0..target.len() / 2 {
            let opcode = target[2 * i];
            let operand = target[2 * i + 1];
            instructions.push(Instruction::from((opcode, operand)));
        }

        target.reverse();

        Self {
            a,
            b,
            c,
            instructions,
            ip: 0,
        }
    }
}

impl Debug for Machine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("A: {}, B: {}, C: {}", self.a, self.b, self.c))?;
        f.write_fmt(format_args!(" IP: {} | {:?}", self.ip, self.instructions))
    }
}

fn output_to_string(output: Vec<u8>) -> String {
    output
        .iter()
        .map(|el| format!("{}", el))
        .collect::<Vec<String>>()
        .join(", ")
}

pub struct Puzzle17 {}

impl Aoc2024 for Puzzle17 {
    fn name(&self) -> String {
        "Day 17: Chronospatial Computer".to_string()
    }

    fn solve_a(&self, input: &String) -> String {
        let machine = Machine::from(input);

        let output = machine.clone().exec();
        output_to_string(output)
    }

    fn solve_b(&self, _input: &String) -> String {
        // use dynamic programming TODO
        "Not implmented".to_string()
    }
}
