use crate::utils::get_input_string;

pub fn solve_1(filename: &str) -> String {
    let directions = get_input_string(filename);

    let count = directions
        .chars()
        .fold(
            0,
            |acc, current|
                match current {
                    '(' => acc + 1,
                    ')' => acc - 1,
                    _ => panic!("Invalid input for direction")
                }
        )
        ;

    count.to_string()
}


pub fn solve_2(filename: &str) -> String {
    let directions = get_input_string(filename);

    println!("{}", directions.len());

    let mut floor = 0;

    for (index, char) in directions.chars().enumerate() {
        floor = match char {
            '(' => floor + 1,
            ')' => floor - 1,
            _ => panic!("Invalid input for direction")
        };

        if floor == -1 {
            return (index + 1).to_string()
        }


    }


    "Could not find an answer".to_string()
}