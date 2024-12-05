use std::cmp::Ordering;
use std::collections::HashMap;
use crate::days;

pub struct Day;

impl Day {}

impl days::Day for Day {
    fn day(&self) -> u32 {
        5
    }

    fn part1(&self, input: &str) -> Option<i64> {
        let mut i = input.split("\n\n").map(|part| part.split_terminator('\n'));

        let rules = i.next().unwrap()
            .map(|rule| {
                let mut rule = rule.split('|');
                let first = rule.next().and_then(|n| n.parse::<i64>().ok()).unwrap();
                let second = rule.next().and_then(|n| n.parse::<i64>().ok()).unwrap();

                (first, second)
            })
            .fold(HashMap::new(), |mut map, (first, second)| {
                map.entry(first).or_insert_with(Vec::new).push(second);
                map
            });
        Some(i.next().unwrap()
            .map(|print|
                print.split(",")
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<i64>>())
                 .filter(|print| print.iter()
                    .fold((None, true), |(last, ok), current| {
                        match last {
                            None => (Some(current), ok),
                            Some(last) => {
                                (Some(current), ok && rules.get(&last).map(|followers| followers.contains(&current)).unwrap_or(false))
                            },
                        }
                    }).1)
            .map(|print| print[print.len()/2])
            .sum())
    }
    fn part2(&self, input: &str) -> Option<i64> {
        let mut i = input.split("\n\n").map(|part| part.split_terminator('\n'));

        let rules = i.next().unwrap()
            .map(|rule| {
                let mut rule = rule.split('|');
                let first = rule.next().and_then(|n| n.parse::<i64>().ok()).unwrap();
                let second = rule.next().and_then(|n| n.parse::<i64>().ok()).unwrap();

                (first, second)
            })
            .fold(HashMap::new(), |mut map, (first, second)| {
                map.entry(first).or_insert_with(Vec::new).push(second);
                map
            });

        Some(i.next().unwrap()
            .map(|print|
                print.split(",")
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<i64>>())
            .filter(|print| !print.iter()
                .fold((None, true), |(last, ok), current| {
                    match last {
                        None => (Some(current), ok),
                        Some(last) => {
                            (Some(current), ok && rules.get(&last).map(|followers| followers.contains(&current)).unwrap_or(false))
                        },
                    }
                }).1)
            .map(|print| {
                let mut print = print.to_owned();
                print.sort_by(|a,b| {
                    if rules.get(a).map(|followers| followers.contains(&b)).unwrap_or(false) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
                print
            })
            .map(|print| print[print.len()/2])
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;

    const DAY: super::Day = super::Day;
    #[test]
    fn part1_example1() {
        let text = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(DAY.part1(text), Some(143))
    }
    #[test]
    fn part2_example1() {
        let text = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(DAY.part2(text), Some(123))
    }
}
