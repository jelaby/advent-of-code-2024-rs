use itertools::Itertools;
use regex::Regex;
use crate::days;

pub struct Day;

impl Day {}

fn parse<'a>(input: &'a str) -> (Regex, Vec<&'a str>) {
    let mut lines = input.lines();

    let towels = lines.next().unwrap();
    let towels = Regex::new(&format!("^({})+$", towels.split(", ").join("|"))).unwrap();

    let _ = lines.next();

    let designs = lines.collect();

    (towels, designs)
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        19
    }

    fn part1(&self, input: &str) -> Option<String> {
        let (towels, designs) = parse(input);

        Some(designs.iter()
            .filter(|design| towels.is_match(design))
            .count()).map(|r| r.to_string())
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
    fn part1_example1() {
        let text = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(DAY.part1(text), Some(6.to_string()))
    }
    #[test]
    fn part2_example1() {
        let text = "";
        assert_eq!(DAY.part2(text), Some("4".to_string()))
    }
}
