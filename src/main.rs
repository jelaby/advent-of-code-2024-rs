extern crate core;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod days;

use crate::days::AoCError;
use chrono;
use chrono::NaiveDate;
use reqwest;
use std::fs;
use std::time::Instant;

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
    F: Fn(&str) -> Option<String>,
{
    if let Some((result, duration)) = get_input(day, part).ok().and_then(|content| {
        let start = Instant::now();
        run(content.trim()).map(|r| (r, Instant::now().duration_since(start)))
    }) {
        print!("\tPart {part}:\t{result}\t({duration:?}");
    }
}

fn main() {
    let days: Vec<Box<&dyn days::Day>> = vec![
        Box::new(&day25::Day),
        Box::new(&day24::Day),
        Box::new(&day23::Day),
        Box::new(&day22::Day),
        Box::new(&day21::Day),
        Box::new(&day20::Day),
        Box::new(&day19::Day),
        Box::new(&day18::Day),
        Box::new(&day17::Day),
        Box::new(&day16::Day),
        Box::new(&day15::Day),
        Box::new(&day14::Day),
        Box::new(&day13::Day),
        Box::new(&day12::Day),
        Box::new(&day11::Day),
        Box::new(&day10::Day),
        Box::new(&day9::Day),
        Box::new(&day8::Day),
        Box::new(&day7::Day),
        Box::new(&day6::Day),
        Box::new(&day5::Day),
        Box::new(&day4::Day),
        Box::new(&day3::Day),
        Box::new(&day2::Day),
        Box::new(&day1::Day),
    ];

    for day in days {
        let number = day.day();

        print!("Day {number}:");
        run_part(number, 2, |l| day.part2(l).map(|r| r.to_string()));
        run_part(number, 1, |l| day.part1(l).map(|r| r.to_string()));
        println!();
    }
}
