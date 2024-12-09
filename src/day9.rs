use crate::days;
use itertools::Itertools;
use std::cmp::max;

pub struct Day;

impl Day {}

fn value(c: char) -> i32 {
    c as i32 - '0' as i32
}
fn parse(input: &str) -> Vec<i32> {
    input
        .chars()
        .chunks(2)
        .into_iter()
        .enumerate()
        .flat_map(|(i, mut cc)| {
            let mut result = vec![];
            for _ in 0..value(cc.next().unwrap()) {
                result.push(i as i32);
            }
            if let Some(c) = cc.next().map(|c| value(c)) {
                for _ in 0..c {
                    result.push(-1);
                }
            }
            result
        })
        .collect()
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        9
    }

    fn part1(&self, input: &str) -> Option<i64> {
        let mut map = parse(input);

        let mut i = map.len() - 1;
        let mut target = 0;
        while i > target {
            let id = map[i];
            if id >= 0 {
                while map[target] >= 0 {
                    target += 1;
                }
                if target < i {
                    map[target] = id;
                    map[i] = -1;
                }
            }
            i -= 1;
        }

        Some(map.iter()
            .enumerate()
            .map(|(i, id)| i as i64 * (max(*id as i64, 0)))
            .sum())
        } fn part2(&self, input: &str) -> Option<i64> { None
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;

    const DAY: super::Day = super::Day;
    #[test]
    fn part1_example1() {
        let text = "2333133121414131402";
        assert_eq!(DAY.part1(text), Some(1928))
    }
    #[test]
    fn part2_example1() {
        let text = "";
        assert_eq!(DAY.part2(text), Some(4))
    }
}
