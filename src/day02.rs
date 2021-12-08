use crate::domain::{NaivePosition, Position, SubmarineCommand};
use crate::utils::{get_input, PuzzlePart};

pub fn solve(part: PuzzlePart) -> i64 {
    println!("Puzzle day 02 - {:?}", part);
    let input = get_input("src/input02.txt");

    match part {
        PuzzlePart::Part1 => calculate_naive_position(input),
        PuzzlePart::Part2 => calculate_position(input),
    }
}

fn calculate_naive_position(commands: Vec<SubmarineCommand>) -> i64 {
    let pos = commands
        .iter()
        .fold(NaivePosition { x: 0, y: 0 }, |p, command| {
            command.naive_apply_from(p)
        });

    (pos.x * pos.y).abs()
}

fn calculate_position(commands: Vec<SubmarineCommand>) -> i64 {
    let pos = commands
        .iter()
        .fold(Position { x: 0, y: 0, aim: 0 }, |p, command| {
            command.apply_from(p)
        });

    (pos.x * pos.y).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_position() {
        let input = vec![
            SubmarineCommand::Forward(5),
            SubmarineCommand::Down(5),
            SubmarineCommand::Forward(8),
            SubmarineCommand::Up(3),
            SubmarineCommand::Down(8),
            SubmarineCommand::Forward(2),
        ];

        let result = calculate_naive_position(input);
        assert_eq!(result, 150)
    }

    #[test]
    fn test_calculate_position_using_aim() {
        let input = vec![
            SubmarineCommand::Forward(5),
            SubmarineCommand::Down(5),
            SubmarineCommand::Forward(8),
            SubmarineCommand::Up(3),
            SubmarineCommand::Down(8),
            SubmarineCommand::Forward(2),
        ];

        let result = calculate_position(input);
        assert_eq!(result, 900)
    }
}
