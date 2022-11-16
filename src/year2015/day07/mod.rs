use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use anyhow::Result;
use fancy_regex::Regex;
use itertools::Itertools;

use crate::utils::get_input;

pub fn solve_1(filename: &str) -> Result<String> {
    let symbols: Vec<CircuitSymbol> = get_input(filename);

    let symbol_queue = VecDeque::from_iter(symbols.into_iter());
    let resolution = resolve_symbols(symbol_queue);

    Ok(resolved_to_string(&resolution))
}

fn resolved_to_string(resolved: &HashMap<String, u16>) -> String {
    resolved
        .iter()
        .map(|(key, value)| key.to_owned() + ": " + value.to_string().as_str())
        .sorted()
        .join(", ")
}

fn resolve_symbols(mut symbols: VecDeque<CircuitSymbol>) -> HashMap<String, u16> {
    let mut resolved = HashMap::new();

    while let Some(symbol) = symbols.pop_front() {
        match resolve_symbol(&resolved, symbol) {
            CircuitSymbol::Value { name, value } => {
                resolved.insert(name, value);
            }
            CircuitSymbol::Unary { name, operator, operand } => {
                symbols.push_back(CircuitSymbol::Unary { name, operator, operand });
            }
            CircuitSymbol::Ternary { name, left, operator, right } => {
                symbols.push_back(CircuitSymbol::Ternary { name, left, operator, right });
            }
        }
    };

    resolved
}


fn resolve_symbol(resolved: &HashMap<String, u16>, symbol: CircuitSymbol) -> CircuitSymbol {
    match symbol {
        CircuitSymbol::Value { .. } => {
            symbol
        }
        CircuitSymbol::Unary { name, operator, operand } => {
            apply_unary(resolved, name, operator, operand)
        }
        CircuitSymbol::Ternary { name, operator, left, right } => {
            apply_ternary(resolved, name, operator, left, right)
        }
    }
}

fn apply_unary(resolved: &HashMap<String, u16>, name: String, operator: CircuitOperator, operand: CircuitValue) -> CircuitSymbol {
    match operand {
        CircuitValue::Number(value) => {
            CircuitSymbol::Value { name, value: apply(operator, None, value) }
        }
        CircuitValue::Variable(variable_name) => {
            if let Some(value) = resolve_name(resolved, &variable_name) {
                CircuitSymbol::Value { name, value: apply(operator, None, value) }
            } else {
                CircuitSymbol::Unary { name, operator, operand: CircuitValue::Variable(variable_name) }

            }
        }
    }
}

fn resolve_circuit(resolved: &HashMap<String, u16>, value: &CircuitValue) -> Option<u16> {
    match value {
        CircuitValue::Number(number) => Some(*number),
        CircuitValue::Variable(name) => resolve_name(resolved, name)
    }
}

fn resolve_name(resolved: &HashMap<String, u16>, name: &str) -> Option<u16> {
    resolved.get(name).copied()
}

fn apply_ternary(resolved: &HashMap<String, u16>, name: String, operator: CircuitOperator, left: CircuitValue, right: CircuitValue) -> CircuitSymbol {
    let l = resolve_circuit(resolved, &left);
    let r = resolve_circuit(resolved, &right);

    match (l, r) {
        (None, None) => CircuitSymbol::Ternary { name, operator, left, right },
        (Some(l_value), None) => CircuitSymbol::Ternary { name, operator, left: CircuitValue::Number(l_value), right },
        (None, Some(r_value)) => CircuitSymbol::Ternary { name, operator, left, right: CircuitValue::Number(r_value) },
        (Some(l_value), Some(r_value)) => {
            CircuitSymbol::Value { name, value: apply(operator, Some(l_value), r_value) }
        }
    }
}

fn apply(operator: CircuitOperator, left: Option<u16>, right: u16) -> u16 {
    match operator {
        CircuitOperator::Assignment => right,
        CircuitOperator::And => left.unwrap() & right,
        CircuitOperator::Or => left.unwrap() | right,
        CircuitOperator::Not => !right,
        CircuitOperator::Lshift => left.unwrap() << right,
        CircuitOperator::Rshift => left.unwrap() >> right,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CircuitValue {
    Number(u16),
    Variable(String),
}

impl FromStr for CircuitValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = if is_numeric(s) {
            CircuitValue::Number(s.parse().unwrap())
        } else {
            CircuitValue::Variable(s.to_string())
        };

        Ok(value)
    }
}

// fixme: utility
fn is_numeric(s: &str) -> bool {
    s.trim().chars().all(|c| c.is_numeric())
}

pub fn solve_2(filename: &str) -> Result<String> {
    let symbols: Vec<CircuitSymbol> = get_input(filename);

    let updated_symbols = symbols
        .into_iter()
        .filter(|s| s.get_name() != "b");

    let mut symbol_queue = VecDeque::from_iter(updated_symbols);
    symbol_queue.push_front(CircuitSymbol::Unary {
        name: "b".to_string(),
        operator: CircuitOperator::Assignment,
        operand: CircuitValue::Number(956),
    });

    let resolution = resolve_symbols(symbol_queue);

    Ok(resolved_to_string(&resolution))
}


#[derive(Debug, PartialEq, Eq)]
pub enum CircuitSymbol {
    Value { name: String, value: u16 },
    Unary { name: String, operator: CircuitOperator, operand: CircuitValue },
    Ternary { name: String, left: CircuitValue, operator: CircuitOperator, right: CircuitValue },
}

impl CircuitSymbol {
    pub fn get_name(&self) -> &String {
        match self {
            CircuitSymbol::Value { name, .. } => name,
            CircuitSymbol::Unary { name, .. } => name,
            CircuitSymbol::Ternary { name, .. } => name,
        }
    }
}


impl FromStr for CircuitSymbol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"([\w\s]+)\s->\s(\w+)").unwrap();
        let captures = re.captures(s).unwrap().unwrap();

        let symbol = captures.get(1).unwrap().as_str();
        let name = captures.get(2).unwrap().as_str();

        let re_ternary = Regex::new(r"([0-9a-z]+)\s([A-Z]+)\s([0-9a-z]+)").unwrap();
        let re_unary = Regex::new(r"([A-Z]+)\s([0-9a-z]+)").unwrap();

        let circuit_symbol = if re_ternary.is_match(symbol).unwrap() {
            let ternary_captures = re_ternary.captures(symbol).unwrap().unwrap();

            CircuitSymbol::Ternary {
                name: name.to_string(),
                left: ternary_captures.get(1).unwrap().as_str().parse().unwrap(),
                operator: ternary_captures.get(2).unwrap().as_str().parse().unwrap(),
                right: ternary_captures.get(3).unwrap().as_str().parse().unwrap(),
            }
        } else if re_unary.is_match(symbol).unwrap() {
            let unary_captures = re_unary.captures(symbol).unwrap().unwrap();

            CircuitSymbol::Unary {
                name: name.to_string(),
                operator: unary_captures.get(1).unwrap().as_str().parse().unwrap(),
                operand: unary_captures.get(2).unwrap().as_str().parse().unwrap(),
            }
        } else if is_numeric(symbol) {
            CircuitSymbol::Value { name: name.to_string(), value: symbol.parse().unwrap() }
        } else {
            CircuitSymbol::Unary { name: name.to_string(), operator: CircuitOperator::Assignment, operand: symbol.parse().unwrap() }
        };

        Ok(circuit_symbol)
    }
}


#[derive(Debug, PartialEq, Eq)]
pub enum CircuitOperator {
    Assignment,
    And,
    Or,
    Not,
    Lshift,
    Rshift,
}

impl FromStr for CircuitOperator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(CircuitOperator::And),
            "OR" => Ok(CircuitOperator::Or),
            "NOT" => Ok(CircuitOperator::Not),
            "LSHIFT" => Ok(CircuitOperator::Lshift),
            "RSHIFT" => Ok(CircuitOperator::Rshift),
            _ => Err(())
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::year2015::day07::*;
    use crate::year2015::day07::CircuitOperator::Not;

    #[test]
    fn test_parse() {
        let a = "NOT go -> gp";

        let actual: CircuitSymbol = a.parse().unwrap();
        let expected = CircuitSymbol::Unary { name: "gp".to_string(), operator: Not, operand: CircuitValue::Variable("go".to_string()) };
        assert_eq!(actual, expected);
    }
}