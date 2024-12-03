use crate::days;

pub struct Day;

impl days::Day for Day {
    fn day(&self) -> u32 {
        1
    }

    fn part1(&self, input: &str) -> Option<i64> {
        let mut left = vec![];
        let mut right = vec![];

        for line in input.split_terminator("\n") {
            let mut parts = line.split_whitespace();
            left.push(parts.next().unwrap().parse::<i64>().unwrap());
            right.push(parts.next().unwrap().parse::<i64>().unwrap());
        }

        left.sort();
        right.sort();

        let mut result = 0;
        for pair in left.iter().zip(right.iter()) {
            result += (pair.0 - pair.1).abs();
        }

        Some(result)
    }
    fn part2(&self, input: &str) -> Option<i64> {
        let mut left = vec![];
        let mut right = vec![];

        for line in input.split("\n") {
            let mut parts = line.split_whitespace();
            left.push(parts.next().unwrap().parse::<i64>().unwrap());
            right.push(parts.next().unwrap().parse::<i64>().unwrap());
        }

        left.sort();
        right.sort();

        let mut result = 0;

        for l in left {
            let i = right.partition_point(|&r| r < l);
            let j = right.partition_point(|&r| r <= l);
            result += l * (j - i) as i64;
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;

    const DAY: crate::day1::Day = crate::day1::Day;
    #[test]
    fn part1_example1() {
        let text = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(DAY.part1(text), Some(11))
    }
    #[test]
    fn part2_example1() {
        let text = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(DAY.part2(text), Some(31))
    }
}
