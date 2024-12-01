pub trait Day {

    fn day(&self) -> i32;
    fn part1(&self, _lines: &str) -> Option<i64> {
        None
    }
    fn part2(&self, _lines: &str) -> Option<i64> {
        None
    }
}