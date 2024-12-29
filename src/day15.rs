use crate::day15::Block::*;
use crate::day15::Dir::*;
use crate::days;
use nalgebra::Vector2;

pub struct Day;

impl Day {}

#[derive(Copy, Clone, Debug)]
enum Block {
    Space,
    Robot,
    Wall,
    Crate,
    CrateLeft,
    CrateRight,
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

fn parse(input: &str, part2: bool) -> (Vec<Vec<Block>>, Vector2<usize>, Vec<Dir>) {
    let mut parts = input.split_terminator("\n\n");

    let map: Vec<Vec<Block>> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Wall,
                    '.' => Space,
                    'O' => Crate,
                    '@' => Robot,
                    _ => panic!("Unrecognised map char {c}"),
                })
                .collect()
        })
        .collect();

    let map = if part2 { to_part2(&map) } else { map };

    let robot = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find(|(_, &block)| matches!(block, Robot))
                .map(|(x, _)| Vector2::new(x, y))
        })
        .unwrap();

    let commands = parts
        .next()
        .unwrap()
        .lines()
        .flat_map(|line| line.chars())
        .map(|c| match c {
            '<' => Left,
            '>' => Right,
            '^' => Up,
            'v' => Down,
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
                    Space => [Space, Space].into_iter(),
                    Robot => [Robot, Space].into_iter(),
                    Wall => [Wall, Wall].into_iter(),
                    Crate => [CrateLeft, CrateRight].into_iter(),
                    _ => panic!("Unexpected {block:?} while converting to part 2"),
                })
                .collect()
        })
        .collect()
}

fn vec_for(dir: Dir) -> Vector2<i64> {
    match dir {
        Left => Vector2::new(-1, 0),
        Right => Vector2::new(1, 0),
        Up => Vector2::new(0, -1),
        Down => Vector2::new(0, 1),
    }
}

fn can_move_block(map: &mut Vec<Vec<Block>>, p: Vector2<usize>, v: Vector2<i64>) -> bool {
    let n = (p.cast() + v).map(|i| i as usize);
    let r = Vector2::new(1, 0);

    match map[p.y][p.x] {
        Space => true,
        Wall => false,
        Crate => can_move_block(map, n, v),
        CrateLeft => {
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
        CrateRight => can_move_block(map, p - r, v),
        Robot => can_move_block(map, n, v),
    }
}

fn move_block(map: &mut Vec<Vec<Block>>, p: Vector2<usize>, v: Vector2<i64>) -> bool {
    let n = (p.cast() + v).map(|i| i as usize);
    let r = Vector2::new(1, 0);

    if !can_move_block(map, p, v) {
        return false;
    }

    match map[p.y][p.x] {
        Space => {
        }
        Wall => panic!("Tried to move wall at {p:?}+{v:?} => {n:?}"),
        Crate => {
            move_block(map, n, v);
            map[n.y][n.x] = map[p.y][p.x];
            map[p.y][p.x] = Space;
        }
        CrateLeft => {
            if v.x == 1 {
                move_block(map, n + r, v);
                map[n.y][n.x + 1] = map[n.y][n.x];
                map[n.y][n.x] = map[p.y][p.x];
                map[p.y][p.x] = Space;
            } else if v.x == -1 {
                move_block(map, n, v);
                map[n.y][n.x] = map[p.y][p.x];
                map[p.y][p.x] = map[p.y][p.x + 1];
                map[p.y][p.x + 1] = Space;
            } else {
                move_block(map, n + r, v);
                move_block(map, n, v);
                map[n.y][n.x] = map[p.y][p.x];
                map[n.y][n.x + 1] = map[p.y][p.x + 1];
                map[p.y][p.x] = Space;
                map[p.y][p.x + 1] = Space;
            }
        }
        CrateRight => {
            move_block(map, p - r, v);
        }
        Robot => {
            move_block(map, n, v);
            map[n.y][n.x] = map[p.y][p.x];
            map[p.y][p.x] = Space;
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
                .map(move |(x, c)| if matches!(c, Crate) || matches!(c, CrateLeft) { y * 100 + x } else { 0 })
        })
        .sum()
}

fn solve(input: &str, part2: bool) -> Option<String> {
    let (mut map, mut pos, commands) = parse(input, part2);

    for command in commands {
        let dir = vec_for(command);

        if move_block(&mut map, pos, dir) {
            pos = (pos.cast() + dir).map(|i| i as usize);
        }
    }

    Some(score(&map) as i64).map(|r| r.to_string())
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        15
    }

    fn part1(&self, input: &str) -> Option<String> {
        solve(input, false)
    }
    fn part2(&self, input: &str) -> Option<String> {
        solve(input, true)
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
