mod utils;
mod domain;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

use argh::FromArgs;
use chrono::Datelike;
use paris::{error, info};

fn default_day() -> u32 {
    chrono::offset::Local::now().day()
}

type SolverFn = fn(String) -> String;

#[derive(FromArgs)]
/// Reach new heights.
struct InitialArgs {
    #[argh(option, default = "default_day()")]
    /// which day to solve
    day: u32,
}


fn main() {
    let args: InitialArgs = argh::from_env();

    println!();
    info!("<green><u>⭐ ️Advent of Code 2021 ⭐️");
    info!("Solving day {:?}", args.day);

    let (first, second): (SolverFn, SolverFn) = match args.day {
        1 => (day01::solve_1, day01::solve_2),
        2 => (day02::solve_1, day02::solve_2),
        3 => (day03::solve_1, day03::solve_2),
        4 => (day04::solve_1, day04::solve_2),
        5 => (day05::solve_1, day05::solve_2),
        6 => (day06::solve_1, day06::solve_2),
        7 => (day07::solve_1, day07::solve_2),
        8 => (day08::solve_1, day08::solve_2),
        9 => (day09::solve_1, day09::solve_2),
        _ => return
    };

    let is_solved = solve_problem_set(first, args.day);

    if !is_solved {
        error!("Did not solve problems\n");
        return;
    }

    solve_problem_set(second, args.day);
    info!("<green><u>Hurray, you are one day closer to finding the sleigh keys 🎉\n");

}

fn solve_problem_set(solver: SolverFn, day: u32) -> bool {

    let test_solution = solve_problem(solver, day, false);

    if !test_solution {
        return false
    }

    solve_problem(solver, day, true)
}

fn solve_problem(solver: SolverFn, day: u32, real: bool) -> bool {

    let suffix = if real { "" } else { "_test" };
    let filename = format!("src/input0{}{}.txt", day, suffix);

    let answer = solver(filename);
    info!("Answer for {} problem = {}\n", if real {"real"} else {"test"}, answer);

    let mut is_correct = String::new();
    println!("Is it correct?");
    std::io::stdin().read_line(&mut is_correct).unwrap();

    if is_correct.trim() == "y" {
        info!("\n{} problem completed. Continuing...\n", if real {"real"} else {"test"});
        true
    } else {
        error!("\nbut it's wrong\n");
        false
    }
}
