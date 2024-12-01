use crate::days;

pub struct Day;

impl Day {
    fn short_fuse(&self, lines: String) -> i64 {
        let lines = lines.split("\n");
        lines.count() as i64
    }
}

impl days::Day for Day {
    fn day(&self) -> i32 {
        0
    }

    fn part1(&self, lines: String) -> Option<i64> {
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
some other line"
                    .to_string()
            ),
            Some(2)
        )
    }
}
