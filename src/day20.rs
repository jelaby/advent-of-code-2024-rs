use crate::days;
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

fn find_cheats(
    map: &Vec<Vec<bool>>,
    cost_from_start: &Vec<Vec<i64>>,
    cost_to_end: &Vec<Vec<i64>>,
) -> Vec<((i64, i64), (i64, i64), i64)> {
    let mut result = vec![];

    for y in 1..(map.len() as i64 - 1) {
        for x in 1..(map[y as usize].len() as i64 - 1) {
            let cost_to_here = cost_from_start[y as usize][x as usize];
            for &(dy, dx) in &DIRS {
                let nx = x + dx;
                let ny = y + dy;
                let nnx = nx + dx;
                let nny = ny + dy;

                if get(map, nx, ny, true) {
                    if !get(map, nnx, nny, true) {
                        let cost_from_here = get(cost_to_end, nnx, nny, WALL_COST);
                        result.push(((x, y), (dx, dy), cost_to_here + 2 + cost_from_here));
                    }
                }
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
) -> usize {
    let costs_from_start = calculate_costs(map, start);
    let costs_to_end = calculate_costs(map, end);

    let normal_cost = costs_from_start[end.1][end.0];
    assert_eq!(normal_cost, costs_to_end[start.1][start.0]);

    let cheats = find_cheats(map, &costs_from_start, &costs_to_end);

    cheats
        .iter()
        .map(|(_, _, cost)| cost)
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
        Some(count_cheats(&map, start, end, 100)).map(|r| r.to_string())
    }
    fn part2(&self, input: &str) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;

    const DAY: super::Day = super::Day;
    #[test]
    fn part1_example1_full() {
        let text = "\
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
        assert_eq!(DAY.part1(text), Some(0.to_string()))
    }
    #[test]
    fn part1_example1() {
        let text = "\
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
        let (map, start, end) = super::parse(text);
        assert_eq!(super::count_cheats(&map, start, end, 64), 1);
        assert_eq!(super::count_cheats(&map, start, end, 60), 1);
        assert_eq!(super::count_cheats(&map, start, end, 41), 1);
        assert_eq!(super::count_cheats(&map, start, end, 40), 2);
        assert_eq!(super::count_cheats(&map, start, end, 10), 10);
    }
    #[test]
    fn part1_smallest() {
        let text = "\
#####
#S#E#
#####";
        let (map, start, end) = super::parse(text);
        assert_eq!(super::count_cheats(&map, start, end, 100), 1);
    }
    #[test]
    fn part2_example1() {
        let text = "";
        assert_eq!(DAY.part2(text), Some("4".to_string()))
    }
}
