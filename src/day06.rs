use crate::utils::{PuzzlePart};


pub fn solve(part: PuzzlePart) -> u64 {
    println!("Puzzle day 0 - {:?}", part);
    let input = vec![
        4, 1, 4, 1, 3, 3, 1, 4, 3, 3, 2, 1, 1, 3, 5, 1, 3, 5, 2, 5, 1, 5, 5, 1, 3, 2, 5, 3, 1, 3,
        4, 2, 3, 2, 3, 3, 2, 1, 5, 4, 1, 1, 1, 2, 1, 4, 4, 4, 2, 1, 2, 1, 5, 1, 5, 1, 2, 1, 4, 4,
        5, 3, 3, 4, 1, 4, 4, 2, 1, 4, 4, 3, 5, 2, 5, 4, 1, 5, 1, 1, 1, 4, 5, 3, 4, 3, 4, 2, 2, 2,
        2, 4, 5, 3, 5, 2, 4, 2, 3, 4, 1, 4, 4, 1, 4, 5, 3, 4, 2, 2, 2, 4, 3, 3, 3, 3, 4, 2, 1, 2,
        5, 5, 3, 2, 3, 5, 5, 5, 4, 4, 5, 5, 4, 3, 4, 1, 5, 1, 3, 4, 4, 1, 3, 1, 3, 1, 1, 2, 4, 5,
        3, 1, 2, 4, 3, 3, 5, 4, 4, 5, 4, 1, 3, 1, 1, 4, 4, 4, 4, 3, 4, 3, 1, 4, 5, 1, 2, 4, 3, 5,
        1, 1, 2, 1, 1, 5, 4, 2, 1, 5, 4, 5, 2, 4, 4, 1, 5, 2, 2, 5, 3, 3, 2, 3, 1, 5, 5, 5, 4, 3,
        1, 1, 5, 1, 4, 5, 2, 1, 3, 1, 2, 4, 4, 1, 1, 2, 5, 3, 1, 5, 2, 4, 5, 1, 2, 3, 1, 2, 2, 1,
        2, 2, 1, 4, 1, 3, 4, 2, 1, 1, 5, 4, 1, 5, 4, 4, 3, 1, 3, 3, 1, 1, 3, 3, 4, 2, 3, 4, 2, 3,
        1, 4, 1, 5, 3, 1, 1, 5, 3, 2, 3, 5, 1, 3, 1, 1, 3, 5, 1, 5, 1, 1, 3, 1, 1, 1, 1, 3, 3, 1,
    ];
    match part {
        PuzzlePart::Part1 => solve_part_1(input, 80),
        PuzzlePart::Part2 => solve_part_2(input, 256),
    }
}

fn solve_part_1(input: Vec<u64>, days: u16) -> u64 {
    let mut count = (0..=8)
        .map(|i| input.iter().filter(|&fish| *fish == i).count() as u64)
        .collect::<Vec<u64>>();

    for _ in 0..days {
        let number_of_zeroes = count[0];
        count = count[1..].to_vec();
        count[6] += number_of_zeroes;
        count.push(number_of_zeroes);
    }

    count.iter().sum::<u64>()
}

fn solve_part_2(input: Vec<u64>, days: u16) -> u64 {
    solve_part_1(input, days)
}

#[cfg(test)]
mod tests {
    use crate::utils::get_input;
    use super::*;
    

    #[test]
    fn test_solve_part_1() {
        let input = get_input("src/input6_test.txt");

        let expected = 5934;

        let result = solve_part_1(input, 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_part_2() {
        let input = get_input("src/input6_test.txt");

        let expected = 26984457539;

        let result = solve_part_2(input, 256);
        assert_eq!(result, expected)
    }
}
