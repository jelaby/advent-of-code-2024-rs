use std::arch::x86_64::_xgetbv;
use num::abs;
use crate::days;
use pathfinding::prelude::astar;

pub struct Day;

impl Day {}

fn parse(input: &str) -> (Vec<Vec<char>>, (usize,usize), (usize,usize)) {
    let map: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    let start = map.iter().enumerate()
        .find_map(|(y,line)| line.iter().enumerate().find_map(|(x,c)| if *c == 'S' { Some((x,y))} else { None})).unwrap();
    let end = map.iter().enumerate()
        .find_map(|(y,line)| line.iter().enumerate().find_map(|(x,c)| if *c == 'E' { Some((x,y))} else { None})).unwrap();

    return (map, start, end);
}

fn find_shortest_route(map: Vec<Vec<char>>, start: (usize,usize), end: (usize,usize)) -> i64 {
    let (path, cost) = astar(&(start,(1i32,0i32)),
        |&(p,(dx,dy))| {
            let mut result = vec![
                ((p,(dy,dx)), 1000),
                ((p,(-dy,-dx)), 1000),
            ];
            let advance = ((p.0 as i32 + dx) as usize, (p.1 as i32 + dy) as usize);
            if map[advance.1][advance.0] != '#' {
                result.push(((advance, (dx,dy)), 1))
            }
            result
        },
        |(p,_)| abs(p.0 as i32-end.0 as i32) + abs(p.1 as i32 - end.1 as i32),
        |(p, _)| *p == end)
        .unwrap();

    cost as i64
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        16
    }

    fn part1(&self, input: &str) -> Option<i64> {

        let (map, start, end) = parse(input);
        Some(find_shortest_route(map, start, end))
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
        let text = "";
        assert_eq!(DAY.part2(text), Some(4))
    }
}
