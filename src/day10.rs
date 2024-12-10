use std::cell::{Cell, RefCell};
use crate::days;
use std::collections::HashSet;
use std::rc::Rc;

pub struct Day;

impl Day {}

fn value(c: char) -> i32 {
    c as i32 - '0' as i32
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .split_terminator('\n')
        .map(|line| line.chars().map(value).collect())
        .collect()
}

const DIRS: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn find_score(
    map: &Vec<Vec<i32>>,
    scores: &mut Vec<Vec<Option<Rc<RefCell<HashSet<(usize, usize)>>>>>>,
    x: usize,
    y: usize,
) {
    if scores[y][x].is_some() {
    } else if map[y][x] == 9 {
        let mut result = HashSet::new();
        result.insert((x, y));
        scores[y][x] = Some(Rc::new(RefCell::new(result)));
    } else {
        let score = DIRS
            .iter()
            .filter_map(|&(dx, dy)| {
                let (x2, y2) = (x as i64 + dx, y as i64 + dy);
                if x2 >= 0
                    && x2 < map[y].len() as i64
                    && y2 >= 0
                    && y2 < map.len() as i64
                    && map[y2 as usize][x2 as usize] == map[y][x] + 1
                {
                    find_score(map, scores, x2 as usize, y2 as usize);
                    scores[y][x].clone()
                } else {
                    None
                }
            })
            .fold(HashSet::new(), |mut acc, items| {
                let _ = items.borrow().iter().for_each(|&item| {
                    acc.insert(item);
                });
                acc
            });
        scores[y][x] = Some(Rc::new(RefCell::new(score)));
    }
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        10
    }

    fn part1(&self, input: &str) -> Option<i64> {
        let map = parse(input);

        let mut scores = vec![vec![None; map[0].len()]; map.len()];

        Some(
            (0..map.len())
                .map(|y| {
                    (0..map[y].len())
                        .filter(|&x| map[y][x] == 0)
                        .map(|x| {
                            find_score(&map, &mut scores, x, y);
                            scores[y][x].clone().unwrap().borrow().len() as i64
                        })
                        .sum::<i64>()
                })
                .sum(),
        )
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
    fn part1_simple() {
        let text = "\
0123
7654
8912";
        assert_eq!(DAY.part1(text), Some(1))
    }
    #[test]
    fn part1_two_heads() {
        let text = "\
01234
56765
09890";
        assert_eq!(DAY.part1(text), Some(2))
    }
    #[test]
    fn part1_example1() {
        let text = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(DAY.part1(text), Some(36))
    }
    #[test]
    fn part2_example1() {
        let text = "";
        assert_eq!(DAY.part2(text), Some(4))
    }
}
