use std::collections::{HashMap, HashSet};
use crate::days;

pub struct Day;

impl Day {}

fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut result = HashMap::new();
    input.lines()
        .map(|line| line.split('-')).
        map(|mut pair| (pair.next().unwrap(), pair.next().unwrap()))
        .for_each(|(l,r)| {
            result.entry(l)
                .and_modify(|e: &mut HashSet<&str>| { e.insert(r); })
                .or_insert_with(|| HashSet::from_iter([r]));
            result.entry(r)
                .and_modify(|e: &mut HashSet<&str>| { e.insert(l); })
                .or_insert_with(|| HashSet::from_iter([l]));
        });

    result
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        23
    }

    fn part1(&self, input: &str) -> Option<String> {

        let groups = parse(input);

        let mut triples = HashSet::new();

        for (this, group) in &groups {

            for other in group {

                let other_group = groups.get(other).unwrap();

                for third in group.intersection(other_group) {
                    let mut set = vec![this,other,third];
                    set.sort();
                    triples.insert(set);
                }

            }

        }

        Some(triples.iter()
            .filter(|i| i.iter()
                .map(|c| c.chars().next().unwrap())
                .any(|c| c == 't'))
            .count())
            .map(|r| r.to_string())
    }
    fn part2(&self, input: &str) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;
    use super::*;

    const DAY: super::Day = super::Day;
    #[test]
    fn part1_example1() {
        let text = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        assert_eq!(DAY.part1(text), Some(7.to_string()))
    }
    #[test]
    fn part2_example1() {
        let text = "";
        assert_eq!(DAY.part2(text), Some("4".to_string()))
    }
}
