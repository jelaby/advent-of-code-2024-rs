use crate::days;
use num::abs;
use pathfinding::prelude::astar;

pub struct Day;

impl Day {}

fn parse(input: &str) -> (Vec<(usize, usize)>, (usize, usize)) {
    let coords = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let max_x = coords.iter().map(|c| c.0).max().unwrap();
    let max_y = coords.iter().map(|c| c.1).max().unwrap();

    (coords, (max_x, max_y))
}

fn corrupt_memory(coords: &Vec<(usize, usize)>, max: (usize, usize), n: usize) -> Vec<Vec<bool>> {
    let mut result = vec![vec![false; max.0 + 1]; max.1 + 1];

    for i in 0..n {
        let coord = coords[i];

        result[coord.1][coord.0] = true;
    }

    result
}

fn cost_of(map: &Vec<Vec<bool>>, max: (usize, usize)) -> Option<i64> {
    if let Some((_, cost)) = astar(
        &(0, 0),
        |&(x, y)| {
            let mut result = vec![];
            if x > 0 && !map[y][x - 1] {
                result.push(((x - 1, y), 1))
            }
            if x < max.0 && !map[y][x + 1] {
                result.push(((x + 1, y), 1))
            }
            if y > 0 && !map[y - 1][x] {
                result.push(((x, y - 1), 1))
            }
            if y < max.1 && !map[y + 1][x] {
                result.push(((x, y + 1), 1))
            }
            result
        },
        |&(x, y)| abs((max.0 as i64) - (x as i64)) + abs((max.1 as i64) - (y as i64)),
        |&p| p == max,
    ) {
        Some(cost)
    } else {
        None
    }
}

fn do_part1(input: &str, n: usize) -> Option<i64> {
    let (coords, max) = parse(input);

    let map = corrupt_memory(&coords, max, n);

    cost_of(&map, max)
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        18
    }

    fn part1(&self, input: &str) -> Option<String> {
        do_part1(input, 1024).map(|r| r.to_string())
    }
    fn part2(&self, input: &str) -> Option<String> {
        let (coords, max) = parse(input);

        let mut bottom = 0;
        let mut top = coords.len();

        loop {
            let n = (bottom + top) / 2;

            let map = corrupt_memory(&coords, max, n);

            match cost_of(&map, max) {
                Some(_) => bottom = n,
                None => top = n,
            }

            if top == bottom + 1 {
                return Some(format!("{},{}", coords[bottom].0, coords[bottom].1));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;

    const DAY: super::Day = super::Day;
    #[test]
    fn part1_example1() {
        let text = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!(super::do_part1(text, 12), Some(22))
    }
    #[test]
    fn part2_example1() {
        let text = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!(DAY.part2(text), Some("6,1".to_string()))
    }
}
