use crate::days;

pub struct Day;

impl Day {}

impl days::Day for Day {
    fn day(&self) -> u32 {
        3
    }

    fn part1(&self, input: &str) -> Option<i64> {
        None
    }
    fn part2(&self, input: &str) -> Option<i64> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;

    const DAY: crate::day2::Day = crate::day2::Day;
    #[test]
    fn part1_example1() {
        let text = "";
        assert_eq!(DAY.part1(text), Some(2))
    }
    #[test]
    fn part2_example1() {
        let text = "";
        assert_eq!(DAY.part2(text), Some(4))
    }
}
