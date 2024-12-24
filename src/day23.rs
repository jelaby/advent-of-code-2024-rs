use crate::days;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct Day;

impl Day {}

fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut result = HashMap::new();
    input
        .lines()
        .map(|line| line.split('-'))
        .map(|mut pair| (pair.next().unwrap(), pair.next().unwrap()))
        .for_each(|(l, r)| {
            result
                .entry(l)
                .and_modify(|e: &mut HashSet<&str>| {
                    e.insert(r);
                })
                .or_insert_with(|| HashSet::from_iter([r]));
            result
                .entry(r)
                .and_modify(|e: &mut HashSet<&str>| {
                    e.insert(l);
                })
                .or_insert_with(|| HashSet::from_iter([l]));
        });

    result
}

fn intersection<'a>(a: &HashSet<&'a str>, b: &HashSet<&'a str>) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    a.iter().filter(|&i| b.contains(i)).for_each(|i| {
        result.insert(*i);
    });
    result
}

fn next_largest_groups<'a,'b>(
    connections: &HashMap<&'a str, HashSet<&'a str>>,
    group: &'b Vec<&'a str>,
) -> HashSet<Vec<&'a str>> {
    {
        let mut result = HashSet::new();
        let mut candidates = connections.get(group[0]).unwrap().clone();
        for &t in &group[1..] {
            candidates = intersection(connections.get(t).unwrap(), &candidates);
        }

        for candidate in candidates {
            let mut new_set = Vec::new();
            for g in group {
                new_set.push(*g)
            }
            new_set.push(candidate);
            new_set.sort();

            result.insert(new_set);
        }
        result
    }
}

fn all_next_largest_groups<'a,'b>(
    connections: &HashMap<&'a str, HashSet<&'a str>>,
    groups: &'b HashSet<Vec<&'a str>>,
) -> HashSet<Vec<&'a str>> {
    let mut result = HashSet::new();
    for group in groups {
        for g in next_largest_groups(connections, group) {
            result.insert(g);
        }
    }
    result
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        23
    }

    fn part1(&self, input: &str) -> Option<String> {
        let connections = parse(input);

        let mut triples = HashSet::new();

        for (this, group) in &connections {
            for other in group {
                let other_group = connections.get(other).unwrap();

                for third in group.intersection(other_group) {
                    let mut set = vec![this, other, third];
                    set.sort();
                    triples.insert(set);
                }
            }
        }

        Some(
            triples
                .iter()
                .filter(|i| {
                    i.iter()
                        .map(|c| c.chars().next().unwrap())
                        .any(|c| c == 't')
                })
                .count(),
        )
        .map(|r| r.to_string())
    }
    fn part2(&self, input: &str) -> Option<String> {
        let connections = parse(input);

        let mut largest_groups = HashSet::new();

        for (computer, _) in &connections {
            largest_groups.insert(vec![*computer]);
        }

        loop {
            let next_larger_groups = all_next_largest_groups(&connections, &largest_groups);
            if next_larger_groups.is_empty() {
                break;
            } else {
                largest_groups = next_larger_groups;
            }
        }

        assert_eq!(largest_groups.len(), 1);
        let largest_set = largest_groups.iter().next().unwrap();

        Some(largest_set.join(","))
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;

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
        assert_eq!(DAY.part2(text), Some("co,de,ka,ta".to_string()))
    }
}
