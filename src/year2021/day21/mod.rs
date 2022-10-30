use crate::utils::get_input;
use std::cmp::max;
use std::collections::HashMap;

// todo: replace turn with iteration?
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Turn {
    PlayerOne,
    PlayerTwo,
}

impl Turn {
    fn toggle(&self) -> Self {
        match self {
            Turn::PlayerOne => Turn::PlayerTwo,
            Turn::PlayerTwo => Turn::PlayerOne,
        }
    }
}

fn parser(input: Option<&String>) -> Option<usize> {
    input?.split(": ").last()?.parse::<usize>().ok()
}

fn parse_input(filename: &str) -> (usize, usize) {
    let input: Vec<String> = get_input(filename);

    let first = parser(input.first()).unwrap();
    let last = parser(input.last()).unwrap();

    (first, last)
}

pub fn solve_1(filename: &str) -> String {
    let (first, second) = parse_input(filename);

    let mut first_score: usize = 0;
    let mut first_pos: usize = first - 1;
    let mut second_score = 0;
    let mut second_pos = second - 1;
    let mut iterations = 1;
    let mut throw = 1;

    loop {
        if iterations % 2 == 1 {
            first_pos = (first_pos + throw * 3 + 3) % 10;
            first_score += first_pos + 1;

            if first_score >= 1000 {
                println!(
                    "Player {} won, iteration {}, scores: {}, {}",
                    1, iterations, first_score, second_score
                );
                return (3 * iterations * second_score).to_string();
            }
        } else {
            second_pos = (second_pos + throw * 3 + 3) % 10;
            second_score += second_pos + 1;

            if second_score >= 1000 {
                println!(
                    "Player {} won, iteration {}, scores: {}, {}",
                    2, iterations, first_score, second_score
                );
                return (3 * iterations * second_score).to_string();
            }
        }

        iterations += 1;
        throw += 3;
    }
}

pub fn solve_2(filename: &str) -> String {
    let (first, second) = parse_input(filename);

    let throws = calculate_dirac_throw();
    let mut memo = HashMap::new();

    let (first_wins, second_wins) = play_quantum_dirac(
        (first-1, 0),
        (second-1, 0),
        Turn::PlayerOne,
        &throws,
        &mut memo,
    );

    println!("\nTotal wins: {} - {}", first_wins, second_wins);

    max(first_wins, second_wins).to_string()
}

// todo: replace player with struct, and add references to memo?
type Player = (usize, usize);
type State = (Player, Player, Turn);
type Wins = (usize, usize);
type Memo = HashMap<State, Wins>;

fn play_quantum_dirac(
    first: (usize, usize),
    second: (usize, usize),
    turn: Turn,
    throws: &HashMap<usize, usize>,
    memo: &mut Memo,
) -> (usize, usize) {
    let state: State = (first, second, turn);

    if memo.contains_key(&state) {
        *memo.get(&state).unwrap()
    } else if turn == Turn::PlayerOne && second.1 >= 21 {
        (0, 1)
    } else if turn == Turn::PlayerTwo && first.1 >= 21 {
        (1, 0)
    } else {
        let (first_wins, second_wins) = iterate(first, second, &turn, throws, memo);

        memo.insert((first, second, turn), (first_wins, second_wins));

        println!("Branch victories: {} - {}", first_wins, second_wins);
        (first_wins, second_wins)
    }
}

fn iterate(
    first: (usize, usize),
    second: (usize, usize),
    turn: &Turn,
    throws: &HashMap<usize, usize>,
    memo: &mut Memo,
) -> (usize, usize) {
    let mut first_victories = 0;
    let mut second_victories = 0;

    for (throw, freq) in throws.iter() {
        let (player_a, player_b) = match turn {
            Turn::PlayerOne => {
                let updated_first = update_player(first.0, first.1, *throw);
                (updated_first, second)
            }
            Turn::PlayerTwo => {
                let updated_second = update_player(second.0, second.1, *throw);
                (first, updated_second)
            }
        };

        let (new_first_victories, new_second_victories) =
            play_quantum_dirac(player_a, player_b, turn.toggle(), throws, memo);

        first_victories += freq * new_first_victories;
        second_victories += freq * new_second_victories;
    }
    (first_victories, second_victories)
}

fn calculate_dirac_throw() -> HashMap<usize, usize> {
    let throws: Vec<(usize, usize, usize)> = generate_dirac_permutations();
    let values = throws
        .into_iter()
        .map(|throw| throw.0 + throw.1 + throw.2)
        .collect::<Vec<usize>>();

    let mut frequencies = HashMap::new();

    for value in values {
        *frequencies.entry(value).or_insert(0) += 1;
    }

    frequencies
}

fn generate_dirac_permutations() -> Vec<(usize, usize, usize)> {
    let mut res = vec![];

    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                res.push((i, j, k))
            }
        }
    }

    res
}

fn aggregate_scores(scores: Vec<(usize, usize)>) -> (usize, usize) {
    let first_victories = scores.iter().map(|&score| (score).0).sum::<usize>();
    let second_victories = scores.iter().map(|&score| (score).1).sum::<usize>();
    (first_victories, second_victories)
}

// todo: use struct?
fn update_player(current_pos: usize, current_score: usize, throw: usize) -> (usize, usize) {
    let new_pos = (current_pos + throw) % 10;
    let new_score = current_score + new_pos + 1;

    (new_pos, new_score)
}
