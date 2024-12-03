extern crate core;

mod day1;
mod day2;
mod day3;
mod days;

use crate::days::AoCError;
use chrono;
use chrono::NaiveDate;
use reqwest;
use std::fs;
use std::string::ToString;

const NO_VALUE: &str = "-";
const EASTERN_STANDARD_TIME: chrono::FixedOffset =
    chrono::FixedOffset::west_opt(4 * 60 * 60).unwrap();

fn input_filename(day: u32, _part: u32) -> String {
    format!("input/day{day}.txt")
}

fn is_in_past(day: u32) -> bool {
    let now = chrono::Local::now();
    let this_day = NaiveDate::from_ymd_opt(2024, 12, day)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(EASTERN_STANDARD_TIME)
        .unwrap();

    now > this_day
}

fn read_cookie() -> String {
    fs::read_to_string(".cookie")
        .map_err(|e| AoCError::CookieFile(e))
        .unwrap()
}

fn get_input(day: u32, part: u32) -> Result<String, AoCError> {
    fs::read_to_string(input_filename(day, part))
        .or_else(|_| fs::read_to_string(input_filename(day, 1)))
        .or_else(|e| {
            if is_in_past(day) {
                reqwest::blocking::Client::new()
                    .get(format!("https://adventofcode.com/2024/day/{day}/input"))
                    .header("Cookie", read_cookie())
                    .send()
                    .and_then(|r| r.text())
                    .inspect(|content| {
                        let _ = fs::create_dir("input");
                        let _ = fs::write(input_filename(day, part), content);
                    })
                    .map_err(AoCError::from)
            } else {
                Err(AoCError::from(e))
            }
        })
}

fn run_part<F>(day: u32, part: u32, run: F)
where
    F: Fn(&str) -> Option<i64>,
{
    let content = get_input(day, part);
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
        Box::new(day3::Day),
        Box::new(day2::Day),
        Box::new(day1::Day),
    ];

    for day in days {
        let number = day.day();

        print!("Day {number}:");
        run_part(number, 2, |l| day.part2(l));
        run_part(number, 1, |l| day.part1(l));
        println!();
    }
}
