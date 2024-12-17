use crate::day15::Block::*;
use crate::day15::Dir::*;
use crate::days;
use nalgebra::{Vector2};
use num::{Num, NumCast};

pub struct Day;

impl Day {}

#[derive(Copy, Clone, Debug)]
enum Block {
    SPACE,
    ROBOT,
    WALL,
    CRATE,
    CRATE_L,
    CRATE_R,
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

fn parse(input: &str, part2: bool) -> (Vec<Vec<Block>>, Vector2<usize>, Vec<Dir>) {
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

    let mut map = if part2 { to_part2(&map) } else { map };

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

fn to_part2(map: &Vec<Vec<Block>>) -> Vec<Vec<Block>> {
    map.iter()
        .map(|row| {
            row.iter()
                .flat_map(|block| match block {
                    SPACE => [SPACE, SPACE].into_iter(),
                    ROBOT => [ROBOT, SPACE].into_iter(),
                    WALL => [WALL, WALL].into_iter(),
                    CRATE => [CRATE_L, CRATE_R].into_iter(),
                    _ => panic!("Unexpected {block:?} while converting to part 2"),
                })
                .collect()
        })
        .collect()
}

fn vec_for(dir: Dir) -> Vector2<i64> {
    match dir {
        LEFT => Vector2::new(-1, 0),
        RIGHT => Vector2::new(1, 0),
        UP => Vector2::new(0, -1),
        DOWN => Vector2::new(0, 1),
    }
}

fn can_move_block(map: &mut Vec<Vec<Block>>, p: Vector2<usize>, v: Vector2<i64>) -> bool {
    let n = (p.cast() + v).map(|i| i as usize);
    let r = Vector2::new(1, 0);

    match map[p.y][p.x] {
        SPACE => true,
        WALL => false,
        CRATE => can_move_block(map, n, v),
        CRATE_L => {
            if v.y == 0 {
                if v.x == 1 {
                    can_move_block(map, n + r, v)
                } else {
                    can_move_block(map, n, v)
                }
            } else {
                can_move_block(map, n, v) && can_move_block(map, n + r, v)
            }
        }
        CRATE_R => can_move_block(map, p - r, v),
        ROBOT => can_move_block(map, n, v),
    }
}

fn move_block(map: &mut Vec<Vec<Block>>, p: Vector2<usize>, v: Vector2<i64>) -> bool {
    let n = (p.cast() + v).map(|i| i as usize);
    let r = Vector2::new(1, 0);

    if !can_move_block(map, p, v) {
        return false;
    }

    match map[p.y][p.x] {
        SPACE => {
        }
        WALL => panic!("Tried to move wall at {p:?}+{v:?} => {n:?}"),
        CRATE => {
            move_block(map, n, v);
            map[n.y][n.x] = map[p.y][p.x];
            map[p.y][p.x] = SPACE;
        }
        CRATE_L => {
            if v.x == 1 {
                move_block(map, n + r, v);
                map[n.y][n.x + 1] = map[n.y][n.x];
                map[n.y][n.x] = map[p.y][p.x];
                map[p.y][p.x] = SPACE;
            } else if v.x == -1 {
                move_block(map, n, v);
                map[n.y][n.x] = map[p.y][p.x];
                map[p.y][p.x] = map[p.y][p.x + 1];
                map[p.y][p.x + 1] = SPACE;
            } else {
                move_block(map, n + r, v);
                move_block(map, n, v);
                map[n.y][n.x] = map[p.y][p.x];
                map[n.y][n.x + 1] = map[p.y][p.x + 1];
                map[p.y][p.x] = SPACE;
                map[p.y][p.x + 1] = SPACE;
            }
        }
        CRATE_R => {
            move_block(map, p - r, v);
        }
        ROBOT => {
            move_block(map, n, v);
            map[n.y][n.x] = map[p.y][p.x];
            map[p.y][p.x] = SPACE;
        }
    }

    true
}

fn score(map: &Vec<Vec<Block>>) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, c)| if matches!(c, CRATE) || matches!(c, CRATE_L) { y * 100 + x } else { 0 })
        })
        .sum()
}

fn show_map(map: &Vec<Vec<Block>>, robot: Vector2<usize>) {
    println!();
    for (y,row) in map.iter().enumerate() {
        for (x,block) in row.iter().enumerate() {
            print!(
                "{}",
                if x == robot.x && y == robot.y {
                    '@'
                } else {
                    match block {
                        SPACE => ' ',
                        ROBOT => '*',
                        CRATE => 'O',
                        WALL => '#',
                        CRATE_L => '[',
                        CRATE_R => ']',
                    }
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

    fn part1(&self, input: &str) -> Option<String> {
        let (mut map, mut pos, commands) = parse(input, false);

        for command in commands {
            let dir = vec_for(command);

            if move_block(&mut map, pos, dir) {
                pos = (pos.cast() + dir).map(|i| i as usize);
            }
        }

        Some(score(&map) as i64).map(|r| r.to_string())
    }
    fn part2(&self, input: &str) -> Option<String> {
        let (mut map, mut pos, commands) = parse(input, true);

        for command in commands {
            let dir = vec_for(command);

            if move_block(&mut map, pos, dir) {
                pos = (pos.cast() + dir).map(|i| i as usize);
            }
        }

        Some(score(&map) as i64).map(|r| r.to_string())
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
        assert_eq!(DAY.part1(text), Some("10092".to_string()))
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
        assert_eq!(DAY.part1(text), Some("2028".to_string()))
    }
    #[test]
    fn part2_example1() {
        let text = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        assert_eq!(DAY.part2(text), Some((105 + 207 + 306).to_string()))
    }
    #[test]
    fn part2_example2() {
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
        assert_eq!(DAY.part2(text), Some("9021".to_string()))
    }
}
