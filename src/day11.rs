use std::cmp::max;
use std::collections::HashMap;
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

fn count_splits(stone: i64, iterations: usize) -> usize {
    count_splits_cached(stone, iterations, &mut HashMap::new())
}
fn count_splits_cached(stone: i64, iterations: usize, cache: &mut HashMap<(i64,usize),usize>) -> usize {
    if iterations == 0 {
        1
    } else {
        match cache.get(&(stone, iterations)) {
            Some(count) => *count,
            None => {
                let count = {
                    if stone == 0 {
                        count_splits_cached(1, iterations - 1, cache)
                    } else {
                        let length = get_length(stone);
                        if length % 2 == 0 {
                            let divisor = num::pow(10, length / 2);
                            count_splits_cached(stone / divisor, iterations - 1, cache)
                                + count_splits_cached(stone % divisor, iterations - 1, cache)
                        } else {
                            count_splits_cached(stone * 2024, iterations - 1, cache)
                        }
                    }
                };
                cache.insert((stone, iterations), count);
                count
            }
        }
    }
}

fn iterate_n(stones: &Vec<i64>, count: usize) -> usize {
    stones.iter()
        .map(|&stone| count_splits(stone, count))
        .sum()
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        11
    }

    fn part1(&self, input: &str) -> Option<i64> {
        let stones = parse(input);
        Some(iterate_n(&stones, 25) as i64)
    }
    fn part2(&self, input: &str) -> Option<i64> {
        let stones = parse(input);
        Some(iterate_n(&stones, 75) as i64)
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
    fn part1_example1_breakdown2() {
        let stones = super::parse("125 17");
        assert_eq!(super::iterate_n(&stones, 0), 2);
        assert_eq!(super::iterate_n(&stones, 1), 3);
        assert_eq!(super::iterate_n(&stones, 2), 4);
        assert_eq!(super::iterate_n(&stones, 3), 5);
        assert_eq!(super::iterate_n(&stones, 4), 9);
        assert_eq!(super::iterate_n(&stones, 5), 13);
        assert_eq!(super::iterate_n(&stones, 6), 22);
    }
    #[test]
    fn part2_example1() {
        let text = "125 17";
        assert_eq!(DAY.part2(text), Some(65601038650482))
    }
}
