use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Debug)]
pub enum Fold {
    X(i64),
    Y(i64),
}

fn parse_input(filename: &str) -> (HashSet<(i64, i64)>, Vec<Fold>) {
    let mut positions: HashSet<(i64, i64)> = HashSet::new();
    let mut folds = vec![];

    std::fs::read_to_string(filename)
        .expect("file not found!")
        .lines()
        .for_each(|line| {
            if line.contains("fold") {
                let split: Vec<&str> = line.split('=').collect();
                let value: i64 = split[1].parse().unwrap();

                match split[0].contains('x') {
                    true => folds.push(Fold::X(value)),
                    false => folds.push(Fold::Y(value)),
                };
            } else if line.contains(',') {
                let split: Vec<&str> = line.split(',').collect();
                positions.insert((split[0].parse().unwrap(), split[1].parse().unwrap()));
            }
        });

    (positions, folds)
}

pub fn solve_1(filename: &str) -> String {
    let (elements, folds) = parse_input(filename);

    let first_fold = folds.first().unwrap();
    fold_paper(elements, first_fold).len().to_string()
}

pub fn solve_2(filename: &str) -> String {
    let (mut elements, folds) = parse_input(filename);

    for fold in folds {
        elements = fold_paper(elements, &fold);
    }

    folded_to_string(&elements)
}

fn fold_paper(input: HashSet<(i64, i64)>, fold: &Fold) -> HashSet<(i64, i64)> {
    let mut folded = HashSet::new();

    for position in input {
        match fold {
            Fold::X(value) => {
                match position.0.cmp(value) {
                    Ordering::Less => folded.insert((position.0, position.1)),
                    Ordering::Equal | Ordering::Greater => {
                        folded.insert((position.0 - 2 * (position.0 - *value), position.1))
                    }
                }
            }
            Fold::Y(value) => {
                match position.1.cmp(value) {
                    Ordering::Less => folded.insert(position),
                    Ordering::Equal | Ordering::Greater => {
                        folded.insert((position.0, position.1 - 2 * (position.1 - *value)))
                    }
                }
            }
        };
    }

    folded
}

fn folded_to_string(folded: &HashSet<(i64, i64)>) -> String {
    let max_x = folded.iter().max_by_key(|&pos| pos.0).unwrap().0;
    let max_y = folded.iter().max_by_key(|&pos| pos.1).unwrap().1;

    let mut result = "".to_string();

    for j in 0..=max_y {
        let mut row = "".to_string();
        for i in 0..=max_x {
            if folded.contains(&(i, j)) {
                row.push('▧');
            } else {
                row.push(' ');
            }
        }

        row.push('\n');
        result.push_str(&row);
    }

    result
}
