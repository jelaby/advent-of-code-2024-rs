use crate::days;
use std::collections::HashSet;

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
    scores: &mut Vec<Vec<Option<HashSet<(usize, usize)>>>>,
    x: usize,
    y: usize,
) {
    if scores[y][x].is_some() {
    } else if map[y][x] == 9 {
        let mut result = HashSet::new();
        result.insert((x, y));
        scores[y][x] = Some(result);
    } else {
        let score = DIRS
            .iter()
            .filter(|&(dx, dy)| {
                let (x2, y2) = (x as i64 + dx, y as i64 + dy);
                x2 >= 0
                    && x2 < map[y].len() as i64
                    && y2 >= 0
                    && y2 < map.len() as i64
                    && map[y2 as usize][x2 as usize] == map[y][x] + 1
            })
            .fold(HashSet::new(), |mut acc, &(dx, dy)| {
                let (x2, y2) = ((x as i64 + dx) as usize, (y as i64 + dy) as usize);
                find_score(map, scores, x2, y2);
                let _ = scores[y2][x2].as_ref().unwrap().iter().for_each(|item| {
                    acc.insert(*item);
                });
                acc
            });
        scores[y][x] = Some(score);
    }
}
fn find_trailhead_score(
    map: &Vec<Vec<i32>>,
    scores: &mut Vec<Vec<Option<i32>>>,
    x: usize,
    y: usize,
) -> i32 {
    if scores[y][x].is_some() {
        return scores[y][x].unwrap();
    } else if map[y][x] == 9 {
        let mut result = HashSet::new();
        result.insert((x, y));
        scores[y][x] = Some(1);
        return 1;
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
                    Some(find_trailhead_score(map, scores, x2 as usize, y2 as usize))
                } else {
                    None
                }
            })
            .sum();
        scores[y][x] = Some(score);
        return score;
    }
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        10
    }

    fn part1(&self, input: &str) -> Option<String> {
        let map = parse(input);

        let mut scores = vec![vec![None; map[0].len()]; map.len()];

        Some(
            (0..map.len())
                .map(|y| {
                    (0..map[y].len())
                        .filter(|&x| map[y][x] == 0)
                        .map(|x| {
                            find_score(&map, &mut scores, x, y);
                            scores[y][x].clone().unwrap().len() as i64
                        })
                        .sum::<i64>()
                })
                .sum::<i64>(),
        ).map(|r| r.to_string())
    }
    fn part2(&self, input: &str) -> Option<String> {
        let map = parse(input);

        let mut scores = vec![vec![None; map[0].len()]; map.len()];

        Some(
            (0..map.len())
                .map(|y| {
                    (0..map[y].len())
                        .filter(|&x| map[y][x] == 0)
                        .map(|x| find_trailhead_score(&map, &mut scores, x, y) as i64)
                        .sum::<i64>()
                })
                .sum::<i64>(),
        ).map(|r| r.to_string())
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
        assert_eq!(DAY.part1(text), Some("1".to_string()))
    }
    #[test]
    fn part1_two_heads() {
        let text = "\
01234
56765
09890";
        assert_eq!(DAY.part1(text), Some("2".to_string()))
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
        assert_eq!(DAY.part1(text), Some("36".to_string()))
    }
    #[test]
    fn part2_example1() {
        let text = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(DAY.part2(text), Some("81".to_string()))
    }
}
