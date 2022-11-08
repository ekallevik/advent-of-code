use fancy_regex::Regex;
use itertools::Itertools;
use paris::info;
use crate::utils::get_input_string;

pub fn solve_1(filename: &str) -> String {
    let mut password = get_input_string(filename);

    loop {
        password = increment_password(&password);

        info!("{password}");
        if is_valid(&password) {
            break
        }
    }

    password
}

pub fn solve_2(_: &str) -> String {
    let mut password = "hepxxyzz".to_string();

    loop {
        info!("{password}");
        password = increment_password(&password);
        if is_valid(&password) {
            break
        }
    }

    password
}

fn increment_password(password: &str) -> String {
    let mut output = "".to_string();
    let reversed = password.chars().rev();

    let mut should_increment = true;

    for c in reversed {
        let character = match (should_increment, c) {
            (_, 'i') => {
                let mut new_output = "".to_string();
                for _ in output.chars() {
                    new_output.push('a');
                }
                output = new_output;
                should_increment = false;
                'j'
            }
            (_, 'l') => {
                let mut new_output = "".to_string();
                for _ in output.chars() {
                    new_output.push('a');
                }
                should_increment = false;
                output = new_output;
                'm'
            }
            (_, 'o') => {
                let mut new_output = "".to_string();
                for _ in output.chars() {
                    new_output.push('a');
                }
                should_increment = false;
                output = new_output;
                'p'
            }
            (true, 'z') => {
                should_increment = true;
                'a'
            }
            (true, _) => {
                should_increment = false;
                char::from_u32(c as u32 + 1).unwrap()
            }
            (false, _) => char::from_u32(c as u32).unwrap()
        };

        output.push(character);
    }

    output.chars().rev().collect()
}

fn is_valid(password: &str) -> bool {
    let re_invalid = Regex::new(r#"i|o|l"#).unwrap();
    !re_invalid.is_match(password).unwrap()
        && contains_sequence(password)
        && contains_pairs(password)
}

fn contains_sequence(password: &str) -> bool {
    for (a, b, c) in password.chars().tuple_windows() {
        if a as u32 == (b as u32 - 1) && b as u32 == (c as u32 - 1) {
            return true;
        }
    }

    false
}

fn contains_pairs(password: &str) -> bool {
    let re = Regex::new(r#"([a-z])\1\w*(?!\1)([a-z])\2"#).unwrap();
    re.is_match(password).unwrap()
}

