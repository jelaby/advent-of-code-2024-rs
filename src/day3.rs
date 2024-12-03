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
        None
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
        let text = "";
        assert_eq!(DAY.part2(text), Some(4))
    }
}
