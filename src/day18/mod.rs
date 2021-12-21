use crate::utils::get_input;
use std::borrow::Borrow;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Snailfish {
    Number(usize),
    Pair(Box<Snailfish>, Box<Snailfish>),
}

impl Snailfish {
    // todo: Use option to create a more generic?
    // todo: or remove number?
    // todo: improve error handling
    fn new_pair(left: usize, right: usize) -> Snailfish {
        Snailfish::Pair(
            Box::new(Snailfish::Number(left)),
            Box::new(Snailfish::Number(right)),
        )
    }

    fn add(self, other: Snailfish) -> Snailfish {
        Snailfish::Pair(Box::new(self), Box::new(other))
    }

    fn contains(self, other: Snailfish) -> bool {
        match self {
            Snailfish::Number(_) => false,
            Snailfish::Pair(left, right) => {
                let left: &Snailfish = left.borrow();
                let right: &Snailfish = right.borrow();

                *left == other || other == *right
            }
        }
    }
    /*
       fn explode(mut self) -> Option<(usize, usize)> {
           match self {
               Snailfish::Number(_) => None
               Snailfish::Pair(left, right) => {
                   self = Snailfish::Number(0)
                   Some((left, right))
               }
           };

       }

    */

    fn get_number_value(self) -> usize {
        match self {
            Snailfish::Number(value) => value,
            Snailfish::Pair(_, _) => panic!("Should not happen"),
        }
    }
}

impl fmt::Display for Snailfish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match self {
            Snailfish::Number(value) => format!("{:?}", value),
            Snailfish::Pair(left, right) => format!("[{}, {}]", left, right),
        };
        write!(f, "{}", printable)
    }
}

impl FromStr for Snailfish {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair: (&str, &str) = s.split_once(",").unwrap();

        let left = pair.0.chars().nth(1).unwrap();
        let left: usize = left.to_digit(10).unwrap() as usize;

        let right = pair.1.chars().next().unwrap();
        let right: usize = right.to_digit(10).unwrap() as usize;

        Ok(Snailfish::new_pair(left, right))
    }
}

pub fn solve_1(filename: &str) -> String {
    let input: Vec<Snailfish> = get_input(filename);

    let result = input.into_iter().reduce(|acc, elem| acc.add(elem)).unwrap();

    println!("{}", result);

    filename.to_string()
}

pub fn solve_2(filename: &str) -> String {
    filename.to_string()
}

/*
fn explode(exploding: Snailfish, left: Option<Snailfish>, right: Option<Snailfish>) -> Snailfish {

    match exploding {
        Snailfish::Number(_) => panic!("Should never happen"),
        Snailfish::Pair(left_number, right_number) => {
            let left_number = left_number.get_number_value();
            let right_number = right_number.get_number_value();

            let new_left = if let Some(left) = left {
                // add left
                match left {
                    Snailfish::Number(_) => None
                    Snailfish::Pair(current_left, _) =>
                }


            } else {None};

            if let Some(right) = right {
                // add right
            }

        }
    };


    Snailfish::Number(1)
}

 */

#[cfg(test)]
mod tests {
    use super::*;
    
    

    #[test]
    fn test_add_snailfish_literal_pairs() {
        let left = Snailfish::new_pair(1, 1);
        let right = Snailfish::new_pair(2, 2);

        let added = left.add(right);
        let expected = Snailfish::Pair(
            Box::new(Snailfish::new_pair(1, 1)),
            Box::new(Snailfish::new_pair(2, 2)),
        );

        assert_eq!(added, expected)
    }

    #[test]
    fn test_add_snailfish_pairs() {
        let left = Snailfish::Pair(
            Box::new(Snailfish::new_pair(1, 1)),
            Box::new(Snailfish::new_pair(2, 2)),
        );
        let right = Snailfish::new_pair(3, 3);

        let added = left.add(right);
        let expected = Snailfish::Pair(
            Box::new(Snailfish::Pair(
                Box::new(Snailfish::new_pair(1, 1)),
                Box::new(Snailfish::new_pair(2, 2)),
            )),
            Box::new(Snailfish::new_pair(3, 3)),
        );

        assert_eq!(added, expected)
    }

    #[test]
    fn test_snailfish_contains_other() {
        let snailfish = Snailfish::Pair(
            Box::new(Snailfish::new_pair(1, 1)),
            Box::new(Snailfish::new_pair(2, 2)),
        );
        let other = Snailfish::new_pair(2, 2);

        let expected = snailfish.contains(other);

        assert!(expected)
    }
}
