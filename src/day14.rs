use crate::days;
use nalgebra::{DMatrix, Dyn, OMatrix, Vector2};
use regex::Regex;
use std::sync::LazyLock;

pub struct Day;

impl Day {}

#[derive(Debug, PartialEq, Eq)]
struct Robot {
    p: Vector2<i64>,
    v: Vector2<i64>,
}

static LINE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"p=(?<px>[0-9-]+),(?<py>[0-9-]+) v=(?<vx>[0-9-]+),(?<vy>[0-9-]+)").unwrap()
});
fn parse_line(line: &str) -> Robot {
    let captures = LINE_PATTERN.captures(line).unwrap();
    Robot {
        p: Vector2::new(
            captures["px"].parse().unwrap(),
            captures["py"].parse().unwrap(),
        ),
        v: Vector2::new(
            captures["vx"].parse().unwrap(),
            captures["vy"].parse().unwrap(),
        ),
    }
}

fn parse(input: &str) -> Vec<Robot> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn go(robots: &Vec<Robot>, size: Vector2<i64>, time: i64) -> Vec<Robot> {
    robots
        .iter()
        .map(|robot| Robot {
            p: Vector2::new(
                (robot.p.x + robot.v.x * time).rem_euclid(size.x),
                (robot.p.y + robot.v.y * time).rem_euclid(size.y),
            ),
            v: robot.v,
        })
        .collect()
}

fn quadrant(robot: &Robot, size: Vector2<i64>) -> Option<usize> {
    let border = size / 2;
    let p = robot.p;

    if p.x != border.x && p.y != border.y {
        let quadrant = if p.x > border.x { 1 } else { 0 } + if p.y > border.y { 2 } else { 0 };
        Some(quadrant)
    } else {
        None
    }
}

fn safety_factor(robots: &Vec<Robot>, size: Vector2<i64>) -> i64 {
    let mut quadrants = vec![0; 4];

    robots.iter().for_each(|robot| {
        if let Some(quadrant) = quadrant(robot, size) {
            quadrants[quadrant] += 1;
        }
    });

    quadrants.iter().product()
}

fn do_part1(input: &str, sizex: i64, sizey: i64) -> Option<i64> {
    let size = Vector2::new(sizex, sizey);
    Some(safety_factor(&go(&parse(input), size, 100), size))
}

fn plot_robots(robots: &Vec<Robot>, size: &Vector2<i64>) -> OMatrix<usize, Dyn, Dyn> {
    let mut result = DMatrix::<usize>::zeros(size.x as usize, size.y as usize);

    robots
        .iter()
        .for_each(|r| result[(r.p.x as usize, r.p.y as usize)] += 1);

    result
}

fn all_adjacent(robots: &Vec<Robot>, size: &Vector2<i64>, time: i64) -> bool {
    let adjacent = [(0, -1), (-1, 0), (1, 0), (0, 1)];

    let robots = go(robots, *size, time);

    let map = plot_robots(&robots, size);

    let mut have_neighbours = 0;
    for p in robots.iter().map(|r| r.p) {
        let x = p.x;
        let y = p.y;
        if map[(x as usize, y as usize)] > 1 {
            return false;
        }
        if let Some(_) = adjacent
            .iter()
            .map(|&a| (x + a.0, y + a.1))
            .filter(|&(x, y)| x >= 0 && x < size.x && y >= 0 && y < size.y)
            .find(|&(x, y)| map[(x as usize, y as usize)] != 0)
        {
            have_neighbours += 1;
        }
    }
    have_neighbours > robots.len() / 2
}

fn do_part2(input: &str, sizex: i64, sizey: i64) -> Option<i64> {
    let size = Vector2::new(sizex, sizey);
    let robots = parse(input);

    for time in 0..10000 {
        if all_adjacent(&robots, &size, time) {
            return Some(time);
        }
    }
    None
}

impl days::Day for Day {
    fn day(&self) -> u32 {
        14
    }

    fn part1(&self, input: &str) -> Option<String> {
        do_part1(input, 101, 103).map(|r| r.to_string())
    }
    fn part2(&self, input: &str) -> Option<String> {
        do_part2(input, 101, 103).map(|r| r.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::day14::Robot;
    use crate::day14::{do_part1, go};
    use nalgebra::Vector2;

    #[test]
    fn part1_example1() {
        let text = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(do_part1(text, 11, 7), Some(12))
    }

    #[test]
    fn part1_step0() {
        assert_eq!(
            go(
                &vec![super::Robot {
                    p: Vector2::new(2, 4),
                    v: Vector2::new(2, -3)
                }],
                Vector2::new(11, 7),
                0
            ),
            vec![Robot {
                p: Vector2::new(2, 4),
                v: Vector2::new(2, -3)
            }]
        );
    }
    #[test]
    fn part1_step1() {
        assert_eq!(
            go(
                &vec![super::Robot {
                    p: Vector2::new(2, 4),
                    v: Vector2::new(2, -3)
                }],
                Vector2::new(11, 7),
                1
            ),
            vec![Robot {
                p: Vector2::new(4, 1),
                v: Vector2::new(2, -3)
            }]
        );
    }
    #[test]
    fn part1_step2() {
        assert_eq!(
            go(
                &vec![super::Robot {
                    p: Vector2::new(2, 4),
                    v: Vector2::new(2, -3)
                }],
                Vector2::new(11, 7),
                2
            ),
            vec![Robot {
                p: Vector2::new(6, 5),
                v: Vector2::new(2, -3)
            }]
        );
    }
    #[test]
    fn part1_step3() {
        assert_eq!(
            go(
                &vec![super::Robot {
                    p: Vector2::new(2, 4),
                    v: Vector2::new(2, -3)
                }],
                Vector2::new(11, 7),
                3
            ),
            vec![Robot {
                p: Vector2::new(8, 2),
                v: Vector2::new(2, -3)
            }]
        );
    }
    #[test]
    fn part1_step4() {
        assert_eq!(
            go(
                &vec![super::Robot {
                    p: Vector2::new(2, 4),
                    v: Vector2::new(2, -3)
                }],
                Vector2::new(11, 7),
                4
            ),
            vec![Robot {
                p: Vector2::new(10, 6),
                v: Vector2::new(2, -3)
            }]
        );
    }
    #[test]
    fn part1_step5() {
        assert_eq!(
            go(
                &vec![super::Robot {
                    p: Vector2::new(2, 4),
                    v: Vector2::new(2, -3)
                }],
                Vector2::new(11, 7),
                5
            ),
            vec![Robot {
                p: Vector2::new(1, 3),
                v: Vector2::new(2, -3)
            }]
        );
    }
    #[test]
    fn quadrant_test() {
        fn assert(x: i64, y: i64, expected: Option<usize>) {
            assert_eq!(
                super::quadrant(
                    &Robot {
                        p: Vector2::new(x, y),
                        v: Vector2::new(0, 0)
                    },
                    Vector2::new(11, 7)
                ),
                expected
            );
        }
        assert(0, 0, Some(0));
        assert(1, 0, Some(0));
        assert(2, 0, Some(0));
        assert(3, 0, Some(0));
        assert(4, 0, Some(0));
        assert(5, 0, None);
        assert(6, 0, Some(1));
        assert(7, 0, Some(1));
        assert(8, 0, Some(1));
        assert(9, 0, Some(1));
        assert(10, 0, Some(1));

        assert(0, 0, Some(0));
        assert(0, 1, Some(0));
        assert(0, 2, Some(0));
        assert(0, 3, None);
        assert(0, 4, Some(2));
        assert(0, 5, Some(2));
        assert(0, 6, Some(2));

        assert(0, 0, Some(0));
        assert(5, 3, None);
        assert(10, 6, Some(3));

        assert(4, 2, Some(0));
        assert(5, 2, None);
        assert(6, 2, Some(1));
        assert(6, 3, None);
        assert(6, 4, Some(3));
        assert(5, 4, None);
        assert(4, 4, Some(2));
        assert(4, 3, None);

        assert(0, 0, Some(0));
    }
}
