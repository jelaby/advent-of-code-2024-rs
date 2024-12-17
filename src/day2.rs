use crate::days;

pub struct Day;

impl Day {}

fn check_report(report: &Vec<i64>) -> bool {
    let mut direction = Direction::Unknown;
    let mut i = report.iter();
    let mut last = i.next().unwrap();
    loop {
        let n = i.next();

        match n {
            None => break true,
            Some(n) => {
                if n == last {
                    break false;
                }

                if (n - last).abs() > 3 {
                    break false;
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
                            break false;
                        }
                    }
                    Direction::Down => {
                        if n > last {
                            break false;
                        }
                    }
                }

                last = n;
            }
        }
    }
}

enum Direction {
    Unknown,
    Up,
    Down,
}
impl days::Day for Day {
    fn day(&self) -> u32 {
        2
    }

    fn part1(&self, input: &str) -> Option<String> {
        let reports = input
            .split_terminator("\n")
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect::<Vec<Vec<i64>>>();

        Some(reports.iter().filter(|r| check_report(r)).count() as i64).map(|r| r.to_string())
    }
    fn part2(&self, input: &str) -> Option<String> {
        let reports = input
            .split_terminator("\n")
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect::<Vec<Vec<i64>>>();

        Some(
            reports
                .iter()
                .filter(|report| {
                    if check_report(&report) {
                        return true;
                    }

                    for i in 0..reports.len() {
                        if check_report(
                            &report
                                .iter()
                                .enumerate()
                                .filter(|(j, _)| i != *j)
                                .map(|(_, &v)| v)
                                .collect(),
                        ) {
                            return true;
                        }
                    }

                    return false;
                })
                .count() as i64,
        ).map(|r| r.to_string())
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
        assert_eq!(DAY.part1(text), Some("2".to_string()))
    }
    #[test]
    fn part2_example1() {
        let text = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(DAY.part2(text), Some("4".to_string()))
    }
    #[test]
    fn part2_remove_first() {
        let text = "1 8 9 10 11";
        assert_eq!(DAY.part2(text), Some("1".to_string()))
    }
}
