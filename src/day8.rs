use crate::days;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

pub struct Day;

impl Day {}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Vector {
    x: i64,
    y: i64,
}

impl Sub<&Point> for &Point {
    type Output = Vector;

    fn sub(self, rhs: &Point) -> Self::Output {
        Vector { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        &self - &rhs
    }
}

impl Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub<&Vector> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Vector) -> Self::Output {
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        8
    }

    fn part1(&self, input: &str) -> Option<i64> {
        let freq_antennae = input.split_terminator('\n').enumerate()
            .flat_map(|(y, line)| line.chars().enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, c)| ((x, y), c)))
            .fold(HashMap::new(), |mut m, ((x, y), c)| {
                m.entry(c).or_insert_with(|| vec![]).push(Point{x: x as i64, y: y as i64});
                m
            });

        let width = input.split_terminator('\n').next().unwrap().len() as i64;
        let height = input.split_terminator('\n').count() as i64;

        let mut antinodes = HashSet::new();

        for (_, antennae) in freq_antennae.iter() {
            for a in antennae {
                for b in antennae {
                    if a != b {
                        let diff = b - a;
                        let antinode = b + &diff;
                        if antinode.x >=0 && antinode.y >=0 && antinode.x < width && antinode.y < height {
                            antinodes.insert(antinode);
                        }
                        let antinode = a - &diff;
                        if antinode.x >=0 && antinode.y >=0 && antinode.x < width && antinode.y < height {
                            antinodes.insert(antinode);
                        }
                    }
                }
            }
        }

        Some(antinodes.len() as i64)
    }
    fn part2(&self, input: &str) -> Option<i64> {
        let freq_antennae = input.split_terminator('\n').enumerate()
            .flat_map(|(y, line)| line.chars().enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, c)| ((x, y), c)))
            .fold(HashMap::new(), |mut m, ((x, y), c)| {
                m.entry(c).or_insert_with(|| vec![]).push(Point{x: x as i64, y: y as i64});
                m
            });

        let width = input.split_terminator('\n').next().unwrap().len() as i64;
        let height = input.split_terminator('\n').count() as i64;

        let mut antinodes = HashSet::new();

        for (_, antennae) in freq_antennae.iter() {
            for a in antennae {
                for b in antennae {
                    if a != b {
                        let diff = b - a;

                        let mut antinode = *a;
                        while antinode.x >=0 && antinode.y >=0 && antinode.x < width && antinode.y < height {
                            antinodes.insert(antinode);

                            antinode = &antinode - &diff;
                        }

                        let mut antinode = *b;
                        while antinode.x >=0 && antinode.y >=0 && antinode.x < width && antinode.y < height {
                            antinodes.insert(antinode);

                            antinode = &antinode + &diff;
                        }
                    }
                }
            }
        }

        Some(antinodes.len() as i64)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;

    const DAY: super::Day = super::Day;
    #[test]
    fn part1_no_nodes() {
        let text = "\
............
............
............
............
............
............
............
............
............
............
............
............
";
        assert_eq!(DAY.part1(text), Some(0))
    }
    #[test]
    fn part1_one_node() {
        let text = "\
............
............
............
............
............
......A.....
............
............
............
............
............
............
";
        assert_eq!(DAY.part1(text), Some(0))
    }
    #[test]
    fn part1_two_nodes() {
        let text = "\
............
............
............
............
............
......A.....
............
.......A....
............
............
............
............
";
        assert_eq!(DAY.part1(text), Some(2))
    }
    #[test]
    fn part1_example1() {
        let text = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
        assert_eq!(DAY.part1(text), Some(14))
    }
    #[test]
    fn part2_example1() {
        let text = "\
T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";
        assert_eq!(DAY.part2(text), Some(9))
    }
    #[test]
    fn part2_example2() {
        let text = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
        assert_eq!(DAY.part2(text), Some(34))
    }
}
