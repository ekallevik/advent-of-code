use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Symbol {
    Variable(char),
    Value(isize),
}

impl FromStr for Symbol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let symbol = match s {
            "w" => Self::Variable('w'),
            "x" => Self::Variable('x'),
            "y" => Self::Variable('y'),
            "z" => Self::Variable('z'),
            _   => Self::Value(s.parse().unwrap()),
        };

        Ok(symbol)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Inp(char),
    Add(char, Symbol),
    Mul(char, Symbol),
    Div(char, Symbol),
    Mod(char, Symbol),
    Eql(char, Symbol),
}

impl Instruction {

    pub fn apply(&self, registry: &mut [isize; 4], input: &mut VecDeque<u32>) {

        // todo: store this value in A?
        let index = |char: &char| -> usize {
            match char {
                'w' => 0,
                'x' => 1,
                'y' => 2,
                'z' => 3,
                _ => panic!("Should not happen")
            }
        };
        let get = |c: &char| -> isize {registry[index(c)]};

        match &self {
            Instruction::Inp(a) => {
                // todo: maybe use indexing here as well
                registry[index(a)] = input.pop_front().unwrap() as isize;
            }
            Instruction::Add(a, b) => {
                let b_value = match b {
                    Symbol::Variable(c) => get(c),
                    Symbol::Value(v) => *v
                };

                registry[index(a)] = get(a) + b_value
            }
            Instruction::Mul(a, b) => {

                let b_value = match b {
                    Symbol::Variable(c) => get(c),
                    Symbol::Value(v) => *v
                };

                registry[index(a)] = get(a) * b_value
            }
            Instruction::Div(a, b) => {

                let b_value = match b {
                    Symbol::Variable(c) => get(c),
                    Symbol::Value(v) => *v
                };

                registry[index(a)] = get(a) / b_value
            }
            Instruction::Mod(a, b) => {
                let b_value = match b {
                    Symbol::Variable(c) => get(c),
                    Symbol::Value(v) => *v
                };

                registry[index(a)] = get(a) % b_value
            }
            Instruction::Eql(a, b) => {

                let b_value = match b {
                    Symbol::Variable(c) => get(c),
                    Symbol::Value(v) => *v
                };

                registry[index(a)] = if get(a) == b_value {1} else {0};
            }
        }
    }


}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let (instruction, operands) = s.split_once(' ').unwrap();
        let operands = operands.chars().collect::<Vec<char>>();
        let first = *operands.first().unwrap();
        let second: String = (*operands.last().unwrap()).to_string();

        let i = match instruction {
            "inp" => Instruction::Inp(first),
            "add" => Instruction::Add(first, second.parse::<Symbol>().unwrap()),
            "mul" => Instruction::Mul(first, second.parse::<Symbol>().unwrap()),
            "div" => Instruction::Div(first, second.parse::<Symbol>().unwrap()),
            "mod" => Instruction::Mod(first, second.parse::<Symbol>().unwrap()),
            "eql" => Instruction::Eql(first, second.parse::<Symbol>().unwrap()),
            _ => unreachable!("Reached: {}", instruction)
        };

        Ok(i)
    }
}

/*

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, VecDeque};
    use crate::day24::instruction::{Instruction, Symbol};

    #[test]
    fn test_read_input() {

        let mut registry = HashMap::new();
        let instruction = Instruction::Inp('w');
        let mut input: VecDeque<u32> = VecDeque::new();
        input.push_back(8);

        instruction.apply(&mut registry, &mut input);
        let mut expected: HashMap<char, Symbol> = HashMap::new();
        expected.insert('w', Symbol::Value(8));

        assert_eq!(registry, expected)
    }

    #[test]
    fn test_add() {

        let mut registry = HashMap::new();
        let instructions = vec![
            Instruction::Inp('w'),
            Instruction::Add('w', Symbol::Value(4)),
        ];
        let mut input: VecDeque<u32> = VecDeque::new();
        input.push_back(8);

        for op in instructions {
            op.apply(&mut registry, &mut input)
        }

        let mut expected: HashMap<char, Symbol> = HashMap::new();
        expected.insert('w', Symbol::Value(12));

        assert_eq!(registry, expected)
    }

    #[test]
    fn test_mul() {

        let mut registry = HashMap::new();
        let instructions = vec![
            Instruction::Inp('w'),
            Instruction::Mul('w', Symbol::Value(-1)),
        ];
        let mut input = VecDeque::new();
        input.push_back(8);

        for op in instructions {
            op.apply(&mut registry, &mut input)
        }

        let mut expected: HashMap<char, Symbol> = HashMap::new();
        expected.insert('w', Symbol::Value(-8));

        assert_eq!(registry, expected)
    }

    #[test]
    fn test_second_is_3x_first() {

        let mut registry = HashMap::new();
        let instructions = vec![
            Instruction::Inp('z'),
            Instruction::Inp('x'),
            Instruction::Mul('z', Symbol::Value(-1)),
            Instruction::Eql('z', Symbol::Variable('x')),
        ];
        let mut input = VecDeque::new();
        input.push_back(3);
        input.push_back(9);

        for op in instructions {
            op.apply(&mut registry, &mut input)
        }

        let mut expected: HashMap<char, Symbol> = HashMap::new();
        expected.insert('x', Symbol::Value(9));
        expected.insert('z', Symbol::Value(0));

        assert_eq!(registry, expected)
    }
}

 */