use crate::days;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day;

impl Day {}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Gate<'a> {
    Fixed(bool),
    Operation(&'a str, &'a str, Op),
}
use Gate::*;

impl <'a> Gate<'a> {
    fn eval(&self, gates: &HashMap<&'a str, Gate<'a>>, evaluating: &mut HashSet<&'a str>) -> Option<bool> {
        match self {
            Fixed(val) => Some(*val),
            Operation(l, r, op) => {
                if evaluating.contains(l) || evaluating.contains(r) {
                    None
                } else {
                    evaluating.insert(*l);
                    let lv = gates.get(l).and_then(|g| g.eval(gates, evaluating))?;
                    evaluating.remove(*l);
                    evaluating.insert(*r);
                    let rv = gates.get(r).and_then(|g| g.eval(gates, evaluating))?;
                    evaluating.remove(*r);
                    Some(op.eval(lv, rv))
                }
            }
        }
    }
}

fn parse<'a>(input: &'a str) -> HashMap<&'a str, Gate<'a>> {
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

fn input_bits(gates: &HashMap<&str, Gate>) -> i64 {
    gates
        .keys()
        .filter(|name| &name[0..1] == "x")
        .map(|name| name[1..].parse::<i64>().unwrap())
        .max()
        .unwrap()
}

fn mutate<'a, 'b>(
    gates: &'b HashMap<&'a str, Gate<'a>>,
    bits: i64,
    x: i64,
    y: i64,
) -> HashMap<&'a str, Gate<'a>> {
    let mut result = HashMap::new();

    for (&name, &gate) in gates {
        let bit = name[1..].parse::<i64>();
        if &name[0..1] == "x" {
            result.insert(name, Fixed(x & (1i64 << bit.unwrap()) != 0));
        } else if &name[0..1] == "y" {
            result.insert(name, Fixed(y & (1i64 << bit.unwrap()) != 0));
        } else {
            result.insert(name, gate.clone());
        }
    }

    result
}

fn source_gates<'a>(gates: &HashMap<&'a str, Gate<'a>>, gate: &'a str) -> HashSet<&'a str> {
    let mut result = HashSet::new();

    let mut queue = VecDeque::new();

    queue.push_back(gate);

    while let Some(gate) = queue.pop_front() {
        if &gate[0..1] != "x" && &gate[0..1] != "y" {
            result.insert(gate);
        }

        match gates.get(gate) {
            Some(Fixed(val)) => {}
            Some(Operation(a, b, _)) => {
                queue.push_back(*a);
                queue.push_back(*b);
            }
            None => {
                panic!("Could not find gate {gate}");
            }
        }
    }

    result
}

fn union<'a>(l: &HashSet<&'a str>, r: &HashSet<&'a str>) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    l.iter().for_each(|i| {
        result.insert(*i);
    });
    r.iter().for_each(|i| {
        result.insert(*i);
    });
    result
}

fn minus<'a, 'b>(l: &HashSet<&'a str>, r: &HashSet<&'b str>) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    l.iter().filter(|&&i| !r.contains(i)).for_each(|i| {
        result.insert(*i);
    });
    result
}

fn find_candidate_output_bits<F>(
    gates: &HashMap<&str, Gate>,
    bits: i64,
    a: i64,
    operation: &F,
) -> Option<(u32, u32, u32)>
where
    F: Fn(i64, i64) -> i64,
{
    (0..=45).find_map(|i| {
        let b = 0 ^ (1 << i);

        match eval(&mutate(&gates, bits, a, b)) {
            None => None,
            Some(result) => {
                let expected = operation(a, b);

                let incorrect_bits_this_time = expected ^ result;

                if incorrect_bits_this_time.count_ones() == 2 {
                    let lsb = incorrect_bits_this_time.trailing_zeros();
                    let msb = (incorrect_bits_this_time ^ (1 << lsb)).trailing_zeros();

                    Some((i, lsb, msb))
                } else {
                    None
                }
            }
        }
    })
}

fn count_errors<F>(
    gates: &HashMap<&str, Gate>,
    bits: i64,
    a: i64,
    operation: &F,
) -> usize
where
    F: Fn(i64, i64) -> i64,
{
    (0..=45).filter(|i| {
        let b = 0 ^ (1 << i);

        match eval(&mutate(&gates, bits, a, b)) {
            None => true,
            Some(result) => {
                let expected = operation(a, b);
                expected != result
            }
        }
    }).count()
}

fn find_candidate_swap_gates<'a>(
    gates: &HashMap<&'a str, Gate<'a>>,
    lsb: u32,
    msb: u32,
) -> HashSet<&'a str> {
    let lsb_gate = gates
        .get_key_value(format!("z{lsb:02}").as_str())
        .unwrap()
        .0;
    let msb_gate = gates
        .get_key_value(format!("z{msb:02}").as_str())
        .unwrap()
        .0;

    let lsb_gates = source_gates(&gates, &lsb_gate);
    let msb_gates = source_gates(&gates, &msb_gate);

    println!("{lsb_gates:?}");
    println!("{msb_gates:?}");

    let mut common_gates = union(&lsb_gates, &msb_gates);
    for b in (0..lsb).rev() {
        for g in source_gates(
            &gates,
            gates.get_key_value(format!("z{b:02}").as_str()).unwrap().0,
        ) {
            common_gates.remove(g);
        }
    }

    common_gates
}

fn swap_gates<'a>(
    gates: &HashMap<&'a str, Gate<'a>>,
    a: &'a str,
    b: &'a str,
) -> HashMap<&'a str, Gate<'a>> {
    let mut result = HashMap::new();

    for (&name, gate) in gates {
        let name = {
            if name == a {
                b
            } else if name == b {
                a
            } else {
                name
            }
        };
        result.insert(name, gate.clone());
    }

    result
}

fn check_gates<'a, F>(
    gates: &HashMap<&'a str, Gate<'a>>,
    bits: i64,
    a: i64,
    input_bit: u32,
    operation: &F,
    previous_errors: usize
) -> bool
where
    F: Fn(i64, i64) -> i64,
{
    count_errors(gates, bits, a, operation) < previous_errors
}

fn do_part2<F>(input: &str, swap_count: i64, operation: F) -> Option<String>
where
    F: Fn(i64, i64) -> i64,
{
    let gates = parse(input);
    let bits = input_bits(&gates);

    let ones = {
        let mut ones = 0;
        for b in 0..45 {
            ones += 1 << b;
        }
        ones
    };
    // get 0000 for a+b and 1111 for a&b
    let a = {
        let mut a = ones;
        for _ in (0..45) {
            a = operation(a, a) & ones
        }
        a
    };

    let mut result = Vec::new();
    let mut gates = gates;

    fn contains<'a>(s: &Vec<(&'a str, &'a str)>, a: &'a str, b: &'a str) -> bool {
        s.iter().any(|&(x,y)| {
            (x == a && y == b)
            || (x == b && y == a)
            || x == a || x == b || y == a || y == b
        })
    }

    let mut finished = false;
    while !finished {
        finished = true;

        match find_candidate_output_bits(&gates, bits, a, &operation) {
            None => {}
            Some((input_bit, lsb, msb)) => {
                finished = false;

                let error_count = count_errors(&gates, bits, a, &operation);

                let swap_candidates: Vec<&str> = find_candidate_swap_gates(&gates, lsb, msb)
                    .into_iter()
                    .collect();

                for x in 0..swap_candidates.iter().len() {
                    for y in x + 1..swap_candidates.len() {
                        let x = swap_candidates[x];
                        let y = swap_candidates[y];
                        if !contains(&result, x,y) && check_gates(&swap_gates(&gates, x, y), bits, a, input_bit, &operation, error_count) {
                            result.push((x, y));
                            gates = swap_gates(&gates, x, y);
                            println!("Swapping {x} {y} -> {result:?}");
                        }
                    }
                }
            }
        }
    }
    Some(result.iter().flat_map(|(x, y)| [x, y]).sorted().join(","))
}

fn eval(gates: &HashMap<&str, Gate>) -> Option<i64> {
    gates.iter()
        .filter(|(name, _)| name[0..1] == *"z")
        .map(|(name, gate)| (name, gate.eval(&gates, &mut HashSet::new())))
        .map(|(name, value)| (name[1..].parse::<i64>().unwrap(), value))
        .map(|(name, value)| match value {
            Some(true) => Some(1 << name),
            Some(false) => Some(0),
            None => None,
        })
        .fold(Some(0), |acc,i| match acc {
            None => None,
            Some(a) => i.map(|b| a + b)
        })
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        24
    }

    fn part1(&self, input: &str) -> Option<String> {
        let gates = parse(input);

        eval(&gates).map(|r| r.to_string())
    }
    fn part2(&self, input: &str) -> Option<String> {
        do_part2(input, 4, |a, b| a + b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let text = "\
x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";
        assert_eq!(
            do_part2(text, 2, |a, b| a & b),
            Some("z00,z01,z02,z05".to_string())
        )
    }
}
