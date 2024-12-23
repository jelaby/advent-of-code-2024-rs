use crate::days;
use nalgebra::Vector2;
use num::abs;
use std::cmp::min;

pub struct Day;

impl Day {}

type Vec2 = Vector2<i64>;

const NUMERIC: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];
const ARROWS: [[char; 3]; 2] = [[' ', '^', 'A'], ['<', 'v', '>']];

fn locate<const W: usize, const H: usize>(keypad: &[[char; W]; H], c: char) -> Vec2 {
    for y in 0..keypad.len() {
        for x in 0..keypad[y].len() {
            if keypad[y][x] == c {
                return Vec2::new(x as i64, y as i64);
            }
        }
    }
    panic!("Could not locate {c} on {keypad:?}")
}

fn moves_for_keypress<F, const W: usize, const H: usize>(
    keypad: &[[char; W]; H],
    start: char,
    end: char,
    next_moves_for_keypresses: &F,
) -> i64
where
    F: Fn(&str) -> i64,
{
    let start = locate(keypad, start);
    let end = locate(keypad, end);

    let mut results = Vec::with_capacity(2);

    fn single_axis_move_to<F>(start: Vec2, end: Vec2, next_moves_for_keypresses: &F) -> String
    where
        F: Fn(&str) -> i64,
    {
        let move_required = if start.x < end.x {
            ">"
        } else if start.x > end.x {
            "<"
        } else if start.y < end.y {
            "v"
        } else if start.y > end.y {
            "^"
        } else {
            return "".to_string();
        };
        let press_count = abs(end.x - start.x) + abs(end.y - start.y);

        move_required.repeat(press_count as usize)
    }

    // up/down first
    let corner = Vec2::new(start.x, end.y);
    if keypad[corner.y as usize][corner.x as usize] != ' ' {
        results.push(next_moves_for_keypresses(&format!("{}{}A",
                                                        single_axis_move_to(start, corner, next_moves_for_keypresses),
                                                        single_axis_move_to(corner, end, next_moves_for_keypresses))));
    }
    // left/right first
    let corner = Vec2::new(end.x, start.y);
    if keypad[corner.y as usize][corner.x as usize] != ' ' {
        results.push(next_moves_for_keypresses(&format!("{}{}A",
                                                        single_axis_move_to(start, corner, next_moves_for_keypresses),
                                                        single_axis_move_to(corner, end, next_moves_for_keypresses))));
    }

    *results.iter().min().unwrap()
}

fn moves_for_keypresses<F, const W: usize, const H: usize>(
    keypad: &[[char; W]; H],
    sequence: &str,
    next_moves_for_keypresses: &F,
) -> i64
where
    F: Fn(&str) -> i64,
{
    let mut result = 0;
    let mut prev_pos = 'A';

    for c in sequence.chars() {
        result += moves_for_keypress(keypad, prev_pos, c, next_moves_for_keypresses);

        prev_pos = c;
    }

    result
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        21
    }

    fn part1(&self, input: &str) -> Option<String> {
        Some(
            input
                .lines()
                .map(|line| {
                    let keypresses = moves_for_keypresses(&NUMERIC, line, &|sequence| {
                        moves_for_keypresses(&ARROWS, sequence, &|sequence| {
                            moves_for_keypresses(&ARROWS, sequence, &|sequence: &str| {
                                sequence.len() as i64
                            })
                        })
                    });

                    let numeric = line[0..min(3, line.len())].parse::<i64>().unwrap();

                    keypresses * numeric
                })
                .sum::<i64>(),
        )
            .map(|r| r.to_string())
    }
    fn part2(&self, input: &str) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::Day;

    const DAY: super::Day = super::Day;
    #[test]
    fn moves_for_keypress_A() {
        assert_eq!(moves_for_keypress(&ARROWS, 'A', 'A', &|s: &str| s.len() as i64), 1);
        assert_eq!(moves_for_keypress(&ARROWS, 'A', 'A', &|sequence|
            moves_for_keypresses(&ARROWS, sequence, &|sequence: &str| {
                sequence.len() as i64
            })), 1);
    }
    #[test]
    fn moves_for_keypress_up() {
        assert_eq!(moves_for_keypress(&ARROWS, 'A', '^', &|s: &str| s.len() as i64), 2);
        assert_eq!(moves_for_keypress(&ARROWS, 'A', '^', &|sequence|
            moves_for_keypresses(&ARROWS, sequence, &|sequence: &str| {
                sequence.len() as i64
            })), 8);
    }
    #[test]
    fn part1_one_char_3() {
        let text = "3";

        // ^A
        // <A >A
        // v<<A>>^A vA^A

        assert_eq!(DAY.part1(text), Some((12 * 3).to_string()))
    }
    #[test]
    fn part1_example_a() {
        let text = "029A";
        assert_eq!(DAY.part1(text), Some((68 * 29).to_string()))
    }
    #[test]
    fn part1_example_b() {
        let text = "980A";
        assert_eq!(DAY.part1(text), Some((60 * 980).to_string()))
    }
    #[test]
    fn part1_example_c() {
        let text = "179A";
        assert_eq!(DAY.part1(text), Some((68 * 179).to_string()))
    }
    #[test]
    fn part1_example_d() {
        let text = "456A";
        assert_eq!(DAY.part1(text), Some((64 * 456).to_string()))
    }
    #[test]
    fn part1_example_e() {
        let text = "379A";
        assert_eq!(DAY.part1(text), Some((64 * 379).to_string()))
    }
    #[test]
    fn part1_example1() {
        let text = "029A
980A
179A
456A
379A";
        assert_eq!(DAY.part1(text), Some(126384.to_string()))
    }
    #[test]
    fn part2_example1() {
        let text = "029A
980A
179A
456A
379A";
        assert_eq!(DAY.part2(text), Some("".to_string()))
    }
}
