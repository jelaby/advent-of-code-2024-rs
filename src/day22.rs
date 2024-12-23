use crate::days;

pub struct Day;

impl Day {}

fn mix(a: i64, b: i64) -> i64 {
    a ^ b
}

fn prune(n: i64) -> i64 {
    n.rem_euclid(16777216)
}

fn next_secret(n: i64) -> i64 {
    let n = prune(mix(n, n * 64));
    let n = prune(mix(n, n / 32));
    let n = prune(mix(n, n * 2048));
    n
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        22
    }

    fn part1(&self, input: &str) -> Option<String> {

        Some(input.lines()
            .map(|l| l.parse::<i64>().unwrap())
            .map(|n| {
                let mut n = n;
                for i in 0..2000 {
                    n = next_secret(n);
                }
                n
            })
            .sum::<i64>())
            .map(|r| r.to_string())
    }
    fn part2(&self, input: &str) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;
    use super::*;
    const DAY: super::Day = super::Day;
    #[test]
    fn part1_example1() {
        let text = "\
1
10
100
2024";
        assert_eq!(DAY.part1(text), Some(37327623.to_string()))
    }
    #[test]
    fn part2_example1() {
        let text = "";
        assert_eq!(DAY.part2(text), Some("4".to_string()))
    }
}
