use crate::days;

pub struct Day;

impl Day {}

fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .split_terminator('\n')
        .map(|line| {
            let mut parts = line.split(": ");
            let total = parts.next().unwrap().parse().unwrap();
            let items = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|i| i.parse().unwrap())
                .collect();
            return (total, items);
        })
        .collect()
}

fn is_possible(total: i64, items: &Vec<i64>) -> bool {
    is_possible_n(total, &items[1..items.len()], items[0])
}

fn add(a: i64, b: i64) -> i64 {
    a + b
}
fn mul(a: i64, b: i64) -> i64 {
    a * b
}

const OP: [fn(i64, i64) -> i64; 2] = [add, mul];

fn is_possible_n(target: i64, items: &[i64], total: i64) -> bool {
    if items.is_empty() {
        return target == total;
    }
    for op in OP {
        if is_possible_n(target, &items[1..items.len()], op(total, items[0])) {
            return true;
        }
    }
    return false;
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        7
    }

    fn part1(&self, input: &str) -> Option<i64> {
        let equations = parse(input);

        Some(
            equations
                .iter()
                .filter(|(total, values)| is_possible(*total, values))
                .map(|(total, _)| total)
                .sum(),
        )
    }
    fn part2(&self, input: &str) -> Option<i64> {
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
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20\
";
        assert_eq!(DAY.part1(text), Some(3749))
    }
    #[test]
    fn part2_example1() {
        let text = "";
        assert_eq!(DAY.part2(text), Some(4))
    }
}
