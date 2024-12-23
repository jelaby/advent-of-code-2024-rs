use crate::days;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

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

fn prices(initial_secret: i64) -> Vec<i64> {
    let mut n = initial_secret;
    let mut prices = vec![n % 10];
    for _ in 0..2000 {
        n = next_secret(n);
        let price = n % 10;
        prices.push(price);
    }
    prices
}
fn best_sequence(prices_list: &Vec<Vec<i64>>) -> Vec<i64> {
    let histogram = &mut HashMap::new();

    for prices in prices_list {
        let mut already_added_sequences = HashSet::new();

        for window in prices.windows(5) {
            let diffs: Vec<_> = window.windows(2).map(|pair| pair[1] - pair[0]).collect();

            if !already_added_sequences.contains(&diffs) {
                already_added_sequences.insert(diffs.clone());
                histogram
                    .entry(diffs)
                    .and_modify(|count| *count += window[4])
                    .or_insert(window[4]);
            }
        }
    }

    let (result, value) = histogram
        .iter()
        .sorted_by(|(_, a), (_, b)| b.cmp(a))
        .next()
        .unwrap();

    result.clone()
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        22
    }

    fn part1(&self, input: &str) -> Option<String> {
        Some(
            input
                .lines()
                .map(|l| l.parse::<i64>().unwrap())
                .map(|n| {
                    let mut n = n;
                    for i in 0..2000 {
                        n = next_secret(n);
                    }
                    n
                })
                .sum::<i64>(),
        )
        .map(|r| r.to_string())
    }
    fn part2(&self, input: &str) -> Option<String> {
        let prices_list = input.lines().map(|l| l.parse::<i64>().unwrap())
            .map(|initial_secret| prices(initial_secret))
            .collect();


        let best_sequence = best_sequence(&prices_list);

        let mut result = 0;

        for prices in prices_list {
            for window in prices.windows(5) {
                let diffs: Vec<_> = window.windows(2).map(|pair| pair[1] - pair[0]).collect();

                if diffs == best_sequence {
                    result += window[4];
                    break;
                }
            }
        }

        Some(result).map(|r| r.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;
    use super::*;
    const DAY: super::Day = super::Day;
    #[test]
    fn prices_output() {
        assert_eq!(prices(123)[0..10], vec![3,0,6,5,4,4,6,4,4,2]);
    }
    #[test]
    fn best_sequence_example() {
        assert_eq!(best_sequence(&vec![prices(123)[0..10].to_vec()]), vec![-1,-1,0,2]);
    }
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
        let text = "\
1
2
3
2024";
        assert_eq!(DAY.part2(text), Some(23.to_string()))
    }
}
