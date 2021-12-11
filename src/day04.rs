use vectrix::Matrix;

type Board = Matrix<(i64, bool), 5, 5>;

pub fn solve_1(filename: &String) -> String {
    let (draws, boards) = parse_input(filename);
    solve_part_1(boards, draws).to_string()
}

pub fn solve_2(filename: &String) -> String {
    let (draws, boards) = parse_input(filename);
    solve_part_2(boards, draws).to_string()
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

fn parse_input(input: &String) -> (Vec<i64>, Vec<Board>) {
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
