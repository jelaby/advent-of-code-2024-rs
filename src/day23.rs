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

//     a
//     |
//     b = c = d = e = f
//     |   |
//     g --'
//
//  a -> {b}
//     b -> {a/g/c/d/e/f} -- no mutual connections -- group is just a,b
//  b -> {a/c/d/e/f/g}
//     a -> {b} -- no mutual connections -- group is just a,b
//     c -> {b/g/d/e/f} -- b already in group
//        g -> {b/c} -- only already matched members -- group b,c,g
//     d -> {b/c/e/f} -- matches already matched members -- matches more
//        e -> {b/c/d/f} -- matches all already matched members -- plus more
//           f -> matches all already matched members
//
//fn find_mutual_groups<'a>(
//    computer_groups: &HashMap<&'a str, HashSet<&str>>,
//    potential_members: &HashSet<&'a str>,
//    confirmed_members: &HashSet<&'a str>,
//) -> Vec<Vec<&'a str>> {
//    let result = vec![];
//
//    for other in potential_members {
//        let other_group = computer_groups.get(other).unwrap();
//
//        let mutual_group = potential_members.intersection(other_group);
//
//        if mutual_group.count() == 0 {}
//    }
//
//    result
//}

fn union<'a>(item: &'a str, group: &HashSet<&'a str>) -> HashSet<&'a str>
{
    let mut result: HashSet<&'a str> = HashSet::new();
    result.insert(item);
    group.iter().for_each(|i| { result.insert(i); });
    result
}

fn intersection<'a>(a: &HashSet<&'a str>, b: &HashSet<&'a str>) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    a.iter().filter(|&i| b.contains(i)).for_each(|i| { result.insert(*i); });
    result
}

fn confirm_group(computer_groups: &HashMap<&str, HashSet<&str>>, group: &HashSet<&str>) -> bool {
    for other in group {
        let other_group = computer_groups.get(&other[..]).unwrap();

        for i in group {
            // other_group must be a superset of this group
            if !other_group.contains(i) {
                return false;
            }
        }
    }

    return true;
}

fn find_mutual_sets<'a>(
    computer_groups: &HashMap<&'a str, HashSet<&'a str>>,
) -> HashSet<Vec<&'a str>> {
    let mut result = HashSet::new();

    for (this, group) in computer_groups {
        let group = union(this, &group);

        for other in &group {
            let other_group = union(other, computer_groups.get(other).unwrap());
            let mutual_group:HashSet<&str> = intersection(&group, &other_group);

            if confirm_group(computer_groups, &mutual_group) {
                result.insert(mutual_group.iter().map(|i| *i).collect());
            }
        }
    }

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
        let groups = parse(input);

        let sets = find_mutual_sets(&groups);

        let biggest_group = sets
            .iter()
            .sorted_by(|a, b| b.len().cmp(&a.len()))
            .next()
            .unwrap();

        Some(biggest_group.iter().sorted_by(|a, b| a.cmp(b)).join(","))
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
