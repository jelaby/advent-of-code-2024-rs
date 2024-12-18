use crate::days;
use itertools::Itertools;
use num::pow;
use regex::Regex;

pub struct Day;

impl Day {}

#[derive(Debug, Eq, PartialEq)]
struct Machine {
    a: i64,
    b: i64,
    c: i64,
    i: i64,
    output: Vec<i64>,
}

impl Machine {
    fn combo(&self, operand: i64) -> i64 {
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

impl Machine {
    fn new(a: i64, b: i64, c: i64) -> Machine {
        Machine {
            a,
            b,
            c,
            i: 0,
            output: vec![],
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
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
    fn execute(&self, machine: &mut Machine, operand: i64) {
        match self {
            OpCode::ADV => machine.a = machine.a / pow(2, machine.combo(operand) as usize),
            OpCode::BXL => machine.b = machine.b ^ operand,
            OpCode::BST => machine.b = machine.combo(operand) & 0x7,
            OpCode::JNZ => {
                if machine.a != 0 {
                    machine.i = operand - 2
                }
            }
            OpCode::BXC => machine.b = machine.b ^ machine.c,
            OpCode::OUT => machine.output.push(machine.combo(operand) & 0x7),
            OpCode::BDV => machine.b = machine.a / pow(2, machine.combo(operand) as usize),
            OpCode::CDV => machine.c = machine.a / pow(2, machine.combo(operand) as usize),
        }
    }
}

fn opcode_for(n: i64) -> OpCode {
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

fn parse_register(input: &str) -> i64 {
    Regex::new(r"Register \w: (\d+)")
        .ok()
        .and_then(|r| r.captures(input))
        .and_then(|c| c.get(1))
        .and_then(|v| v.as_str().parse().ok())
        .unwrap()
}

fn parse_program(input: &str) -> Vec<i64> {
    Regex::new(r"Program: (.*)")
        .ok()
        .and_then(|r| r.captures(input))
        .and_then(|c| c.get(1))
        .map(|v| {
            v.as_str()
                .split(",")
                .map(|o| o.parse::<i64>().expect(&format!("Failed to parse {o}")))
                .collect()
        })
        .unwrap()
}

fn parse(input: &str) -> (i64, i64, i64, Vec<i64>) {
    let mut lines = input.lines();

    let a = parse_register(lines.next().unwrap());
    let b = parse_register(lines.next().unwrap());
    let c = parse_register(lines.next().unwrap());

    let _ = lines.next();

    let program = parse_program(lines.next().unwrap());

    (a, b, c, program)
}

fn run(machine: &mut Machine, input: &Vec<i64>) {
    while machine.i < input.len() as i64 {
        opcode_for(input[machine.i as usize]).execute(machine, input[(machine.i + 1) as usize]);

        machine.i += 2;
    }
}

fn solve_part_2(input: &Vec<i64>, expected: &Vec<i64>) -> Option<i64> {

    let mut queue = vec![];
    for i in (0..=0x7).rev() {
        queue.push(i)
    }

    while !queue.is_empty() {

        let a = queue.pop().unwrap();

        let mut machine = Machine::new(a, 0, 0);

        run(&mut machine, input);

        if machine.output.len() > 0 && machine.output.len() <= expected.len()
            && machine.output[..] == expected[expected.len() - machine.output.len()..]
        {
            if machine.output.len() == expected.len() {
                return Some(a);
            } else {
                for i in (0..=0x7).rev() {
                    if a > 0 || i > 0 {
                        queue.push(a << 3 | i);
                    }
                }
            }
        }
    }

    return None;
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        17
    }

    fn part1(&self, input: &str) -> Option<String> {
        let (a, b, c, program) = parse(input);

        let mut machine = Machine {
            a,
            b,
            c,
            i: 0,
            output: vec![],
        };

        run(&mut machine, &program);

        Some(machine.output.iter().map(|n| n.to_string()).join(","))
    }
    fn part2(&self, input: &str) -> Option<String> {
        let (_, _, _, program) = parse(input);

        solve_part_2(&program, &program).map(|r| r.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::Day;

    const DAY: super::Day = super::Day;
    #[test]
    fn part1_example0() {
        let mut machine = Machine::new(0, 2024, 43690);
        opcode_for(4).execute(&mut machine, 0);
        assert_eq!(machine.b, 44354)
    }
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
        let text = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!(DAY.part2(text), Some("117440".to_string()))
    }
}
