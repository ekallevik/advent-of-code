use crate::domain::Line;
use crate::utils::{get_input, PuzzlePart};



pub fn solve(part: PuzzlePart) -> u64 {
    println!("Puzzle day 05 - {:?}", part);
    let input = get_input("src/input05.txt");
    match part {
        PuzzlePart::Part1 => solve_part_1(input),
        PuzzlePart::Part2 => solve_part_2(input),
    }
}

fn solve_part_1(input: Vec<Line>) -> u64 {
    let size = input
        .iter()
        .map(|line| (line).largest_point())
        .max()
        .unwrap();

    let ocean = vec![vec![0; size as usize]; size as usize];

    for line in input.iter() {}

    23
}

fn solve_part_2(input: Vec<Line>) -> u64 {
    solve_part_1(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_solve_part_1() {
        let input = get_input("src/input05_test.txt");

        let expected = 5934;

        let result = solve_part_1(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_part_2() {
        let input = get_input("src/input05_test.txt");

        let expected = 26984457539;

        let result = solve_part_2(input);
        assert_eq!(result, expected)
    }
}
