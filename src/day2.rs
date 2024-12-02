use crate::days;

pub struct Day;

enum Direction {
    Unknown,
    Up,
    Down,
}
impl days::Day for Day {
    fn day(&self) -> i32 {
        2
    }

    fn part1(&self, input: &str) -> Option<i64> {
        let reports = input
            .split_terminator("\n")
            .map(|line| line.split_whitespace().map(|n| n.parse().unwrap()).collect())
            .collect::<Vec<Vec<i64>>>();

        Some(
            reports
                .iter()
                .filter(|report| {
                    let mut direction = Direction::Unknown;
                    let mut i = report.iter();
                    let mut last = i.next().unwrap();
                    'result: loop {
                        let n = i.next();

                        match n {
                            None => break true,
                            Some(n) => {
                                if n == last {
                                    break 'result false;
                                }

                                if (n - last).abs() > 3 {
                                    break 'result false;
                                }

                                match direction {
                                    Direction::Unknown => {
                                        if n > last {
                                            direction = Direction::Up
                                        } else {
                                            direction = Direction::Down
                                        }
                                    }
                                    Direction::Up => {
                                        if n < last {
                                            break 'result false;
                                        }
                                    }
                                    Direction::Down => {
                                        if n > last {
                                            break 'result false;
                                        }
                                    }
                                }

                                last = n;
                            }
                        }
                    }
                })
                .count() as i64,
        )
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
        let text = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(DAY.part1(text), Some(2))
    }
    #[test]
    fn part2_example1() {
        let text = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(DAY.part2(text), Some(31))
    }
}
