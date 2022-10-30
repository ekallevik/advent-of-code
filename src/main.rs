extern crate core;

mod year2021;
mod domain;
mod utils;
mod year2015;

use crate::utils::{PuzzlePart, solution::Solution};
use argh::FromArgs;
use chrono::Datelike;
use paris::info;
use std::time::Instant;
use strum::IntoEnumIterator;

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

fn main() -> Result<(), std::io::Error> {
    let args: InitialArgs = argh::from_env();
    let day = args.day;
    let year = args.year;

    println!();
    info!("<green><u>⭐ ️Advent of Code {year} ⭐️");
    info!("Solving day {:?}", day);

    let (first, second) = match year {
        2015 => year2015::get_solvers(day),
        2021 => year2021::get_solvers(day),
        _ => {
            println!("Year {year} is not supported");
            std::process::exit(1);
        }
    };

    let mut solution = Solution::load_or_create(year, day);

    let test_input = format!("src/year{}/day{:02}/test.txt", year, day);
    let real_input = format!("src/year{}/day{:02}/real.txt", year, day);

    for part in PuzzlePart::iter() {
        let start = Instant::now();

        let result = match part {
            PuzzlePart::FirstTest => first(&test_input),
            PuzzlePart::FirstReal => first(&real_input),
            PuzzlePart::SecondTest => second(&test_input),
            PuzzlePart::SecondReal => second(&real_input),
        };
        println!("\nElapsed: {:#?}", start.elapsed());


        let is_solved = solution.verify_or_update(part, result);
        if !is_solved {
            std::process::exit(1);
        }
        solution.save();



    }

    info!("<green><u>Hurray, you are one day closer to finding the sleigh keys 🎉\n");
    std::process::exit(0);
}
