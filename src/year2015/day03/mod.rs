use std::collections::HashSet;
use crate::domain::position::Position2D;
use crate::utils::get_input_string;

pub fn solve_1(filename: &str) -> String {
    let directions = get_input_string(filename);

    let mut set = HashSet::new();
    let mut current = Position2D(0, 0);
    set.insert(Position2D(0, 0));

    for c in directions.chars() {
        let new_position = to_next_position(&current, c);
        set.insert(new_position.clone());
        current = new_position
    }

    set.len().to_string()
}

pub fn solve_2(filename: &str) -> String {
    let directions = get_input_string(filename);

    // todo: use set of references
    let mut set = HashSet::new();
    let mut santa_current = Position2D(0, 0);
    let mut robo_current = Position2D(0, 0);
    set.insert(Position2D(0, 0));

    for (index, c) in directions.chars().enumerate() {

        let new_position = if index % 2 == 0 {
            to_next_position(&santa_current, c)
        } else {
            to_next_position(&robo_current, c)
        };

        set.insert(new_position.clone());

        if index % 2 == 0 {
            santa_current = new_position
        } else {
            robo_current = new_position
        };
    }

    set.len().to_string()
}

fn to_next_position(current: &Position2D, c: char) -> Position2D {
    match c {
        '>' => Position2D(current.0 + 1, current.1),
        '<' => Position2D(current.0 - 1, current.1),
        '^' => Position2D(current.0, current.1 + 1),
        'v' => Position2D(current.0, current.1 - 1),
        _ => panic!("Invalid direction")
    }
}