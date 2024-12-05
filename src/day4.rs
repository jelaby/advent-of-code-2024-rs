use crate::days;

pub struct Day;

impl Day {}

impl days::Day for Day {
    fn day(&self) -> u32 {
        4
    }

    fn part1(&self, input: &str) -> Option<i64> {
        let puzzle = {
            let mut puzzle = vec![];

            for line in input.split_terminator("\n") {
                let mut chars = vec![];
                for c in line.chars() {
                    chars.push(c);
                }
                puzzle.push(chars);
            }
            puzzle
        };

        const DIRECTIONS: [(i32, i32); 8] = [
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];
        const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

        fn is_xmas(puzzle: &Vec<Vec<char>>, pos: (usize, usize), dir: (i32, i32)) -> bool {
            if puzzle[pos.1][pos.0] != 'X' {
                return false;
            }
            let ((x, y), (dx, dy)) = (pos, dir);
            let mut x = x as i32;
            let mut y = y as i32;
            for c in XMAS {
                if x < 0 || y < 0 || puzzle
                    .get(y as usize)
                    .and_then(|l| l.get(x as usize))
                    .map(|&puzzle_c| puzzle_c != c)
                    .unwrap_or(true)
                {
                    return false;
                }
                x += dx;
                y += dy;
            }

            return true;
        }

        fn count_xmas(puzzle: &Vec<Vec<char>>, pos: (usize, usize)) -> i64 {
            let mut count = 0;
            for dir in DIRECTIONS {
                if is_xmas(puzzle, pos, dir) {
                    count += 1;
                }
            }
            count
        }

        let mut result = 0;
        for y in 0..puzzle.len() {
            for x in 0..puzzle[y].len() {
                result += count_xmas(&puzzle, (x, y));
            }
        }

        Some(result)
    }
    fn part2(&self, input: &str) -> Option<i64> {
        let puzzle = {
            let mut puzzle = vec![];

            for line in input.split_terminator("\n") {
                let mut chars = vec![];
                for c in line.chars() {
                    chars.push(c);
                }
                puzzle.push(chars);
            }
            puzzle
        };

        fn is_xmas(puzzle: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
            let (x,y) = pos;
            const MS:[char;2] = ['M','S'];

            puzzle[y][x] == 'A'
                && MS.contains(&puzzle[y-1][x-1]) && MS.contains(&puzzle[y+1][x+1])
                && puzzle[y-1][x-1] != puzzle[y+1][x+1]
                && MS.contains(&puzzle[y-1][x+1]) && MS.contains(&puzzle[y+1][x-1])
                && puzzle[y-1][x+1] != puzzle[y+1][x-1]
        }

        let mut result = 0;
        for y in 1..puzzle.len() - 1 {
            for x in 1..puzzle[y].len() - 1 {
                if is_xmas(&puzzle,(x,y)) {
                    result += 1;
                }
            }
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;

    const DAY: super::Day = super::Day;
    #[test]
    fn part1_example1() {
        let text = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(DAY.part1(text), Some(18))
    }
    #[test]
    fn part2_example1() {
        let text = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(DAY.part2(text), Some(9))
    }
}
