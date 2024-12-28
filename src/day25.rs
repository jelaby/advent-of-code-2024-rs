use std::cmp::max;
use crate::days;

pub struct Day;

impl Day {}

fn parse(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    input.split_terminator("\n\n")
        .for_each(|schematic| {
            let mut lines = schematic.lines();
            if lines.next().unwrap() == "#####" {
                let lock = lines.enumerate()
                    .fold(vec![0usize;5], |mut acc, (i, line)| {
                        line.chars()
                            .map(|c| match c {
                                '#' => i+1,
                                _ => 0
                            })
                            .zip(acc)
                            .map(|(a,b)| max(a,b))
                            .collect::<Vec<_>>()
                    });
                locks.push(lock);
            } else {
                let key = lines.enumerate()
                    .fold(vec![0usize;5], |mut acc, (i, line)| {
                        line.chars()
                            .map(|c| match c {
                                '#' => 5-i,
                                _ => 0
                            })
                            .zip(acc)
                            .map(|(a,b)| max(a,b))
                            .collect::<Vec<_>>()
                    });
                keys.push(key);
            }
        });
        (locks, keys)
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        25
    }

    fn part1(&self, input: &str) -> Option<String> {
        let (locks, keys) = parse(input);

        Some(keys.iter()
            .map(|key| locks.iter()
                .filter(|lock| lock.iter().zip(key.iter()).all(|(l,k)| k + l <= 5))
                .count())
            .sum::<usize>())
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
    fn parse_lock() {
        let text = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        let (locks, _) = parse(text);
        assert_eq!(locks, vec![vec![0,5,3,4,3], vec![1,2,0,5,3]])
    }

    #[test]
    fn parse_key() {
        let text = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        let (_, keys) = parse(text);
        assert_eq!(keys, vec![vec![5,0,2,1,3], vec![4,3,4,0,2], vec![3,0,2,0,1]])
    }
    #[test]
    fn part1_example1() {
        let text = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        assert_eq!(DAY.part1(text), Some(3.to_string()))
    }
    #[test]
    fn part2_example1() {
        let text = "";
        assert_eq!(DAY.part2(text), Some("4".to_string()))
    }
}
