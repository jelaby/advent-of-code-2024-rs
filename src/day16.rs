use crate::days;
use itertools::Itertools;
use num::abs;
use pathfinding::prelude::astar;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, LinkedList};

pub struct Day;

impl Day {}

fn parse(input: &str) -> (Vec<Vec<char>>, (i32, i32), (i32, i32)) {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter().enumerate().find_map(|(x, c)| {
                if *c == 'S' {
                    Some((x as i32, y as i32))
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
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .unwrap();

    return (map, start, end);
}

fn find_shortest_route(map: &Vec<Vec<char>>, start: (i32, i32), end: (i32, i32)) -> i64 {
    let (path, cost) = astar(
        &(start, (1i32, 0i32)),
        |&(p, (dx, dy))| {
            let mut result = vec![((p, (dy, dx)), 1000), ((p, (-dy, -dx)), 1000)];
            let advance = (p.0 + dx, (p.1 + dy));
            if map[advance.1 as usize][advance.0 as usize] != '#' {
                result.push(((advance, (dx, dy)), 1))
            }
            result
        },
        |(p, _)| abs(p.0 - end.0) + abs(p.1 - end.1),
        |(p, _)| *p == end,
    )
    .unwrap();

    cost as i64
}

fn routes_to_goal(
    map: &Vec<Vec<char>>,
    max_cost: i32,
    start: (i32, i32),
    start_dir: (i32, i32),
    end: (i32, i32),
) -> Vec<Vec<bool>> {
    let mut visited = HashMap::new();

    let mut queue: PriorityQueue<((i32, i32), (i32, i32), i32), Reverse<i32>> =
        PriorityQueue::new();

    fn push(
        queue: &mut PriorityQueue<((i32, i32), (i32, i32), i32), Reverse<i32>>,
        p: (i32, i32),
        d: (i32, i32),
        cost: i32,
        end: (i32, i32),
    ) {
        queue.push((p, d, cost), Reverse(abs(p.0 - end.0) + abs(p.1 - end.1)));
    }

    push(&mut queue, start, start_dir, 0, end);

    while !queue.is_empty() {
        let ((p, d, cost), _) = queue.pop().unwrap();

        if cost > max_cost {
            continue;
        } else if map[p.1 as usize][p.0 as usize] == '#' {
            continue;
        } else if map[p.1 as usize][p.0 as usize] == 'E' {
            visited.insert((p, d), cost);
            continue;
        } else {
            if let Some(&old_cost) = visited.get(&(p, d)) {
                if old_cost <= cost {
                    continue;
                }
            }

            visited.insert((p, d), cost);

            push(&mut queue, (p.0 + d.0, p.1 + d.1), d, cost + 1, end);
            push(&mut queue, p, (d.1, d.0), cost + 1000, end);
            push(&mut queue, p, (-d.1, -d.0), cost + 1000, end);
        }
    }

    let mut queue = LinkedList::new();
    let mut result = vec![vec![false; map[0].len()]; map.len()];

    for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        if visited.contains_key(&(end, dir)) {
            queue.push_back((end, dir, max_cost));
        }
    }

    while !queue.is_empty() {
        let (p, d, cost) = queue.pop_back().unwrap();

        match visited.get(&(p, d)) {
            Some(&other_cost) => {
                if other_cost == cost {
                    result[p.1 as usize][p.0 as usize] = true;

                    queue.push_back(((p.0 - d.0, p.1 - d.1), d, cost - 1));
                    queue.push_back((p, (d.1, d.0), cost - 1000));
                    queue.push_back((p, (-d.1, -d.0), cost - 1000));
                }
            }
            None => {}
        }
    }

    return result;
}

fn show_routes(map: &Vec<Vec<char>>, visited: &Vec<Vec<bool>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if visited[y][x] {
                print!("O");
            } else {
                print!("{}", map[y][x])
            }
        }
        println!();
    }
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        16
    }

    fn part1(&self, input: &str) -> Option<i64> {
        let (map, start, end) = parse(input);
        Some(find_shortest_route(&map, start, end))
    }
    fn part2(&self, input: &str) -> Option<i64> {
        let (map, start, end) = parse(input);

        let max_cost = find_shortest_route(&map, start, end);

        let mut visited = routes_to_goal(&map, max_cost as i32, start, (1, 0), end);

        show_routes(&map, &visited);

        visited[start.1 as usize][start.0 as usize] = true;
        visited[end.1 as usize][end.0 as usize] = true;

        Some(
            visited
                .iter()
                .map(|row| row.iter().filter(|x| **x).count() as i64)
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
        let text = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!(DAY.part1(text), Some(7036))
    }
    #[test]
    fn part1_example2() {
        let text = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!(DAY.part1(text), Some(11048))
    }
    #[test]
    fn part2_example1() {
        let text = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!(DAY.part2(text), Some(45))
    }
    #[test]
    fn part2_example2() {
        let text = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!(DAY.part2(text), Some(64))
    }
    #[test]
    fn part2_small() {
        let text = "\
####
#.E#
#S.#
####";
        assert_eq!(DAY.part2(text), Some(3))
    }
    #[test]
    fn part2_small2() {
        let text = "\
#####
#..E#
#S.##
#####";
        assert_eq!(DAY.part2(text), Some(5))
    }
}
