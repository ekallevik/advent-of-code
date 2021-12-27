use crate::utils::get_input;
use std::borrow::Borrow;
use std::cmp::max;
use std::collections::VecDeque;
use std::fmt;
use std::str::FromStr;
use itertools::Itertools;
use snailfish::Snailfish;

mod snailfish;

pub fn solve_1(filename: &str) -> String {
    let input: Vec<String> = get_input(filename);
    let mut school = VecDeque::from_iter(input.iter().map(|fish| Snailfish::parse(fish)));

    add_numbers(&mut school).to_string()
}

fn add_numbers(school: &mut VecDeque<Snailfish>) -> usize {
    let mut snailfish = school.pop_front().unwrap();

    while !school.is_empty() {
        let next = school.pop_front().unwrap();
        let added = snailfish.add(next);
        snailfish = reduce(added);
    }
    snailfish.calculate_magnitude()
}

fn reduce(mut snailfish: Snailfish) -> Snailfish {
    loop {
        // todo: drop last return value?
        let (_, exploded, _, _) = snailfish.traverse_and_explode(0, false);
        let (has_splitted, splitted) = exploded.traverse_and_split(false);
        snailfish = splitted;

        if !has_splitted {
            return snailfish
        }
    }
}

pub fn solve_2(filename: &str) -> String {
    let input: Vec<String> = get_input(filename);
    let mut school: Vec<Snailfish> = input.iter().map(|fish| Snailfish::parse(fish)).collect();

    let mut largest_sum = 0;
    for perm in school.iter().permutations(2) {

        let a = *perm.first().unwrap();
        let b = *perm.last().unwrap();


        let mut first_school = VecDeque::from_iter([a.clone(), b.clone()]);
        let first_sum = add_numbers(&mut first_school);

        let mut second_school = VecDeque::from_iter([b.clone(), a.clone()]);
        let second_sum = add_numbers(&mut second_school);

        let sum = max(first_sum, second_sum);
        largest_sum = max(largest_sum, sum);
    }

    largest_sum.to_string()

}


fn breakpoint(message: &str) {
    let mut is_correct = String::new();
    println!("{}", message);
    std::io::stdin().read_line(&mut is_correct).unwrap();
}
