mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod domain;
mod solution;
mod utils;

use crate::solution::Solution;
use crate::utils::PuzzlePart;
use argh::FromArgs;
use chrono::Datelike;
use paris::info;
use strum::IntoEnumIterator;

pub fn hello_to_binary() -> String {
    String::from("Hello")
}

fn default_day() -> u32 {
    chrono::offset::Local::now().day()
}

type SolverFn = fn(&String) -> String;

#[derive(FromArgs)]
/// Reach new heights.
struct InitialArgs {
    #[argh(option, default = "default_day()")]
    /// which day to solve
    day: u32,
}

fn main() -> Result<(), std::io::Error> {
    let args: InitialArgs = argh::from_env();
    let day = args.day;

    println!();
    info!("<green><u>⭐ ️Advent of Code 2021 ⭐️");
    info!("Solving day {:?}", day);
    let mut solution = Solution::load_or_create(day);

    let (first, second): (SolverFn, SolverFn) = match day {
        1 => (day01::solve_1, day01::solve_2),
        2 => (day02::solve_1, day02::solve_2),
        3 => (day03::solve_1, day03::solve_2),
        4 => (day04::solve_1, day04::solve_2),
        5 => (day05::solve_1, day05::solve_2),
        6 => (day06::solve_1, day06::solve_2),
        7 => (day07::solve_1, day07::solve_2),
        8 => (day08::solve_1, day08::solve_2),
        9 => (day09::solve_1, day09::solve_2),
        10 => (day10::solve_1, day10::solve_2),
        11 => (day11::solve_1, day11::solve_2),
        12 => (day12::solve_1, day12::solve_2),
        13 => (day13::solve_1, day13::solve_2),
        _ => {
            println!("Did not find any matching days");
            std::process::exit(1);
        }
    };

    let test_input = format!("src/input{:02}_test.txt", day);
    let real_input = format!("src/input{:02}.txt", day);

    for part in PuzzlePart::iter() {
        let result = match part {
            PuzzlePart::FirstTest => first(&test_input),
            PuzzlePart::FirstReal => first(&real_input),
            PuzzlePart::SecondTest => second(&test_input),
            PuzzlePart::SecondReal => second(&real_input),
        };

        let is_solved = solution.verify_or_update(part, result);
        if !is_solved {
            std::process::exit(1);
        }
        solution.save();
    }

    info!("<green><u>Hurray, you are one day closer to finding the sleigh keys 🎉\n");
    std::process::exit(0);
}
