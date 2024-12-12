use std::collections::HashSet;
use itertools::Itertools;
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
                crate::day12::count(map, neighbour_x as usize, neighbour_y as usize, visited);
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
            let (next_area, next_boundary) = crate::day12::count(&map, x, y, &mut visited);
            score += next_area * next_boundary;
        }
    }

    score
}

fn calculate_area(map: &Vec<Vec<char>>, x: usize, y: usize, visited: &mut Vec<Vec<bool>>) -> usize {
    if visited[y][x] {
        return 0;
    }
    visited[y][x] = true;
    let mut area = 1;
    let t = map[y][x];

    for (dx, dy) in DIR {
        let neighbour_x = x as i64 + dx;
        let neighbour_y = y as i64 + dy;

        if neighbour_x < 0
            || neighbour_x >= map[y].len() as i64
            || neighbour_y < 0
            || neighbour_y >= map.len() as i64
            || map[neighbour_y as usize][neighbour_x as usize] != t
        {} else {
            area += calculate_area(map, neighbour_x as usize, neighbour_y as usize, visited);
        }
    }
    area
}

fn is_same(map: &Vec<Vec<char>>, t: char, x:i64, y:i64) -> bool {
    x >= 0 && x < map[0].len() as i64 && y >= 0 && y < map.len() as i64 && map[y as usize][x as usize] == t
}

fn count_sides(map: &Vec<Vec<char>>, x: usize, y: usize, visited: &mut Vec<Vec<HashSet<(i64,i64)>>>) -> usize {

    let mut result = 0;
    let t = map[y][x];

    for dir in &DIR {
        if !visited[y][x].iter().contains(&dir) {
            visited[y][x].insert(dir.clone());

            let (dx,dy) = *dir;

            let neighbour_x = x as i64 + dx;
            let neighbour_y = y as i64 + dy;

            if !is_same(map, t, neighbour_x, neighbour_y) {
                result += 1;
                trace_edge(&map, x, y, dy,dx, dx, dy, visited);
                trace_edge(&map, x, y, -dy,-dx, dx, dy, visited);
            } else {
                result += count_sides(map, neighbour_x as usize, neighbour_y as usize, visited);
            }
        }
    }

    result
}

fn trace_edge(map: &Vec<Vec<char>>, x: usize, y: usize, dx: i64, dy: i64, wallx:i64, wally:i64, visited: &mut Vec<Vec<HashSet<(i64,i64)>>>) {

    visited[y][x].insert((wallx,wally));

    let next_x = x as i64 + dx;
    let next_y = y as i64 + dy;

    let t = map[y][x];

    if !is_same(&map, t, next_x, next_y) {
        // reached an inside corner -- stop tracing
        return;

    }

    // is the wall still there?
    let over_wall_x = next_x + wallx;
    let over_wall_y = next_y + wally;

    if is_same(map, t, over_wall_x, over_wall_y) {
        // end of the wall -- stop tracing
        return
    }

    // still on a wall, so trace along it further
    trace_edge(&map, next_x as usize, next_y as usize, dx, dy, wallx, wally, visited);
}

fn count_all_2(map: &Vec<Vec<char>>) -> usize{
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut visited2 = vec![vec![HashSet::new(); map[0].len()]; map.len()];
    let mut score = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let next_area = calculate_area(map,x,y,&mut visited);
            if next_area > 0 {
                let sides = count_sides(&map, x, y, &mut visited2);
                score += next_area * sides;
            }
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
        let map = parse(input);
        Some(count_all_2(&map) as i64)
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
        let text = "\
AAAA
BBCD
BBCC
EEEC";
        assert_eq!(DAY.part2(text), Some(80))
    }
    #[test]
    fn part2_example2() {
        let text = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!(DAY.part2(text), Some(236))
    }
    #[test]
    fn part2_example3() {
        let text = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!(DAY.part2(text), Some(368))
    }
    #[test]
    fn part2_example4() {
        let text = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(DAY.part2(text), Some(1206))
    }
    #[test]
    fn part2_small() {
        let text = "O";
        assert_eq!(DAY.part2(text), Some(4))
    }
    #[test]
    fn part2_small2() {
        let text = "OO";
        assert_eq!(DAY.part2(text), Some(8))
    }
    #[test]
    fn part2_two() {
        let text = "AB";
        assert_eq!(DAY.part2(text), Some(8))
    }
}
