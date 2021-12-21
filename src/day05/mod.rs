use crate::domain::Line;
use crate::utils::get_input;

pub fn solve_1(filename: &str) -> String {
    let input: Vec<Line> = get_input(filename);

    let size = input
        .iter()
        .map(|line| (line).largest_point())
        .max()
        .unwrap();

    let _ocean = vec![vec![0; size as usize]; size as usize];

    for _line in input.iter() {}

    "-1".to_string()
}

pub fn solve_2(filename: &str) -> String {
    solve_1(filename)
}
