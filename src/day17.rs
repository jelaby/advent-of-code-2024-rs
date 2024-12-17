use crate::days;
use itertools::Itertools;
use num::pow;
use regex::Regex;

pub struct Day;

impl Day {}

struct Machine {
    a: i32,
    b: i32,
    c: i32,
    i: i32,
    output: Vec<i32>,
}

impl Machine {
    fn combo(&self, operand: i32) -> i32 {
        if operand <= 3 {
            operand
        } else if operand == 4 {
            self.a
        } else if operand == 5 {
            self.b
        } else if operand == 6 {
            self.c
        } else {
            panic!("Invalid combo operand: {operand}");
        }
    }
}

enum OpCode {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl OpCode {
    fn execute(&self, machine: &mut Machine, operand: i32) {
        match self {
            OpCode::ADV => machine.a = machine.a / pow(2, machine.combo(operand) as usize),
            OpCode::BXL => machine.b = machine.b ^ operand,
            OpCode::BST => machine.b = operand & 0x7,
            OpCode::JNZ => {
                if machine.a != 0 {
                    machine.i = operand - 2
                }
            }
            OpCode::BXC => machine.b = machine.b & machine.c,
            OpCode::OUT => machine.output.push(machine.combo(operand) & 0x7),
            OpCode::BDV => machine.b = machine.a / pow(2, machine.combo(operand) as usize),
            OpCode::CDV => machine.c = machine.a / pow(2, machine.combo(operand) as usize),
        }
    }
}

fn opcode_for(n: i32) -> OpCode {
    match n {
        0 => OpCode::ADV,
        1 => OpCode::BXL,
        2 => OpCode::BST,
        3 => OpCode::JNZ,
        4 => OpCode::BXC,
        5 => OpCode::OUT,
        6 => OpCode::BDV,
        7 => OpCode::CDV,
        _ => panic!("Unexpected opcode: {n}"),
    }
}

fn parse_register(input: &str) -> i32 {
    Regex::new(r"Register \w: (\d+)")
        .ok()
        .and_then(|r| r.captures(input))
        .and_then(|c| c.get(1))
        .and_then(|v| v.as_str().parse().ok())
        .unwrap()
}

fn parse_program(input: &str) -> Vec<i32> {
    Regex::new(r"Program: (.*)")
        .ok()
        .and_then(|r| r.captures(input))
        .and_then(|c| c.get(1))
        .map(|v| {
            v.as_str()
                .split(",")
                .map(|o| o.parse::<i32>().expect(&format!("Failed to parse {o}")))
                .collect()
        })
        .unwrap()
}

fn parse(input: &str) -> (i32, i32, i32, Vec<i32>) {
    let mut lines = input.lines();

    let a = parse_register(lines.next().unwrap());
    let b = parse_register(lines.next().unwrap());
    let c = parse_register(lines.next().unwrap());

    let _ = lines.next();

    let program = parse_program(lines.next().unwrap());

    (a, b, c, program)
}

fn run(machine: &mut Machine, input: Vec<i32>) {
    while machine.i < input.len() as i32 {
        opcode_for(input[machine.i as usize]).execute(machine, input[(machine.i + 1) as usize]);

        machine.i += 2;
    }
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        17
    }

    fn part1(&self, input: &str) -> Option<String> {
        let (a, b, c, mut program) = parse(input);

        let mut machine = Machine {
            a,
            b,
            c,
            i: 0,
            output: vec![],
        };

        run(&mut machine, program);

        Some(machine.output.iter().map(|n| n.to_string()).join(","))
    }
    fn part2(&self, input: &str) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;

    const DAY: super::Day = super::Day;
    #[test]
    fn part1_example1() {
        let text = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!(DAY.part1(text), Some("4,6,3,5,6,3,5,2,1,0".to_string()))
    }
    #[test]
    fn part2_example1() {
        let text = "";
        assert_eq!(DAY.part2(text), Some("".to_string()))
    }
}
