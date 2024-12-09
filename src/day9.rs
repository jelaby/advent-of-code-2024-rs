use crate::days;
use itertools::Itertools;
use std::cmp::{max, min};

pub struct Day;

// Not: 6434671719741 -- too high

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

fn is_big_enough_gap(map: &Vec<i32>, target: usize, size: usize) -> bool {
    for target_end in target..min(map.len(), target + size) {
        if map[target_end] >= 0 {
            return false;
        }
    }
    return true;
}

fn defrag(map: &mut Vec<i32>) {

    let mut contiguous_prefix = 0;
    let mut i = map.len() - 1;
    while i > contiguous_prefix {
        let id = map[i];
        let mut j = i;
        while j > 0 && map[j - 1] == id {
            j -= 1;
        }

        if id >= 0 {
            let size = i + 1 - j;

            let mut target = contiguous_prefix;
            let mut is_contiguous = true;

            'mv: loop {
                if target >= i {
                    break 'mv;
                }
                if map[target] >= 0 {
                    if is_contiguous {
                        contiguous_prefix = target;
                    }
                    target += 1;
                } else {
                    is_contiguous = false;

                    if is_big_enough_gap(&map, target, size) {
                        for k in 0..size {
                            map[target + k] = map[j + k];
                            map[j + k] = -1;
                        }
                        break 'mv;
                    } else {
                        target += 1;
                    }
                }
            }
        }
        i = j - 1;
    }
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

        Some(
            map.iter()
                .enumerate()
                .map(|(i, id)| i as i64 * (max(*id as i64, 0)))
                .sum(),
        )
    }
    fn part2(&self, input: &str) -> Option<i64> {
        let mut map = parse(input);

        defrag(&mut map);

        Some(
            map.iter()
                .enumerate()
                .map(|(i, id)| i as i64 * (max(*id as i64, 0)))
                .sum(),
        )
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
        let text = "2333133121414131402";
        assert_eq!(DAY.part2(text), Some(2858))
    }

    #[test]
    fn defrag() {
        let mut map = vec![0,0,-1,-1,1];
        super::defrag(&mut map);
        assert_eq!(map, [0,0,1,-1,-1]);
    }

    #[test]
    fn defrag_size_one() {
        let mut map = vec![0,0,-1,-1,1,1,1,2,3,3,3];
        super::defrag(&mut map);
        assert_eq!(map, [0,0,2,-1,1,1,1,-1,3,3,3]);
    }

    #[test]
    fn defrag_size_one_exact() {
        let mut map = vec![0,0,-1,1,1,1,2,3,3,3];
        super::defrag(&mut map);
        assert_eq!(map, [0,0,2,1,1,1,-1,3,3,3]);
    }

    #[test]
    fn defrag_exact_fit() {
        let mut map = vec![0,0,-1,-1,1,1,2,2];
        super::defrag(&mut map);
        assert_eq!(map, [0,0,2,2,1,1,-1,-1]);
    }

    #[test]
    fn defrag_exact_fit_once() {
        let mut map = vec![0,0,-1,-1,-1,1,1,2,2];
        super::defrag(&mut map);
        assert_eq!(map, [0,0,2,2,-1,1,1,-1,-1]);
    }

    #[test]
    fn defrag_fits_adjacent() {
        let mut map = vec![0,0,-1,-1,1,1];
        super::defrag(&mut map);
        assert_eq!(map, [0,0,1,1,-1,-1]);
    }
    #[test]
    fn defrag_too_big_to_move_adjacent() {
        let mut map = vec![0,0,-1,-1,1,1,1];
        super::defrag(&mut map);
        assert_eq!(map, [0,0,-1,-1,1,1,1]);
    }

    #[test]
    fn defrag_not_backwards() {
        let mut map = vec![0,0,-1,1,1,-1,-1,2];
        super::defrag(&mut map);
        assert_eq!(map, [0,0,2,1,1,-1,-1,-1]);
    }

    #[test]
    fn defrag_step_target_bug() {
        let mut map = vec![0,0,-1,1,1,-1,-1,-1,-1,2,2,2,2];
        super::defrag(&mut map);
        assert_eq!(map, [0,0,-1,1,1,2,2,2,2,-1,-1,-1,-1]);
    }
}
