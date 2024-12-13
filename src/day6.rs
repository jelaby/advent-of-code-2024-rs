use crate::days;
use enumset::{EnumSet, EnumSetType};
use nalgebra::Vector2;
use std::ops::{Add, Sub};

pub struct Day;

impl Day {}

fn parse(input: &str) -> (Vec<Vec<bool>>, Vector2<i64>, Dir) {
    let map = input
        .split_terminator('\n')
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect::<Vec<_>>();

    let p = input
        .split_terminator('\n')
        .enumerate()
        .find_map(
            |(y, line)| match line.chars().enumerate().find(|(_, c)| *c == '^') {
                Some((x, _)) => Some(Vector2::new(x as i64, y as i64)),
                None => None,
            },
        )
        .unwrap();

    let d = Dir::Up;

    (map, p, d)
}

#[derive(Debug, EnumSetType)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl<T: Copy + Sub<T, Output=R> + Add<T, Output=R> + From<i32>, R: Copy + From<T>> Add<Dir> for Vector2<T> {
    type Output = Vector2<R>;

    fn add(self, rhs: Dir) -> Self::Output {
        match rhs {
            Dir::Up => Vector2::new(self[0].into(), self[1] - 1.into()),
            Dir::Down => Vector2::new(self[0].into(), self[1] + 1.into()),
            Dir::Left => Vector2::new(self[0] - 1.into(), self[1].into()),
            Dir::Right => Vector2::new(self[0] + 1.into(), self[1].into()),
        }
    }
}

impl<T: Copy + Sub<T, Output=R> + Add<T, Output=R> + From<i32>, R: Copy + From<T>> Sub<Dir> for Vector2<T> {
    type Output = Vector2<R>;

    fn sub(self, rhs: Dir) -> Self::Output {
        match rhs {
            Dir::Up => Vector2::new(self[0].into(), self[1] + 1.into()),
            Dir::Down => Vector2::new(self[0].into(), self[1] - 1.into()),
            Dir::Left => Vector2::new(self[0] + 1.into(), self[1].into()),
            Dir::Right => Vector2::new(self[0] - 1.into(), self[1].into()),
        }
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
    p: Vector2<i64>,
    d: Dir,
}

impl MapIterator<'_> {
    fn new<'a>(map: &'a Vec<Vec<bool>>, p: &'_ Vector2<i64>, d: &'_ Dir) -> MapIterator<'a> {
        MapIterator { map, p: *p, d: *d }
    }
}

impl Iterator for MapIterator<'_> {
    type Item = (Vector2<i64>, Dir);

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

fn find_visited(map: &Vec<Vec<bool>>, p: &Vector2<i64>, d: &Dir) -> Vec<Vec<bool>> {
    let mut result = vec![vec![false; map[0].len()]; map.len()];

    for (p, _) in MapIterator::new(map, p, d) {
        result[p.y as usize][p.x as usize] = true;
    }

    result
}

fn does_it_loop(
    map: &Vec<Vec<bool>>,
    p: &Vector2<i64>,
    d: &Dir,
    visits: &Vec<Vec<EnumSet<Dir>>>,
) -> bool {
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

            if does_it_loop(&map, &(p - d), &d, &visits) {
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
