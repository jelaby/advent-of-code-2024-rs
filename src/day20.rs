use crate::days;
use num::abs;
use std::collections::VecDeque;

pub struct Day;

impl Day {}

fn parse(input: &str) -> (Vec<Vec<bool>>, (usize, usize), (usize, usize)) {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter().enumerate().find_map(|(x, c)| {
                if *c == 'S' {
                    Some((x as usize, y as usize))
                } else {
                    None
                }
            })
        })
        .unwrap();
    let end = map
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter().enumerate().find_map(|(x, c)| {
                if *c == 'E' {
                    Some((x as usize, y as usize))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let map = map
        .iter()
        .map(|row| row.iter().map(|c| *c == '#').collect())
        .collect();

    return (map, start, end);
}

const WALL_COST: i64 = i32::MAX as i64;

fn calculate_costs(map: &Vec<Vec<bool>>, start: (usize, usize)) -> Vec<Vec<i64>> {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    let mut result = vec![vec![WALL_COST; map[0].len()]; map.len()];

    while !queue.is_empty() {
        let ((x, y), cost) = queue.pop_front().unwrap();

        if !map[y][x] && result[y][x] > cost {
            result[y][x] = cost;
            queue.push_back(((x + 1, y), cost + 1));
            queue.push_back(((x - 1, y), cost + 1));
            queue.push_back(((x, y + 1), cost + 1));
            queue.push_back(((x, y - 1), cost + 1));
        }
    }

    result
}

fn get<T: Copy>(map: &Vec<Vec<T>>, x: i64, y: i64, default: T) -> T {
    if x < 0 || y < 0 || y >= map.len() as i64 || x >= map[0].len() as i64 {
        default
    } else {
        map[y as usize][x as usize]
    }
}

const DIRS: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn find_modern_cheats_from<F>(
    map: &Vec<Vec<bool>>,
    cost_from_start: &Vec<Vec<i64>>,
    cost_to_end: &Vec<Vec<i64>>,
    x: i64,
    y: i64,
    max_cheat: i64,
    callback: &mut F,
) where
    F: FnMut(i64),
{
    let cost_to_here = cost_from_start[y as usize][x as usize];

    for dy in -max_cheat..=max_cheat {
        let max_dx = max_cheat - abs(dy);
        for dx in -max_dx..=max_dx {
            if dx != 0 || dy != 0 {
                let (nx, ny) = (x + dx, y + dy);
                if !get(map, nx, ny, true) {
                    let cost_from_here = cost_to_end[ny as usize][nx as usize];
                    let cheat_time = abs(dx) + abs(dy);
                    callback(cost_to_here + cheat_time + cost_from_here);
                }
            }
        }
    }
}

fn find_modern_cheats(
    map: &Vec<Vec<bool>>,
    cost_from_start: &Vec<Vec<i64>>,
    cost_to_end: &Vec<Vec<i64>>,
    max_cheat: i64,
) -> Vec<i64> {
    let mut result = vec![];

    for y in 1..(map.len() as i64 - 1) {
        for x in 1..(map[y as usize].len() as i64 - 1) {
            if !get(map, x, y, true) {
                find_modern_cheats_from(
                    map,
                    cost_from_start,
                    cost_to_end,
                    x,
                    y,
                    max_cheat,
                    &mut |cheat_time| result.push(cheat_time),
                );
            }
        }
    }

    result
}

fn count_cheats(
    map: &Vec<Vec<bool>>,
    start: (usize, usize),
    end: (usize, usize),
    saving: i64,
    max_cheat: i64,
) -> usize {
    let costs_from_start = calculate_costs(map, start);
    let costs_to_end = calculate_costs(map, end);

    let normal_cost = costs_from_start[end.1][end.0];
    assert_eq!(normal_cost, costs_to_end[start.1][start.0]);

    let cheats = find_modern_cheats(map, &costs_from_start, &costs_to_end, max_cheat);

    cheats
        .iter()
        .map(|cost| cost)
        .map(|&cost| normal_cost - cost)
        .filter(|&cheat_saving| cheat_saving >= saving)
        .count()
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        20
    }

    fn part1(&self, input: &str) -> Option<String> {
        let (map, start, end) = parse(input);
        Some(count_cheats(&map, start, end, 100, 2)).map(|r| r.to_string())
    }
    fn part2(&self, input: &str) -> Option<String> {
        let (map, start, end) = parse(input);
        Some(count_cheats(&map, start, end, 100, 20)).map(|r| r.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::Day;

    const DAY: super::Day = super::Day;
    const TEXT: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    #[test]
    fn part1_example1_full() {
        assert_eq!(DAY.part1(TEXT), Some(0.to_string()))
    }
    #[test]
    fn part1_example1() {
        let (map, start, end) = parse(TEXT);
        assert_eq!(count_cheats(&map, start, end, 64, 2), 1);
        assert_eq!(count_cheats(&map, start, end, 60, 2), 1);
        assert_eq!(count_cheats(&map, start, end, 41, 2), 1);
        assert_eq!(count_cheats(&map, start, end, 40, 2), 2);
        assert_eq!(count_cheats(&map, start, end, 10, 2), 10);
    }
    #[test]
    fn part1_smallest() {
        let text = "\
#####
#S#E#
#####";
        let (map, start, end) = parse(text);
        assert_eq!(count_cheats(&map, start, end, 100, 2), 1);
    }
    #[test]
    fn part2_example77() {
        let (map, start, end) = parse(TEXT);
        assert_eq!(count_cheats(&map, start, end, 77, 20), 0);
    }
    #[test]
    fn part2_example76() {
        let (map, start, end) = parse(TEXT);
        assert_eq!(count_cheats(&map, start, end, 76, 20), 3);
    }
    #[test]
    fn part2_example74() {
        let (map, start, end) = parse(TEXT);
        assert_eq!(count_cheats(&map, start, end, 74, 20), 3 + 4);
    }
    #[test]
    fn part2_example72() {
        let (map, start, end) = parse(TEXT);
        assert_eq!(count_cheats(&map, start, end, 72, 20), 3 + 4 + 22);
    }
    #[test]
    fn part2_example70() {
        let (map, start, end) = parse(TEXT);
        assert_eq!(count_cheats(&map, start, end, 70, 20), 3 + 4 + 22 + 12);
    }
    #[test]
    fn part2_example68() {
        let (map, start, end) = parse(TEXT);
        assert_eq!(count_cheats(&map, start, end, 68, 20), 3 + 4 + 22 + 12 + 14);
    }
    #[test]
    fn part2_jump_20() {
        let (map, start, end) = parse(
            "\
########
#S #  E#
## ##  #
## ### #
##     #
########
",
        );
        assert_eq!(count_cheats(&map, start, end, 1, 2), 1);
        assert_eq!(count_cheats(&map, start, end, 1, 3), 4);
        assert_eq!(count_cheats(&map, start, end, 1, 4), 6);
    }
}
