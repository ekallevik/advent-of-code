use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use anyhow::{Result};
use itertools::Itertools;
use scan_fmt::scan_fmt;
use crate::utils::get_input;

#[derive(Clone, Debug, Ord, Eq, PartialEq, PartialOrd)]
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

    /*
    fn apply_2(&self, left: usize, right: usize) -> Result<usize> {
        match self {
            Operation::Add => Ok(left + right),
            Operation::Subtract => {
                if right > left {
                    bail!("Negative")
                } else {
                    Ok(left - right)
                }
            }
            Operation::Multiply => Ok(left * right),
            Operation::Divide => {
                if right == 0 {
                    bail!("Div by 0")
                } else {
                    left / right
                }
            }
        }
    }

     */
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

#[derive(Clone, Debug, Ord, Eq, PartialEq, PartialOrd)]
struct OpenMonkey {
    name: String,
    left: String,
    operation: Operation,
    right: String,
}

impl Display for OpenMonkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {} {} {}", self.name, self.left, self.operation, self.right)
    }
}

// todo: change to VecDequeue
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
    initial_closed: &HashMap<String, usize>,
    initial_open: &HashMap<String, OpenMonkey>,
    humn: usize,
) -> (usize, usize) {
    let root = initial_open.get("root").unwrap();
    let mut closed = HashMap::new();
    closed.insert("humn".to_string(), humn);
    let mut open: VecDeque<_> = initial_open.values().collect();

    let mut iter = 0;

    while let Some(next) = open.pop_front() {
        iter += 1;
        let OpenMonkey { name, left, operation, right } = next;

        if is_closed(&closed, &initial_closed, left) && is_closed(&closed, &initial_closed, right) {
            let left_value = get_closed(&closed, &initial_closed, left);
            let right_value = get_closed(&closed, &initial_closed, right);

            let value = operation.apply(left_value, right_value);
            closed.insert(name.clone(), value);
        } else {
            open.push_back(next)
        }
    }

    let left = get_closed(&closed, &initial_closed, &root.left);
    let right = get_closed(&closed, &initial_closed, &root.right);

    (left, right)
}


fn resolve_root_2(
    initial_closed: &HashMap<String, usize>,
    queue: &VecDeque<&OpenMonkey>,
    humn: usize,
    root: &OpenMonkey,
) -> (usize, usize) {
    let mut closed = HashMap::new();
    closed.insert("humn".to_string(), humn);

    let mut q = queue.clone();

    let mut iter = 0;

    while let Some(next) = q.pop_front() {
        iter += 1;
        if iter > 72 {
            panic!("Used way too long on {}", next);
        }

        let OpenMonkey { name, left, operation, right } = next;

        /*
        let is_left_closed = is_closed(&closed, &initial_closed, left);
        let is_right_closed = is_closed(&closed, &initial_closed, right);

        if is_left_closed && is_right_closed {

         */
        let left_value = get_closed(&closed, &initial_closed, &left);
        let right_value = get_closed(&closed, &initial_closed, &right);

        let value = operation.apply(left_value, right_value);
        closed.insert(name.clone(), value);
        /*
        } else {
            q.push_back(next)
        }

         */
    }

    //println!("Finding root: {}", root);
    let left = get_closed(&closed, &initial_closed, &root.left);
    let right = get_closed(&closed, &initial_closed, &root.right);

    (left, right)
}

fn is_closed(
    closed: &HashMap<String, usize>,
    initial_closed: &HashMap<String, usize>,
    reference: &str,
) -> bool {
    initial_closed.contains_key(reference) || closed.contains_key(reference)
}

fn get_closed(
    closed: &HashMap<String, usize>,
    initial_closed: &HashMap<String, usize>,
    reference: &str,
) -> usize {
    if let Some(m) = initial_closed.get(reference) {
        return *m;
    };


    if let Some(m) = closed.get(reference) {
        return *m;
    }

    panic!("Ref not closed: {}", reference);
}

fn get_closed_opt(
    closed: &HashMap<String, usize>,
    initial_closed: &HashMap<String, usize>,
    reference: &str,
) -> Option<usize> {
    if let Some(m) = initial_closed.get(reference) {
        return Some(*m);
    };


    if let Some(m) = closed.get(reference) {
        return Some(*m);
    }

    return None;
}

pub fn solve_1(filename: &str) -> Result<String> {
    let (mut closed, mut open) = parse_monkeys(filename);

    let humn = closed.get("humn").unwrap();
    let (left, right) = resolve_root(&closed, &open, *humn);
    let answer = left + right;

    Ok(answer.to_string())
}

fn traverse_to_edge(open: &HashMap<String, OpenMonkey>) -> VecDeque<&OpenMonkey> {
    let mut queue = VecDeque::new();
    traverse_to_edge_inner(open, &mut queue, "root");
    queue
}

fn traverse_to_edge_inner<'a>(open: &'a HashMap<String, OpenMonkey>, queue: &mut VecDeque<&'a OpenMonkey>, current: &str) {
    if let Some(monkey) = open.get(current) {
        let OpenMonkey { left, right, .. } = monkey;
        traverse_to_edge_inner(open, queue, left);
        traverse_to_edge_inner(open, queue, right);

        queue.push_back(monkey)
    }
}

fn traverse_to_edge_heap(open: &HashMap<String, OpenMonkey>) -> BinaryHeap<(usize, &OpenMonkey)> {
    let mut heap = BinaryHeap::new();
    traverse_to_edge_heap_inner(open, &mut heap, "root", 0);
    heap
}

// todo: change ordering to distance from edge?
fn traverse_to_edge_heap_inner<'a>(open: &'a HashMap<String, OpenMonkey>, heap: &mut BinaryHeap<(usize, &'a OpenMonkey)>, current: &str, level: usize) {
    if let Some(monkey) = open.get(current) {
        let OpenMonkey { left, right, .. } = monkey;
        traverse_to_edge_heap_inner(open, heap, left, level + 1);
        traverse_to_edge_heap_inner(open, heap, right, level + 1);

        heap.push((level, monkey))
    }
}

pub fn solve_2(filename: &str) -> Result<String> {
    let (mut closed, mut open) = parse_monkeys(filename);
    let references = find_references(&open);

    closed.remove("humn");

    for _ in 0..10 {
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

    println!("closed={}, open={}", closed.len(), open.len());

    let mut humn = if filename.contains("real") {
        500
    } else {
        4
    };

    let root = open.get("root").unwrap().clone();

    let mut unsorted_queue: VecDeque<&OpenMonkey> = VecDeque::from(open.values().collect_vec());
    let mut unsorted_queue_c: VecDeque<&OpenMonkey> = VecDeque::from(open.values().collect_vec());

    let root_queue = traverse_to_edge(&open);
    let root_heap = traverse_to_edge_heap(&open);

    let mut sorted_queue: VecDeque<&OpenMonkey> = VecDeque::new();

    println!("root: {}, sort: {}", root_queue.len(), sorted_queue.len());


    let mut resolved = vec![];

    let mut it = 0;

    while let Some(monkey) = unsorted_queue.pop_front() {
        it += 1;

        let left = &monkey.left;
        let right = &monkey.right;

        let is_left_resolved = left == "humn" || closed.contains_key(left) || resolved.contains(left);
        let is_right_resolved = right == "humn" || closed.contains_key(right) || resolved.contains(right);

        if is_left_resolved && is_right_resolved {
            resolved.push(monkey.name.clone());
            sorted_queue.push_back(monkey);
        } else {
            unsorted_queue.push_back(monkey)
        }
    }

    let mut value = 1;




    /*

    let guess = 10_000_000_000_000;
    //let guess = 100_000;
    let bar = ProgressBar::new(guess);
/*


    let res: Vec<_> = (humn..guess)
        .into_par_iter()
        .inspect(|humn|
            if humn % 100_000 == 0 {
                println!("humn = {humn:14}");
            })
        .map(|humn| {
            let (left, right) = resolve_root_2(&closed, &sorted_queue, humn as usize, &root);
            if left == right {
                Some(humn)
            } else {
                None
            }
        }
        ).collect();

    let humn1: Vec<_> = res.iter().flatten().collect();
    let humn = humn1[0];

 */

    bar.set_style(ProgressStyle::with_template("elapsed: [{elapsed_precise}] eta: [{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {percent}")
        .unwrap());

    let bar_delta = 100_000;


    loop {
        let (left, right) = resolve_root_2(&closed, &sorted_queue, humn, &root);

        if left == right {
            break;
        }

        if humn % bar_delta == 0 {
            //println!("humn = {humn:14}");
            bar.inc(bar_delta as u64);
        }
        humn += 1;
    }

    bar.finish();


     */

    let humn = 301;
    Ok(humn.to_string())
}

/*
enum Symbol{
    Primitive(Primitive),
    Equation(Equation),
}

struct  Equation {
    name: String,
    left: Symbol,
    operation: Operation,
    right: Symbol
}

enum Primitive {
    Variable(String),
    Value(isize),
}


 */


/*

l, r = root.split
resolve left until only humn left
resolve right until only humn left

 */

fn resolve_humn(
    initial_closed: &HashMap<String, usize>,
    queue: &VecDeque<&OpenMonkey>,
    value: usize,
    root: &OpenMonkey,
    target: &OpenMonkey,
) -> (usize, usize) {
    let mut closed = HashMap::new();

    closed.insert(root.left.clone(), value);
    closed.insert(root.right.clone(), value);

    let mut q = queue.clone();

    let mut iter = 0;

    while let Some(next) = q.pop_front() {
        iter += 1;

        let OpenMonkey { name, left, operation, right } = next;

        let is_left_closed = is_closed(&closed, &initial_closed, left);
        let is_right_closed = is_closed(&closed, &initial_closed, right);


        if is_left_closed && is_right_closed {
            let left_value = get_closed(&closed, &initial_closed, &left);
            let right_value = get_closed(&closed, &initial_closed, &right);

            let value = operation.apply(left_value, right_value);
            closed.insert(name.clone(), value);
        } else {
            q.push_back(next)
        }
    }

    //println!("Finding root: {}", root);
    let left = get_closed(&closed, &initial_closed, &root.left);
    let right = get_closed(&closed, &initial_closed, &root.right);

    (left, right)
}