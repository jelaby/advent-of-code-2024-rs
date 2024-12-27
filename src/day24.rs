use crate::days;
use std::collections::{HashMap, HashSet, VecDeque};
use rand::random;

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

        result.insert(gate);

        match gates.get(gate) {
            Some(Fixed(val)) => {},
            Some(Operation(a, b, _)) => {
                    queue.push_back(*a);
                    queue.push_back(*b);
            },
            None => { panic!("Could not find gate {gate}"); }
        }

    }

    result
}

fn union<'a>(l: &HashSet<&'a str>, r: &HashSet<&'a str>) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    l.iter().for_each(|i| { result.insert(*i); });
    r.iter().for_each(|i| { result.insert(*i); });
    result
}

fn minus<'a,'b>(l: &HashSet<&'a str>, r: &HashSet<&'b str>) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    l.iter().filter(|&&i| !r.contains(i)).for_each(|i| { result.insert(*i); });
    result
}

fn do_part2<F>(input: &str, swap_count: i64, operation: F) -> Option<String>
where
    F: Fn(i64, i64) -> i64,
{
    let mut rng = rand::thread_rng();
    let gates = parse(input);
    let bits = input_bits(&gates);

    let ones = (0..bits).map(|b| 1 << b).sum::<i64>();

    let mut incorrect_bits = 0;

    for i in 0..bits {
        let a = random::<i64>() & ones;
        let b = random::<i64>() & ones;

        let result = eval(&mutate(&gates, bits, a, b));

        let expected = operation(a, b);

        //println!("{a} + {b} -> ({expected}) {result}");
        //println!("    {a:045b}\n  + {b:045b}\n -> {expected:045b}\n    {result:045b}");

        println!("    {:045b}", expected ^ result);

        incorrect_bits |= expected ^ result;
    }
    for i in 0..bits {
        let a = ones;
        let b = ones ^ (1 << i);

        let result = eval(&mutate(&gates, bits, a, b));

        let expected = operation(a, b);

        //println!("{a} + {b} -> ({expected}) {result}");
        //println!("    {a:045b}\n  + {b:045b}\n -> {expected:045b}\n    {result:045b}");

        println!("    {:045b}", expected ^ result);

        incorrect_bits |= expected ^ result;
    }
    for i in 0..bits {
        let a = 0;
        let b = 0 ^ (1 << i);

        let result = eval(&mutate(&gates, bits, a, b));

        let expected = operation(a, b);

        //println!("{a} + {b} -> ({expected}) {result}");
        //println!("    {a:045b}\n  + {b:045b}\n -> {expected:045b}\n    {result:045b}");

        println!("    {:045b}", expected ^ result);

        let incorrect_bits_this_time = expected ^ result;

        incorrect_bits |= expected ^ result;

        if incorrect_bits_this_time.count_ones() == 2 {

            let lsb = incorrect_bits_this_time.trailing_zeros();
            let msb = (incorrect_bits_this_time ^ (1 << lsb)).trailing_zeros();

            let lsb_gate = format!("z{lsb:02}");
            let msb_gate = format!("z{msb:02}");

            let lsb_gates = source_gates(&gates, &lsb_gate);
            let msb_gates = source_gates(&gates, &msb_gate);

            println!("{lsb_gates:?}");
            println!("{msb_gates:?}");

            let mut common_gates = union(&lsb_gates, &msb_gates);
            for b in (0..lsb).rev() {
                common_gates = minus(&common_gates, &source_gates(&gates, &format!("z{b:02}")));
            }

            println!("{common_gates:?}");

        }
    }
    println!("X   {incorrect_bits:045b}");

    let mut potential_swaps = HashSet::new();

    for i in 0..(bits+1) {
        if incorrect_bits & (1<<i) != 0 {
            let output = format!("z{i:02}");

            // swap for string from gates
            let (&output, &value) = gates.get_key_value(output.as_str()).unwrap();

            potential_swaps.insert(output);

            let mut queue = VecDeque::from(vec![value]);
            while let Some(value) = queue.pop_front() {
                match value {
                    Fixed(_) => (),
                    Operation(a, b, op) => {
                        if &a[0..1] != "x" && &a[0..1] != "y" {
                            match op {
                                And => { potential_swaps.insert(a); },
                                _ => (),
                            }
                        }
                        if &b[0..1] != "x" && &b[0..1] != "y" {
                            match op {
                                And => { potential_swaps.insert(b); },
                                _ => (),
                            }
                        }
                        queue.push_back(*gates.get(a).unwrap());
                        queue.push_back(*gates.get(b).unwrap());
                    }
                }
            }
        }
    }
    println!("Swaps: {potential_swaps:?} {}", potential_swaps.len());

    None
}

fn eval(gates: &HashMap<&str, Gate>) -> i64 {
    gates.iter()
        .filter(|(name, _)| name[0..1] == *"z")
        .map(|(name, gate)| (name, gate.eval(&gates)))
        .map(|(name, value)| (name[1..].parse::<i64>().unwrap(), value))
        .map(|(name, value)| match value {
            true => 1,
            false => 0,
        } << name)
        .sum::<i64>()
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        24
    }

    fn part1(&self, input: &str) -> Option<String> {
        let gates = parse(input);

        Some(eval(&gates)).map(|r| r.to_string())
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
