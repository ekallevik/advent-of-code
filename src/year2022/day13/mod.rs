use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::str::{Chars, FromStr};
use anyhow::Result;
use itertools::Itertools;
use crate::utils::{get_input, get_input_string};
use crate::utils::string::is_numeric;

impl Display for Distress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Distress::Array(value) => {
                write!(f, "[").expect("");
                for v in value {
                    write!(f, "{}, ", v).expect("");
                }
                write!(f, "]").expect("");
            }
            Distress::Number(number) => write!(f, "{}", number).expect("")
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Distress {
    Array(Vec<Distress>),
    Number(usize),
}

impl PartialOrd<Self> for Distress {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Distress::Number(l), Distress::Number(r)) => l.partial_cmp(&r),
            (Distress::Number(l), Distress::Array(_)) => {
                let wrapped = Distress::Array(vec![Distress::Number(*l)]);
                wrapped.partial_cmp(other)
            }
            (Distress::Array(_), Distress::Number(r)) => {
                let wrapped = &Distress::Array(vec![Distress::Number(*r)]);
                self.partial_cmp(wrapped)
            }
            (Distress::Array(left_array), Distress::Array(right_array)) => {
                for (i, l) in left_array.iter().enumerate() {
                    if let Some(r) = right_array.get(i) {
                        let ordering = l.cmp(r);

                        match ordering {
                            Ordering::Less => return Some(Ordering::Less),
                            Ordering::Equal => continue,
                            Ordering::Greater => return Some(Ordering::Greater),
                        }
                    } else {
                        return Some(Ordering::Greater);
                    }
                }

                return if left_array.len() < right_array.len() {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Equal)
                };
            }
        }
    }
}

impl Ord for Distress {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromStr for Distress {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut chars = s.chars();
        chars.next();

        Ok(parse_element(&mut chars))
    }
}

fn parse_element(element: &mut Chars) -> Distress {
    let mut result = vec![];
    let mut token = String::new();

    while let Some(current) = element.next() {
        if current.is_numeric() {
            token.push(current);
        } else if current == '[' {
            let elements = parse_element(element);
            result.push(elements);
        } else {
            if !token.is_empty() {
                let number = token.parse().unwrap();
                result.push(Distress::Number(number));
                token.clear();
            }

            if current == ']' {
                break;
            }
        }
    }

    Distress::Array(result)
}

fn parse_pairs(filename: &str) -> Vec<(Distress, Distress)> {
    let input: Vec<String> = get_input(filename);
    let lines = input.iter().filter(|l| !l.is_empty()).collect_vec();

    let mut pairs = vec![];

    for a in lines.chunks(2) {
        let left: Distress = a[0].parse().unwrap();
        let right: Distress = a[1].parse().unwrap();

        pairs.push((left, right));
    }

    pairs
}

fn parse_packets(filename: &str) -> Vec<Distress> {
    let input: Vec<String> = get_input(filename);
    let lines = input.iter().filter(|l| !l.is_empty()).collect_vec();

    lines.iter().map(|elem| elem.parse().unwrap()).collect_vec()
}

pub fn solve_1(filename: &str) -> Result<String> {
    let pairs = parse_pairs(filename);

    let mut result = vec![];
    let mut index = 1;

    for (left, right) in pairs.iter() {
        let ordering = left.cmp(right);

        match ordering {
            Ordering::Less => {
                result.push(index);
            }
            Ordering::Equal |
            Ordering::Greater => {}
        }

        index += 1;
    }

    let sum: usize = result.iter().sum();
    Ok(sum.to_string())
}


pub fn solve_2(filename: &str) -> Result<String> {
    let mut packets = parse_packets(filename);

    let first_divider: Distress = "[[2]]".parse().unwrap();
    let second_divider: Distress = "[[6]]".parse().unwrap();

    packets.push(first_divider.clone());
    packets.push(second_divider.clone());

    let sorted: Vec<&Distress> = packets.iter().sorted().collect_vec();

    let (first_divider_index, _) = sorted.iter().find_position(|b| **b == &first_divider).unwrap();
    let (second_divider_index, _) = sorted.iter().find_position(|b| **b == &second_divider).unwrap();

    let result = (first_divider_index + 1) * (second_divider_index + 1);

    Ok(result.to_string())
}
