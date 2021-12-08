use crate::utils::{PuzzlePart};
use vectrix::Matrix;

type Board = Matrix<(i64, bool), 5, 5>;

pub fn solve(part: PuzzlePart) -> i64 {
    println!("Puzzle day 04 - {:?}", part);
    let input = include_str!("input04.txt");
    let (draws, boards) = parse_input(input);

    match part {
        PuzzlePart::Part1 => solve_part_1(boards, draws),
        PuzzlePart::Part2 => solve_part_2(boards, draws),
    }
}

fn update(board: Board, draw: i64) -> Board {
    board
        .into_iter()
        .map(|(n, marked)| (n, marked || n == draw))
        .collect()
}

fn is_win(board: &Board) -> bool {
    let row_win = board
        .iter_rows()
        .any(|row| row.iter().all(|(_, marked)| *marked));
    let col_win = board
        .iter_columns()
        .any(|col| col.iter().all(|(_, marked)| *marked));
    row_win || col_win
}

fn score(board: &Board, d: i64) -> i64 {
    let sum: i64 = board
        .iter()
        .filter(|(_, marked)| !marked as bool)
        .map(|(n, _)| n)
        .sum();
    sum * d
}

fn solve_part_1(mut boards: Vec<Board>, draws: Vec<i64>) -> i64 {
    for draw in draws {
        for board in boards.iter_mut() {
            *board = update(*board, draw);
            if is_win(board) {
                return score(board, draw);
            }
        }
    }

    unreachable!()
}

fn solve_part_2(mut boards: Vec<Board>, draws: Vec<i64>) -> i64 {
    for d in draws {
        if let [board] = *boards {
            return score(&update(board, d), d);
        }
        boards = boards
            .into_iter()
            .map(|board| update(board, d))
            .filter(|board| !is_win(board))
            .collect();
    }
    unreachable!()
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<Board>) {
    let (draws, boards) = input.split_once("\n\n").unwrap();
    let draws = draws
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    let boards = boards
        .split("\n\n")
        .map(|board| {
            board
                .lines()
                .flat_map(|line| {
                    line.split_whitespace()
                        .map(str::parse)
                        .map(Result::unwrap)
                        .map(|n| (n, false))
                })
                .collect()
        })
        .collect();
    (draws, boards)
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_solve_part_1() {
        let input = include_str!("input04_test.txt");
        let (draws, boards) = parse_input(input);

        let expected = 4512;

        let result = solve_part_1(boards, draws);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_part_2() {
        let input = include_str!("input04_test.txt");
        let (draws, boards) = parse_input(input);

        let expected = 1924;

        let result = solve_part_2(boards, draws);
        assert_eq!(result, expected)
    }
}
