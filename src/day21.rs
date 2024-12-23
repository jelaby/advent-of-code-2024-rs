use crate::days;
use nalgebra::Vector2;
use num::abs;

pub struct Day;

impl Day {}

type Vec2 = Vector2<i64>;

const NUMERIC: [[char; 3]; 4] = [
    ['8', '8', '9'],
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

    // up/down first
    let corner = Vec2::new(start.x, end.y);
    if keypad[corner.y as usize][corner.x as usize] != ' ' {
        let move_required = if end.y > start.y { "v" } else { "^" };
        let press_count = abs(end.y - start.x);

        let mut result = next_moves_for_keypresses(&move_required.repeat(press_count as usize));

        let move_required = if end.x > start.x { ">" } else { "<" };
        let press_count = abs(end.x - start.y);

        result += next_moves_for_keypresses(&move_required.repeat(press_count as usize));

        results.push(result);
    }
    // left/right first
    let corner = Vec2::new(end.x, start.y);
    if keypad[corner.y as usize][corner.x as usize] != ' ' {
        let move_required = if end.x > start.x { ">" } else { "<" };
        let press_count = abs(end.x - start.y);

        let mut result =
            next_moves_for_keypresses(&format!("{}A", move_required.repeat(press_count as usize)));

        let move_required = if end.y > start.y { "v" } else { "^" };
        let press_count = abs(end.y - start.x);

        result +=
            next_moves_for_keypresses(&format!("{}A", move_required.repeat(press_count as usize)));

        results.push(result);
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

                    let numeric = line[0..3].parse::<i64>().unwrap();

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
    use crate::days::Day;

    const DAY: super::Day = super::Day;
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
        assert_eq!(DAY.part1(text), Some((68 * 279).to_string()))
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
