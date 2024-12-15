use crate::day15::Block::*;
use crate::day15::Dir::*;
use crate::days;
use nalgebra::{Scalar, Vector2};
use num::{Num, NumCast};

pub struct Day;

impl Day {}

#[derive(Copy, Clone, Debug)]
enum Block {
    SPACE,
    ROBOT,
    WALL,
    CRATE,
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

fn parse(input: &str) -> (Vec<Vec<Block>>, Vector2<usize>, Vec<Dir>) {
    let mut parts = input.split_terminator("\n\n");

    let mut map: Vec<Vec<Block>> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => WALL,
                    '.' => SPACE,
                    'O' => CRATE,
                    '@' => ROBOT,
                    _ => panic!("Unrecognised map char {c}"),
                })
                .collect()
        })
        .collect();

    let robot = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find(|(x, &block)| matches!(block, ROBOT))
                .map(|(x, _)| Vector2::new(x, y))
        })
        .unwrap();

    //map[robot.y][robot.x] = SPACE;

    let commands = parts
        .next()
        .unwrap()
        .lines()
        .flat_map(|line| line.chars())
        .map(|c| match c {
            '<' => LEFT,
            '>' => RIGHT,
            '^' => UP,
            'v' => DOWN,
            _ => panic!("Unrecognised command char {c}"),
        })
        .collect();

    (map, robot, commands)
}

fn vec_for(dir: Dir) -> Vector2<i64> {
    match dir {
        LEFT => Vector2::new(-1, 0),
        RIGHT => Vector2::new(1, 0),
        UP => Vector2::new(0, -1),
        DOWN => Vector2::new(0, 1),
    }
}

fn move_block(map: &mut Vec<Vec<Block>>, p: Vector2<usize>, v: Vector2<i64>) -> bool {
    let n = (p.cast() + v).map(|i| i as usize);

    match map[n.y][n.x] {
        SPACE => {
            map[n.y][n.x] = map[p.y][p.x];
            map[p.y][p.x] = SPACE;
            true
        }
        WALL => false,
        CRATE => {
            if move_block(map, n, v) {
                map[n.y][n.x] = map[p.y][p.x];
                map[p.y][p.x] = SPACE;
                true
            } else {
                false
            }
        }
        ROBOT => panic!("Unexpected robot at {p:?}+{v:?} => {n:?}"),
    }
}

fn score(map: &Vec<Vec<Block>>) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, c)| if matches!(c, CRATE) { y * 100 + x } else { 0 })
        })
        .sum()
}

fn show_map(map: &Vec<Vec<Block>>) {
    println!();
    for row in map {
        for block in row {
            print!(
                "{}",
                match block {
                    SPACE => ' ',
                    ROBOT => '@',
                    CRATE => 'O',
                    WALL => '#',
                }
            );
        }
        println!();
    }
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        15
    }

    fn part1(&self, input: &str) -> Option<i64> {
        let (mut map, mut pos, commands) = parse(input);

        show_map(&map);
        for command in commands {
            let dir = vec_for(command);

            if move_block(&mut map, pos, dir) {
                pos = (pos.cast() + dir).map(|i| i as usize);
            }

            //show_map(&map);
        }

        Some(score(&map) as i64)
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
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(DAY.part1(text), Some(10092))
    }
    #[test]
    fn part1_example2() {
        let text = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        assert_eq!(DAY.part1(text), Some(2028))
    }
    #[test]
    fn part2_example1() {
        let text = "";
        assert_eq!(DAY.part2(text), Some(4))
    }
}
