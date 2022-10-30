mod day01;

type SolverFn = fn(&str) -> String;

pub fn get_solvers(day: u32) -> (SolverFn, SolverFn) {

    match day {
        1 => (day01::solve_1, day01::solve_2),
        _ => {
            println!("Did not find any matching days");
            std::process::exit(1);
        }
    }

}