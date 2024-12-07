use crate::days;
use std::ops::Add;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub struct Day;

impl Day {}


fn parse(input: &str) -> (Vec<Vec<bool>>, Point, Point) {
    let map = input
        .split_terminator('\n')
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect::<Vec<_>>();

    let p = input
        .split_terminator('\n')
        .enumerate()
        .find_map(
            |(y, line)| match line.chars().enumerate().find(|(_, c)| *c == '^') {
                Some((x, _)) => Some(Point::new(x as i64, y as i64)),
                None => None,
            },
        )
        .unwrap();

    let d = Point::new(0, -1);

    (map, p, d)
}

struct MapIterator<'a> {
    map: &'a Vec<Vec<bool>>,
    p: Point,
    d: Point
}

impl MapIterator<'_> {
    fn new<'a>(map: &'a Vec<Vec<bool>>, p: &'_ Point, d: &'_ Point) -> MapIterator<'a> {
        MapIterator { map, p: *p, d: *d }
    }
}

impl Iterator for MapIterator<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let p_next = self.p + self.d;

            if p_next.y < 0
                || p_next.y >= self.map.len() as i64
                || p_next.x < 0
                || p_next.x >= self.map[p_next.y as usize].len() as i64
            {
                return None;
            }

            if self.map[p_next.y as usize][p_next.x as usize] {
                self.d = Point::new(self.d.y * -1, self.d.x);
            } else {
                self.p = p_next;
                return Some(self.p);
            }
        }
    }
}

fn find_visited(map: &Vec<Vec<bool>>, p: &Point, d: &Point) -> Vec<Vec<bool>> {
    let mut result = vec![vec![false; map[0].len()]; map.len()];

    for p in MapIterator::new(map, p, d) {
        result[p.y as usize][p.x as usize] = true;
    }

    result
}

fn does_it_loop(map: &Vec<Vec<bool>>, p: &Point, d: &Point) -> bool {
    let mut visits = vec![vec![Vec::<Point>::new(); map[0].len()]; map.len()];

    let mut p = *p;
    let mut d = *d;

    loop {
        if visits[p.y as usize][p.x as usize].contains(&d) {
            return true;
        } else {
            visits[p.y as usize][p.x as usize].push(d);
        }
        let p_next = p + d;

        if p_next.y < 0
            || p_next.y >= map.len() as i64
            || p_next.x < 0
            || p_next.x >= map[p_next.y as usize].len() as i64
        {
            return false;
        }

        if map[p_next.y as usize][p_next.x as usize] {
            d = Point::new(d.y * -1, d.x);
        } else {
            p = p_next;
        }
    }
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        6
    }

    fn part1(&self, input: &str) -> Option<i64> {
        let (map, p, d) = parse(input);

        let result = find_visited(&map, &p, &d);

        Some(
            result
                .iter()
                .map(|row| row.iter().filter(|&r| *r).count() as i64)
                .sum(),
        )
    }
    fn part2(&self, input: &str) -> Option<i64> {
        let (mut map, p, d) = parse(input);

        let visited = find_visited(&map, &p, &d);

        let mut result = 0;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if visited[y][x] {
                    map[y][x] = true;

                    if does_it_loop(&map, &p, &d) {
                        result += 1;
                    }
                    map[y][x] = false;
                }
            }
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;

    const DAY: super::Day = super::Day;
    #[test]
    fn part1_example1() {
        let text = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...\
";
        assert_eq!(DAY.part1(text), Some(41))
    }
    #[test]
    fn part2_example1() {
        let text = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...\
";
        assert_eq!(DAY.part2(text), Some(6))
    }
}
