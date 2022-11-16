use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use anyhow::Result;
use fancy_regex::Regex;
use itertools::Itertools;
use paris::info;
use crate::utils::get_input;

type Relation = (String, String);
type ValuedRelation = (String, String, i32);

fn parse_input(input: &str) -> ValuedRelation {
    let re = Regex::new(
        r#"(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+)+"#
    ).unwrap();

    let cap = re.captures(input).unwrap().unwrap();

    let first = cap.get(1).unwrap().as_str();
    let direction = cap.get(2).unwrap().as_str();
    let magnitude: i32 = cap.get(3).unwrap().as_str().parse().unwrap();
    let second = cap.get(4).unwrap().as_str();

    let value = if direction == "gain" { magnitude } else { -magnitude };

    (first.to_string(), second.to_string(), value)
}

fn parse_inputs(filename: &str) -> Vec<ValuedRelation> {
    get_input(filename)
        .into_iter()
        .map(|line: String| parse_input(&*line))
        .collect()
}

pub fn solve_1(filename: &str) -> Result<String> {
    let values = parse_inputs(filename);

    let (relations, guests) = get_relations(values);
    info!("Guests={guests:?}");

    let arrangements = get_arrangements(&guests);
    let happiness = find_best_arrangement(&relations, arrangements);

    Ok(happiness.to_string())
}

fn get_relations(values: Vec<ValuedRelation>) -> (HashMap<Relation, i32>, HashSet<String>) {
    let mut relations: HashMap<(String, String), i32> = HashMap::new();
    let mut guests = HashSet::new();

    for (first, second, happiness) in values {
        guests.insert(first.clone());

        let key = get_sorted_key(&*first, &*second);
        let current = relations.get(&key).unwrap_or(&0);
        relations.insert(key, current + happiness);
    };

    (relations, guests)
}

fn find_best_arrangement(relations: &HashMap<(String, String), i32>, arrangements: Vec<Vec<&String>>) -> i32 {
    arrangements
        .into_iter()
        .map(|arrangement| calculate_happiness(arrangement, relations))
        .max()
        .unwrap()
}


fn get_arrangements(guests: &HashSet<String>) -> Vec<Vec<&String>> {
    let first_guest = guests.iter().next().unwrap();

    guests
        .iter()
        .permutations(guests.len())
        .unique()
        .filter(|permutation| permutation.first().unwrap() == &first_guest)
        .map(|mut permutation| {
            permutation.push(first_guest);
            permutation
        })
        .collect::<Vec<Vec<&String>>>()
}

fn get_sorted_key(first: &str, second: &str) -> (String, String) {
    let (a, b) = match first.cmp(second) {
        Ordering::Less => (first, second),
        Ordering::Equal => (first, second),
        Ordering::Greater => (second, first)
    };
    (a.parse().unwrap(), b.parse().unwrap())
}

fn calculate_happiness(arrangement: Vec<&String>, values: &HashMap<(String, String), i32>) -> i32 {
    let mut sum = 0;

    for windows in arrangement.windows(2) {
        let l = windows.first().unwrap();
        let r = windows.last().unwrap();
        sum += get_happiness_pair(l, r, values);
    };

    sum
}

fn get_happiness_pair(left: &str, right: &str, values: &HashMap<(String, String), i32>) -> i32 {
    let key = get_sorted_key(left, right);
    *values.get(&key).unwrap()
}

pub fn solve_2(filename: &str) -> Result<String> {
    let values = parse_inputs(filename);

    let (mut relations, mut guests) = get_relations(values);

    let myself = "Myself".to_string();

    for guest in &guests {
        relations.insert((guest.clone(), myself.clone()), 0);
    }

    guests.insert("Myself".to_string());

    info!("Guests={guests:?}");

    let arrangements = get_arrangements(&guests);
    let happiness = find_best_arrangement(&relations, arrangements);

    Ok(happiness.to_string())
}