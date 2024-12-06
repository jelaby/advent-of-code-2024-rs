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

impl days::Day for Day {
    fn day(&self) -> u32 {
        6
    }

    fn part1(&self, input: &str) -> Option<i64> {
        let map = input
            .split_terminator('\n')
            .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
            .collect::<Vec<_>>();

        let mut p = input
            .split_terminator('\n')
            .enumerate()
            .find_map(
                |(y, line)| match line.chars().enumerate().find(|(_, c)| *c == '^') {
                    Some((x, _)) => Some(Point::new(x as i64, y as i64)),
                    None => None,
                },
            )
            .unwrap();

        let mut d = Point::new(0, -1);

        let mut result = vec![vec![false; map[0].len()]; map.len()];

        loop {
            result[p.y as usize][p.x as usize] = true;
            let p_next = p + d;

            if p_next.y < 0
                || p_next.y >= map.len() as i64
                || p_next.x < 0
                || p_next.x >= map[p_next.y as usize].len() as i64 {
                break;
            }

            if map[p_next.y as usize][p_next.x as usize] {
                d = Point::new(d.y * -1, d.x);
            } else {
                p = p_next;
            }
        }

        map.iter().zip(result.iter())
            .for_each(|(map_row, result_row)| {
                print!("#");
                map_row.iter().zip(result_row.iter())
                    .for_each(|(m,r)| print!("{}", match (m,r) {
                        (false, false) => ' ',
                        (true, false) => '#',
                        (false, true) => '.',
                        (true, true) => '!',
                    }));
                println!("#");
            });

        Some(
            result
                .iter()
                .map(|row| row.iter().filter(|&r| *r).count() as i64)
                .sum(),
        )
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
        let text = "";
        assert_eq!(DAY.part2(text), Some(4))
    }
}
