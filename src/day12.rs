use crate::days;

pub struct Day;

impl Day {}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

const DIR: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
fn count(map: &Vec<Vec<char>>, x: usize, y: usize, visited: &mut Vec<Vec<bool>>) -> (usize, usize) {
    if visited[y][x] {
        return (0, 0);
    }

    visited[y][x] = true;

    let mut area = 1;
    let mut boundary = 0;
    let t = map[y][x];

    for (dx, dy) in DIR {
        let neighbour_x = x as i64 + dx;
        let neighbour_y = y as i64 + dy;

        if neighbour_x < 0
            || neighbour_x >= map[y].len() as i64
            || neighbour_y < 0
            || neighbour_y >= map.len() as i64
            || map[neighbour_y as usize][neighbour_x as usize] != t
        {
            boundary += 1;
        } else {
            let (neighbour_area, neighbour_boundary) =
                count(map, neighbour_x as usize, neighbour_y as usize, visited);
            area += neighbour_area;
            boundary += neighbour_boundary;
        }
    }

    (area, boundary)
}

fn count_all(map: &Vec<Vec<char>>) -> usize{
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut score = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let (next_area, next_boundary) = count(&map, x, y, &mut visited);
            score += next_area * next_boundary;
        }
    }

    score
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        12
    }

    fn part1(&self, input: &str) -> Option<i64> {
        let map = parse(input);

        Some(count_all(&map) as i64)
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
AAAA
BBCD
BBCC
EEEC";
        assert_eq!(DAY.part1(text), Some(140))
    }
    #[test]
    fn part1_example2() {
        let text = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(DAY.part1(text), Some(772))
    }
    #[test]
    fn part1_small() {
        let text = "O";
        assert_eq!(DAY.part1(text), Some(4))
    }
    #[test]
    fn part1_small2() {
        let text = "OO";
        assert_eq!(DAY.part1(text), Some(12))
    }
    #[test]
    fn part1_two() {
        let text = "AB";
        assert_eq!(DAY.part1(text), Some(8))
    }
    #[test]
    fn part2_example1() {
        let text = "";
        assert_eq!(DAY.part2(text), Some(4))
    }
}
