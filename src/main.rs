
use rusqlite::Connection;
use anyhow::Result;

extern crate core;

mod domain;
mod utils;
mod database;
mod year2015;
mod year2021;
mod year2022;

use crate::utils::{PuzzlePart};
use argh::FromArgs;
use chrono::{Datelike, NaiveDate};
use paris::{error, info, success, warn};
use std::time::Instant;
use strum::IntoEnumIterator;
use crate::database::{get_correct_solution, get_mission, save};
use crate::utils::solution::Solution;

fn default_day() -> u32 {
    chrono::offset::Local::now().day()
}

fn default_year() -> i32 {
    chrono::offset::Local::now().year()
}

#[derive(FromArgs)]
/// Reach new heights.
struct InitialArgs {
    #[argh(option, default = "default_day()", description = "which day to solve")]
    day: u32,
    #[argh(option, default = "default_year()", description = "which year to solve")]
    // year
    year: i32,
}

fn main() -> Result<()> {
    let args: InitialArgs = argh::from_env();
    let day = args.day;
    let month = 12;
    let year = args.year;

    let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    let conn = Connection::open("database.sqlite")?;

    println!();
    info!("<green><u>⭐️Advent of Code {year} ⭐️</>");
    info!("Solving day {:?}\n", day);

    let (first, second) = match year {
        2015 => year2015::get_solvers(day),
        2021 => year2021::get_solvers(day),
        2022 => year2022::get_solvers(day),
        _ => {
            warn!("Year {year} is not supported");
            std::process::exit(1);
        }
    };

    let test_input = format!("src/year{}/day{:02}/test.txt", year, day);
    let real_input = format!("src/year{}/day{:02}/real.txt", year, day);

    for part in PuzzlePart::iter() {
        info!("<u>{part}</>");
        let start = Instant::now();

        let result = match part {
            PuzzlePart::FirstTest => first(&test_input),
            PuzzlePart::FirstReal => first(&real_input),
            PuzzlePart::SecondTest => second(&test_input),
            PuzzlePart::SecondReal => second(&real_input),
        }?;

        let duration = start.elapsed();
        info!("Duration: {:>12?}", duration);
        info!("Answer: {:>12}", result);

        let solution = if let Some(correct) = get_correct_solution(&conn, date, &part) {
            if correct.result != result {
                error!("The solution was not correct. Try again...\n");
                std::process::exit(1);
            }

            let diff = correct.duration.as_micros() as f64 - duration.as_micros() as f64;
            let gain = 100_f64 * diff / (correct.duration.as_micros() as f64);

            if gain >= 0_f64 {
                info!("New solution was <green>{:>.1}%</> faster\n", gain)
            } else {
                info!("New solution was <red>{:>.1}%</> slower\n", gain)
            }

            Solution { date, part, result, is_correct: true, duration }
        } else {
            println!("Is this the correct solution?");
            let mut is_correct_solution = String::new();
            std::io::stdin().read_line(&mut is_correct_solution).unwrap();

            if is_correct_solution.trim() != "y" {
                error!("The solution was not correct. Try again...\n");
                Solution { date, part, result, is_correct: false, duration }
            } else {
                info!("<green>You found the correct solution!\n");
                Solution { date, part, result, is_correct: true, duration }
            }
        };

        save(&conn, &solution).expect("Failed to save to database");

        if !solution.is_correct {
            error!("Incorrect solution. Try again");
            std::process::exit(1);
        }
    }

    let mission = get_mission(&conn, year);
    success!("<green><u>Hurray, you are one day closer to {} 🎉\n", mission);
    std::process::exit(0);
}
