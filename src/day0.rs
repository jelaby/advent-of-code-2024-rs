use crate::days;

pub struct Day;

impl Day {
    fn short_fuse(&self, lines: &str) -> i64 {
        let lines = lines.split_terminator("\n");
        lines.count() as i64
    }
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        0
    }

    fn part1(&self, lines: &str) -> Option<i64> {
        Some(self.short_fuse(lines))
    }
}

#[cfg(test)]
mod tests {

    use crate::days::Day;

    const DAY: crate::day0::Day = crate::day0::Day;
    #[test]
    fn part1_returns_number_of_lines() {
        assert_eq!(
            DAY.part1(
                "some line
some other line"),
            Some(2)
        )
    }
}
