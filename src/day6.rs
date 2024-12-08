use crate::days;
use std::ops::Add;
use enumset::{EnumSet, EnumSetType};

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

impl Add<&Dir> for &Point {
    type Output = Point;

    fn add(self, rhs: &Dir) -> Self::Output {
        go(self, rhs)
    }
}

impl Add<Dir> for Point {
    type Output = Point;

    fn add(self, rhs: Dir) -> Self::Output {
        &self + &rhs
    }
}

pub struct Day;

impl Day {}


fn parse(input: &str) -> (Vec<Vec<bool>>, Point, Dir) {
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

    let d = Dir::Up;

    (map, p, d)
}

#[derive(Debug, EnumSetType)]
enum Dir {
    Up, Down, Left, Right
}

fn go(p: &Point, d: &Dir) -> Point {
    match d {
        Dir::Up => Point::new(p.x, p.y - 1),
        Dir::Down => Point::new(p.x, p.y + 1),
        Dir::Left => Point::new(p.x - 1, p.y),
        Dir::Right => Point::new(p.x + 1, p.y),
    }
}

fn reverse(p: &Point, d: &Dir) -> Point {
    match d {
        Dir::Up => Point::new(p.x, p.y + 1),
        Dir::Down => Point::new(p.x, p.y - 1),
        Dir::Left => Point::new(p.x + 1, p.y),
        Dir::Right => Point::new(p.x - 1, p.y),
    }
}

fn turn(d: &Dir) -> Dir {
    match d {
        Dir::Up => Dir::Right,
        Dir::Down => Dir::Left,
        Dir::Left => Dir::Up,
        Dir::Right => Dir::Down,
    }
}

struct MapIterator<'a> {
    map: &'a Vec<Vec<bool>>,
    p: Point,
    d: Dir
}

impl MapIterator<'_> {
    fn new<'a>(map: &'a Vec<Vec<bool>>, p: &'_ Point, d: &'_ Dir) -> MapIterator<'a> {
        MapIterator { map, p: *p, d: *d }
    }
}

impl Iterator for MapIterator<'_> {
    type Item = (Point, Dir);

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
                self.d = turn(&self.d);
            } else {
                self.p = p_next;
                return Some((self.p, self.d));
            }
        }
    }
}

fn find_visited(map: &Vec<Vec<bool>>, p: &Point, d: &Dir) -> Vec<Vec<bool>> {
    let mut result = vec![vec![false; map[0].len()]; map.len()];

    for (p, _) in MapIterator::new(map, p, d) {
        result[p.y as usize][p.x as usize] = true;
    }

    result
}

fn does_it_loop(map: &Vec<Vec<bool>>, p: &Point, d: &Dir, visits: &Vec<Vec<EnumSet::<Dir>>>) -> bool {
    let visits = &mut visits.clone();
    for (p, d) in MapIterator::new(map, p, d) {
        if visits[p.y as usize][p.x as usize].contains(d) {
            return true;
        } else {
            visits[p.y as usize][p.x as usize] |= d;
        }
    }

    return false;
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

        let mut result = 0;
        let mut visits = vec![vec![EnumSet::<Dir>::new(); map[0].len()]; map.len()];
        for (p, d) in MapIterator::new(&map.clone(), &p, &d) {
            map[p.y as usize][p.x as usize] = true;

            if does_it_loop(&map, &reverse(&p, &d), &d, &visits) {
                result += 1;
            }
            map[p.y as usize][p.x as usize] = false;
            visits[p.y as usize][p.x as usize] |= d;
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
