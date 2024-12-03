use crate::days;
use regex::Regex;

pub struct Day;

impl Day {}

impl days::Day for Day {
    fn day(&self) -> u32 {
        3
    }

    fn part1(&self, input: &str) -> Option<i64> {

        let mul_pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        Some(mul_pattern.captures_iter(input).map(|c| c.extract())
            .map(|(_, [left, right])| left.parse::<i64>().unwrap() * right.parse::<i64>().unwrap())
            .sum())
    }
    fn part2(&self, input: &str) -> Option<i64> {
        let mul_pattern = Regex::new(r"(?<op2>mul)\((?<p1>\d+),(?<p2>\d+)\)|(?<op0>do|don't)\(\)").unwrap();

        let mut sum = 0i64;
        let mut enabled = true;

        for capture in mul_pattern.captures_iter(input) {
            if let Some(op) = capture.name("op0") {
                if op.as_str() == "do" {
                    enabled = true;
                } else if op.as_str() == "don't" {
                    enabled = false;
                }
            } else if let Some(op) = capture.name("op2") {
                if enabled {
                    if op.as_str() == "mul" {
                        if let Some(l) = capture.name("p1").and_then(|m| m.as_str().parse::<i64>().ok()) {
                            if let Some(r) = capture.name("p2").and_then(|m| m.as_str().parse::<i64>().ok()) {
                                sum += l * r
                            }
                        }
                    }
                }
            }
        }
        Some(sum)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;

    const DAY: crate::day3::Day = crate::day3::Day;
    #[test]
    fn part1_example1() {
        let text = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(DAY.part1(text), Some(161))
    }
    #[test]
    fn part2_example1() {
        let text = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(DAY.part2(text), Some(48))
    }
}
