use fancy_regex::Regex;
use crate::utils::get_input;


pub fn solve_1(filename: &str) -> String {
    let lines: Vec<String> = get_input(filename);

    let count = lines
        .iter()
        .filter(|line| is_ridiculous_string(line))
        .count();

    count.to_string()
}

fn is_ridiculous_string(value: &str) -> bool {
    has_three_vowels(value) && does_not_contain_illegal_patterns(value) && has_repeated_char(value)
}

fn has_three_vowels(value: &str) -> bool {
    let re = Regex::new(r"[aeiou]").unwrap();
    let number_of_matches = re
        .find_iter(value)
        .count();
    number_of_matches >= 3
}

fn does_not_contain_illegal_patterns(value: &str) -> bool {
    let re = Regex::new(r"((ab)|(cd)|(pq)|(xy))+").unwrap();
    !re.is_match(value).unwrap()
}

fn has_repeated_char(value: &str) -> bool {
    let re = Regex::new(r"([a-z])\1+").unwrap();
    re.is_match(value).unwrap()
}

pub fn solve_2(filename: &str) -> String {
    let lines: Vec<String> = get_input(filename);

    let count = lines
        .iter()
        .filter(|line| is_nice_string(line))
        .count();

    count.to_string()
}

fn is_nice_string(value: &str) -> bool {
    has_repeated_pair(value) && has_repeated_seperated_char(value)
}

fn has_repeated_pair(value: &str) -> bool {
    let re = Regex::new(r"([a-z]{2})([a-z])*\1+").unwrap();
    re.is_match(value).unwrap()
}

fn has_repeated_seperated_char(value: &str) -> bool {
    let re = Regex::new(r"([a-z])([a-z])\1+").unwrap();
    re.is_match(value).unwrap()
}
