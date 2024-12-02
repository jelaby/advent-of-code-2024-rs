mod day0;
mod day1;
mod day2;
mod days;

use std::fs;
use std::string::ToString;

const NO_VALUE: &str = "-";

fn run_part<F>(number: i32, part: i32, run: F)
where
    F: Fn(&str) -> Option<i64>,
{
    let content = fs::read_to_string(format!("input/day{number}-part{part}.txt"))
        .or_else(|_|  fs::read_to_string(format!("input/day{number}-part1.txt")));
    let result = match content {
        Ok(content) => run(content.trim()),
        Err(_) => None,
    };
    let result = match result {
        Some(x) => &x.to_string(),
        None => NO_VALUE,
    };

    print!("\tPart {part}:\t{result}");
}

fn main() {
    let days: [Box<dyn days::Day>; 3] = [
        Box::new(day2::Day),
        Box::new(day1::Day),
        Box::new(day0::Day),
    ];

    for day in days {
        let number = day.day();

        print!("Day {number}:");
        run_part(number, 2, |l| day.part2(l));
        run_part(number, 1, |l| day.part1(l));
        println!();
    }
}
