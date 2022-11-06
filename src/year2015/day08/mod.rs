use fancy_regex::Regex;
use crate::utils::get_input;

pub fn solve_1(filename: &str) -> String {
    let input: Vec<String> = get_input(filename);

    let answer: usize = input
        .iter()
        .map(|line| find_decoding_difference(line))
        .sum();

    answer.to_string()
}

pub fn solve_2(filename: &str) -> String {
    let input: Vec<String> = get_input(filename);

    let answer: usize = input
        .iter()
        .map(|line| find_encoding_difference(line))
        .sum();

    answer.to_string()
}

fn get_ascii_regex() -> Regex {
    Regex::new(r"(\\x[0-9a-f]{2})").unwrap()
}

fn get_escaped_regex() -> Regex {
    Regex::new(r#"(\\{2})|(\\")"#).unwrap()
}


fn find_decoding_difference(line: &str) -> usize {

    let re_ascii = get_ascii_regex();
    let re_escaped = get_escaped_regex();

    let mut chars = 2;

    if re_ascii.is_match(line).unwrap() {
        chars += 3 * re_ascii.captures_iter(line).count();
    }

    if re_escaped.is_match(line).unwrap() {
        chars += re_escaped.captures_iter(line).count();
    }

    chars
}

fn find_encoding_difference(line: &str) -> usize {

    let re_ascii = get_ascii_regex();
    let re_escaped = get_escaped_regex();

    let mut chars = 4;

    if re_ascii.is_match(line).unwrap() {
        chars += re_ascii.captures_iter(line).count();
    }

    if re_escaped.is_match(line).unwrap() {
        chars += 2*re_escaped.captures_iter(line).count();
    }

    chars
}
