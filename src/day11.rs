use std::cmp::max;
use crate::days;

pub struct Day;

impl Day {}

fn parse(input: &str) -> Vec<i64> {
    input
        .split_terminator('\n')
        .flat_map(|line| line.split_whitespace())
        .map(|word| word.parse::<i64>().unwrap())
        .collect()
}

fn get_length(i: i64) -> usize {
    let mut result = 0;
    let mut i = i;
    while i > 0 {
        i = i / 10;
        result += 1;
    }
    return max(result, 1);
}

fn iterate(stones: &Vec<i64>) -> Vec<i64> {
    stones
        .iter()
        .flat_map(|&stone| {
            if stone == 0 {
                return vec![1];
            }

            let length = get_length(stone);
            if length % 2 == 0 {
                let divisor = num::pow(10, length / 2);
                return vec![stone / divisor, stone % divisor];
            }

            return vec![stone * 2024];
        })
        .fold(Vec::with_capacity(stones.len() * 2), |mut acc, stone| {
            acc.push(stone);
            acc
        })
}

fn iterate_n(stones: Vec<i64>, count: usize) -> Vec<i64> {
    let mut stones = stones.clone();
    for _ in 0..count {
        stones = iterate(&stones);
    }
    return stones;
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        11
    }

    fn part1(&self, input: &str) -> Option<i64> {
        let stones = parse(input);
        Some(iterate_n(stones, 25).len() as i64)
    }
    fn part2(&self, input: &str) -> Option<i64> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;

    const DAY: super::Day = super::Day;
    #[test]
    fn part1_example1() {
        let text = "125 17";
        assert_eq!(DAY.part1(text), Some(55312))
    }
    #[test]
    fn part1_example1_breakdown() {
        let stones = super::parse("125 17");
        let stones = super::iterate(&stones);
        assert_eq!(stones, vec![253000, 1, 7]);
        let stones = super::iterate(&stones);
        assert_eq!(stones, vec![253, 0, 2024, 14168]);
        let stones = super::iterate(&stones);
        assert_eq!(stones, vec![512072, 1, 20, 24, 28676032]);
        let stones = super::iterate(&stones);
        assert_eq!(stones, vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]);
        let stones = super::iterate(&stones);
        assert_eq!(
            stones,
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
        );
        let stones = super::iterate(&stones);
        assert_eq!(
            stones,
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ]
        );
    }
    #[test]
    fn part2_example1() {
        let text = "";
        assert_eq!(DAY.part2(text), Some(4))
    }
}
