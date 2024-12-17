use crate::days;
use num::abs;
use pathfinding::prelude::astar;
use std::collections::HashSet;

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
    visited: &mut Vec<Vec<bool>>,
    passed: &mut HashSet<((i32, i32), (i32, i32))>,
    cost: i32,
    p: (i32, i32),
    d: (i32, i32),
    end: (i32, i32),
) -> bool {
    if cost > max_cost {
        return false;
    }
    if map[p.1 as usize][p.0 as usize] == '#' {
        return false;
    }
    if map[p.1 as usize][p.0 as usize] == 'E' {
        return true;
    }

    if passed.contains(&(p, d)) {
        return false;
    }
    passed.insert((p, d));

    let result1= routes_to_goal(
        map,
        max_cost,
        visited,
        passed,
        cost + 1,
        (p.0 + d.0, p.1 + d.1),
        d,
        end,
    );
    let d2 = (d.1, d.0);
    let result2 = routes_to_goal(
        map,
        max_cost,
        visited,
        passed,
        cost + 1001,
        (p.0 + d.1, p.1 + d.0),
        (d.1, d.0),
        end,
    );
    let result3 = routes_to_goal(
        map,
        max_cost,
        visited,
        passed,
        cost + 1001,
        (p.0 - d.1, p.1 - d.0),
        (-d.1, -d.0),
        end,
    );

    passed.remove(&(p, d));

    if result1 || result2 || result3 {
        visited[p.1 as usize][p.0 as usize] = true;
        return true;
    } else {
        return false;
    }
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

        let mut visited = vec![vec![false; map[0].len()]; map.len()];
        let mut passed = HashSet::new();

        let max_cost = find_shortest_route(&map, start, end);

        routes_to_goal(&map, max_cost as i32, &mut visited, &mut passed,0, start, (1, 0), end);

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
}
