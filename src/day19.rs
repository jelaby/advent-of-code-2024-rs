use crate::days;
use itertools::Itertools;
use regex::Regex;
use std::cmp::min;
use std::collections::{HashMap, HashSet};

pub struct Day;

impl Day {}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines();

    let towels: Vec<&str> = lines.next().unwrap().split(", ").collect();

    let _ = lines.next();

    let designs = lines.sorted().collect();

    (towels, designs)
}

fn count_combinations<'a>(
    cache: &mut HashMap<&'a str, usize>,
    towels: &HashSet<&str>,
    min_towel: usize,
    max_towel: usize,
    design: &'a str,
) -> usize {
    if design.is_empty() {
        1
    } else {
        match cache.get(design) {
            Some(result) => *result,
            None => {
                let mut result = 0;

                for len in min_towel..=min(max_towel, design.len()) {
                    if towels.contains(&design[0..len]) {
                        result +=
                            count_combinations(cache, towels, min_towel, max_towel, &design[len..])
                    }
                }
                cache.insert(design, result);

                result
            }
        }
    }
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        19
    }

    fn part1(&self, input: &str) -> Option<String> {
        let (towels, designs) = parse(input);
        let towels = Regex::new(&format!("^({})+$", towels.iter().join("|"))).unwrap();

        Some(
            designs
                .iter()
                .filter(|design| towels.is_match(design))
                .count(),
        )
        .map(|r| r.to_string())
    }
    fn part2(&self, input: &str) -> Option<String> {
        let (towels, designs) = parse(input);
        let towels = HashSet::from_iter(towels);

        let min_towel = towels.iter().map(|towel| towel.len()).min().unwrap();
        let max_towel = towels.iter().map(|towel| towel.len()).max().unwrap();

        Some(
            designs
                .iter()
                .map(|design| {
                    count_combinations(&mut HashMap::new(), &towels, min_towel, max_towel, design)
                })
                .sum::<usize>(),
        )
        .map(|r| r.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::Day;

    const DAY: super::Day = super::Day;
    #[test]
    fn part1_example1() {
        let text = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(DAY.part1(text), Some(6.to_string()))
    }
    #[test]
    fn part2_example1() {
        let text = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(DAY.part2(text), Some(16.to_string()))
    }
    #[test]
    fn count_combinations_short() {
        assert_eq!(
            count_combinations(&mut HashMap::new(), &HashSet::from(["a"]), 1, 1, "a"),
            1
        );
    }
}
