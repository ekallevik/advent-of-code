use crate::utils::SolverFn;

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
mod day14;

pub fn get_solvers(day: u32) -> (SolverFn, SolverFn) {

    match day {
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
        14 => (day14::solve_1, day14::solve_2),
        _ => {
            println!("Did not find any matching days");
            std::process::exit(1);
        }
    }

}