use crate::days;
use std::collections::HashMap;

pub struct Day;

impl Day {}

enum Op {
    And,
    Or,
    Xor,
}
use Op::*;

impl Op {
    fn eval(&self, l: bool, r: bool) -> bool {
        match self {
            And => l && r,
            Or => l || r,
            Xor => l ^ r,
        }
    }
}

enum Gate<'a> {
    Fixed(bool),
    Operation(&'a str, &'a str, Op),
}
use Gate::*;

impl Gate<'_> {
    fn eval(&self, gates: &HashMap<&str, Gate>) -> bool {
        match self {
            Fixed(val) => *val,
            Operation(l, r, op) => op.eval(
                gates.get(l).map(|g| g.eval(gates)).unwrap(),
                gates.get(r).map(|g| g.eval(gates)).unwrap(),
            ),
        }
    }
}

fn parse<'a>(input: &'a str) -> HashMap<&'a str, Gate> {
    enum Mode {
        INPUTS,
        CONNECTIONS,
    }
    use Gate::*;
    use Mode::*;
    use Op::*;
    let mut mode = INPUTS;

    let mut result = HashMap::new();

    for line in input.lines() {
        match mode {
            INPUTS => {
                if line == "" {
                    mode = CONNECTIONS
                } else {
                    let mut parts = line.split(": ");
                    let name = parts.next().unwrap();
                    let value = match parts.next() {
                        Some("1") => true,
                        Some("0") => false,
                        _ => panic!("Could not parse input line {line}"),
                    };
                    result.insert(name, Fixed(value));
                }
            }
            CONNECTIONS => {
                let mut parts = line.split(" ");
                let l = parts.next().unwrap();
                let op = match parts.next() {
                    Some("AND") => And,
                    Some("OR") => Or,
                    Some("XOR") => Xor,
                    _ => panic!("Could not parse gate line {line}"),
                };
                let r = parts.next().unwrap();
                let _ = parts.next();
                let name = parts.next().unwrap();

                result.insert(name, Operation(l, r, op));
            }
        }
    }

    result
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        24
    }

    fn part1(&self, input: &str) -> Option<String> {
        let gates = parse(input);

        Some(gates.iter()
            .filter(|(name, _)| name[0..1] == *"z")
            .map(|(name, gate)| (name, gate.eval(&gates)))
            .map(|(name, value)| (name[1..].parse::<i64>().unwrap(), value))
            .map(|(name, value)| match value {
                true => 1,
                false => 0,
            } << name)
            .sum::<i64>())
            .map(|r| r.to_string())
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
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
        assert_eq!(DAY.part1(text), Some(4.to_string()))
    }
    #[test]
    fn part1_example2() {
        let text = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
        assert_eq!(DAY.part1(text), Some(2024.to_string()))
    }
    #[test]
    fn part2_example1() {
        let text = "";
        assert_eq!(DAY.part2(text), Some("4".to_string()))
    }
}
