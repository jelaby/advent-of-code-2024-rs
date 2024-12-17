use crate::days;
use itertools::Itertools;
use regex::Regex;
use std::sync::LazyLock;
use nalgebra::Vector2;

pub struct Day;

impl Day {}

struct Machine {
    a: Vector2<i64>,
    b: Vector2<i64>,
    prize: Vector2<i64>,
}

fn parse(input: &str) -> Vec<Machine> {
    static PATTERN: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"\w+: X[=+](?<x>\d+), Y[=+](?<y>\d+)").unwrap());

    fn to_vector(line: &str) -> Vector2<i64> {
        let caps = PATTERN.captures(line).unwrap();
        Vector2::new(caps["x"].parse().unwrap(), caps["y"].parse().unwrap())
    }

    input
        .lines()
        .chunks(4)
        .into_iter()
        .map(|mut chunk| Machine {
            a: to_vector(chunk.next().unwrap()),
            b: to_vector(chunk.next().unwrap()),
            prize: to_vector(chunk.next().unwrap()),
        })
        .collect()
}

/*

n.a + o.b = p

n.ax + o.bx = px
n.ay + o.by = py

n = px - o.bx
    ---------
        ax

(px - o.bx)
 ---------  ay + o.by = py
    ax

px.ay - o.bx.ay + o.by = py
-----   -------
  ax       ax

px.ay - o.bx.ay + o.ax.by
-----   -------   ------- = py
  ax      ax          ax

px.ay - o.bx.ay + o.ax.by = py.ax

o (ax.by - bx.ay) = py.ax - px.ay
o = py.ax - px.ay
    -------------
    ax.by - bx.ay
 */

fn find_moves(m: &Machine) -> Option<(i64, i64)> {
    let a = m.a;
    let b = m.b;
    let p = m.prize;

    let rem = (p.y * a.x - p.x * a.y) % (a.x * b.y - b.x * a.y);

    if rem != 0 {
        return None;
    }

    let o = (p.y * a.x - p.x * a.y) / (a.x * b.y - b.x * a.y);

    let rem = (p.x - o * b.x) % a.x;

    if rem != 0 {
        return None;
    }

    let n = (p.x - o * b.x) / a.x;

    Some((n, o))
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        13
    }

    fn part1(&self, input: &str) -> Option<String> {
        let machines = parse(input);

        Some(
            machines
                .iter()
                .filter_map(|m| find_moves(m))
                .map(|(a, b)| a * 3 + b)
                .sum::<i64>(),
        ).map(|r| r.to_string())
    }
    fn part2(&self, input: &str) -> Option<String> {
        let machines = parse(input);

        Some(
            machines
                .iter()
                .map(|m| Machine {
                    a: m.a,
                    b: m.b,
                    prize: Vector2::new(m.prize.x + 10000000000000, m.prize.y + 10000000000000),
                })
                .filter_map(|m| find_moves(&m))
                .map(|(a, b)| a * 3 + b)
                .sum::<i64>(),
        ).map(|r| r.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::days::Day;

    const DAY: super::Day = super::Day;
    #[test]
    fn part1_example1() {
        let text = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(DAY.part1(text), Some("480".to_string()))
    }
    #[test]
    fn part2_example1() {
        let text = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(DAY.part2(text), Some("875318608908".to_string()))
    }
}
