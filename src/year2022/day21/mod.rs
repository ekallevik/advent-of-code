use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use anyhow::{bail, Result};
use scan_fmt::scan_fmt;
use crate::utils::get_input;

#[derive(Clone, Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    fn apply(&self, left: usize, right: usize) -> usize {
        match self {
            Operation::Add => left + right,
            Operation::Subtract => left - right,
            Operation::Multiply => left * right,
            Operation::Divide => left / right,
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Subtract => write!(f, "-"),
            Operation::Multiply => write!(f, "*"),
            Operation::Divide => write!(f, "/"),
        }
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let operation = match s {
            "+" => Operation::Add,
            "-" => Operation::Subtract,
            "*" => Operation::Multiply,
            "/" => Operation::Divide,
            _ => return Err("Unsupported operation".to_string())
        };

        Ok(operation)
    }
}


#[derive(Clone, Debug)]
struct ClosedMonkey {
    name: String,
    value: usize,
}


#[derive(Clone, Debug)]
struct OpenMonkey {
    name: String,
    left: String,
    operation: Operation,
    right: String,
}

fn parse_monkeys(filename: &str) -> (HashMap<String, usize>, HashMap<String, OpenMonkey>) {
    let input: Vec<String> = get_input(filename);
    let number_of_monkeys = input.len();

    let mut closed = HashMap::new();
    let mut open = HashMap::new();

    for line in input {
        let (name, operation) = line.split_once(": ").unwrap();
        let monkey_name = name.to_string();

        if let Ok(number) = operation.parse::<usize>() {
            closed.insert(monkey_name, number);
        } else {
            let (left, operation, right) = scan_fmt!(operation, "{} {} {}", String, String, String).unwrap();

            let monkey = OpenMonkey {
                name: monkey_name.clone(),
                left: left.clone(),
                operation: operation.parse().unwrap(),
                right: right.clone(),
            };

            open.insert(monkey_name, monkey);
        };
    }

    (closed, open)
}

fn find_references(open: &HashMap<String, OpenMonkey>) -> HashMap<String, Vec<String>> {
    let mut references = HashMap::new();

    for (name, monkey) in open {
        let left = monkey.left.clone();
        let left_entry = references.entry(left).or_insert(vec![]);
        left_entry.push(name.clone());
    }

    for (name, monkey) in open {
        let right = monkey.right.clone();
        let right_entry = references.entry(right).or_insert(vec![]);
        right_entry.push(name.clone());
    }

    references
}

fn resolve_root(
    mut closed: HashMap<String, usize>,
    mut open: HashMap<String, OpenMonkey>,
    references: HashMap<String, Vec<String>>
) -> usize {

    while open.len() != 0 {
        for (name, dependants) in &references {
            if closed.contains_key(name) {
                for dependant in dependants {
                    if let Some(monkey) = open.get(dependant) {
                        let monkey_name = monkey.name.clone();
                        if closed.contains_key(&monkey.left) && closed.contains_key(&monkey.right) {
                            let left = closed.get(&monkey.left).unwrap();
                            let right = closed.get(&monkey.right).unwrap();
                            let value = monkey.operation.apply(*left, *right); // todo

                            closed.insert(monkey_name.clone(), value);
                            open.remove(&monkey_name.clone());
                        }
                    }
                }
            }
        }
    }

    println!("dvpt={}", closed.get("dvpt").unwrap_or(&0));
    println!("sbhj={}", closed.get("sbhj").unwrap_or(&0));

    *closed.get("root").unwrap()
}


fn resolve_root_2(
    original_closed: HashMap<String, usize>,
    original_open: HashMap<String, OpenMonkey>,
    references: &HashMap<String, Vec<String>>,
    root: &OpenMonkey
) -> bool {

    let mut closed = HashMap::new();
    let mut open = HashMap::new();

    while open.len() != original_closed {

        for (name, dependants) in references {

            if closed.contains_key(name) {
                for dependant in dependants {
                    if let Some(monkey) = open.get(dependant) {
                        let monkey_name = monkey.name.clone();
                        if closed.contains_key(&monkey.left) && closed.contains_key(&monkey.right) {
                            let left = closed.get(&monkey.left).unwrap();
                            let right = closed.get(&monkey.right).unwrap();
                            let value = monkey.operation.apply(*left, *right); // todo

                            closed.insert(monkey_name.clone(), value);
                            open.remove(&monkey_name.clone());
                        }
                    }
                }
            }
        }
    };

    let left = closed.get(&root.left).unwrap();
    let right = closed.get(&root.right).unwrap();

    //println!("- left={}, right={}, root={}", left, right, left+right);
    left == right
}

pub fn solve_1(filename: &str) -> Result<String> {
    let (mut closed, mut open) = parse_monkeys(filename);
    let references = find_references(&open);

    let root = resolve_root(closed, open, references);
    Ok(root.to_string())
}

pub fn solve_2(filename: &str) -> Result<String> {

    let (mut closed, mut open) = parse_monkeys(filename);
    let mut references = find_references(&open);

    closed.remove("humn");

    let mut min_refs = HashMap::new();

    for _ in 0..10 {
        for (name, dependants) in &references {

            let mut retained_references = vec![];

            if closed.contains_key(name) {
                for dependant in dependants {

                    if let Some(monkey) = open.get(dependant) {
                        let monkey_name = monkey.name.clone();

                        if closed.contains_key(&monkey.left) && closed.contains_key(&monkey.right) {
                            let left = closed.get(&monkey.left).unwrap();
                            let right = closed.get(&monkey.right).unwrap();
                            let value = monkey.operation.apply(*left, *right); // todo

                            closed.insert(monkey_name.clone(), value);
                            open.remove(&monkey_name.clone());
                        } else {
                            retained_references.push(monkey_name);
                        }
                    }
                }
            }

            if retained_references.is_empty() {
                min_refs.remove(name);
            } else {
                min_refs.insert(name, retained_references);
            };
        }
    }

    println!("refs={}, min={}", references.len(), min_refs.len());
    println!("closed={}, open={}", closed.len(), open.len());

    let mut humn = if filename.contains("real") {
        500
    } else {
        4
    };

    let root = open.get("root").unwrap();

    loop {
        println!("humn = {humn}");

        let mut closed_clone = closed.clone();
        closed_clone.insert("humn".to_string(), humn);

        let open_clone = open.clone();
        let res = resolve_root_2(closed_clone, open_clone, &references, root);

        if res {
            break
        }

        humn += 1;
    }

    Ok(humn.to_string())
}


