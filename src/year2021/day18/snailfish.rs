use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Snailfish {
    Number(usize),
    Pair(Box<Snailfish>, Box<Snailfish>),
}

impl Snailfish {
    pub fn new_pair(left: usize, right: usize) -> Snailfish {
        Snailfish::Pair(
            Box::new(Snailfish::Number(left)),
            Box::new(Snailfish::Number(right)),
        )
    }

    pub fn add(self, other: Snailfish) -> Snailfish {
        Snailfish::Pair(Box::new(self), Box::new(other))
    }

    pub fn explode(&self) -> (usize, Snailfish, usize) {
        match self {
            Snailfish::Number(_) => panic!("Only leaves can explode"),
            Snailfish::Pair(left, right) => {
                (left.get_number_value().unwrap(), Snailfish::Number(0), right.get_number_value().unwrap())
            }
        }
    }

    pub fn get_number_value(&self) -> Option<usize> {
        match *self {
            Snailfish::Number(value) => Some(value),
            Snailfish::Pair(_, _) => None,
        }
    }

    fn is_leaf(&self) -> bool {
        match &self {
            Snailfish::Number(_) => false,
            Snailfish::Pair(left, right) => {
                // todo: replace with double match?
                let left_number = left.get_number_value();
                let right_number = right.get_number_value();

                left_number.is_some() && right_number.is_some()
            }
        }
    }

    fn add_next(self, value: usize) -> Snailfish {
        match self {
            Snailfish::Number(current) => Snailfish::Number(current + value),
            Snailfish::Pair(left, right) => {
                let left = left.add_next(value);
                Snailfish::Pair(Box::from(left), right)
            }
        }
    }

    fn add_prev(self, value: usize) -> Snailfish {
        match self {
            Snailfish::Number(current) => Snailfish::Number(current + value),
            Snailfish::Pair(left, right) => {
                let right = right.add_prev(value);
                Snailfish::Pair(left, Box::from(right))
            }
        }
    }

    pub fn traverse_and_split(&self, has_splitted: bool) -> (bool, Snailfish) {
        match (has_splitted, self) {
            (true, _) => (has_splitted, self.clone()),
            (false, Snailfish::Number(value)) => {
                if *value >= 10 {
                    (true, Snailfish::create_from_split(*value))
                } else {
                    (has_splitted, self.clone())
                }
            }
            (false, Snailfish::Pair(left, right)) => {
                let (left_has_splitted, left_fish) = left.traverse_and_split(has_splitted);
                let (right_has_splitted, right_fish) = right.traverse_and_split(left_has_splitted);
                let fish = Snailfish::Pair(Box::from(left_fish), Box::from(right_fish));
                (left_has_splitted || right_has_splitted, fish)
            }
        }
    }

    fn create_from_split(value: usize) -> Snailfish {
        let base_value = value / 2_usize;
        let left = Snailfish::Number(base_value);
        let right = if value % 2 == 0 {Snailfish::Number(base_value)} else {Snailfish::Number(base_value + 1)};

        Snailfish::Pair(Box::from(left), Box::from(right))
    }

    pub fn traverse_and_explode(self, level: usize, has_exploded: bool) -> (Option<usize>, Snailfish, Option<usize>, bool) {
        if self.is_leaf() {
            return if level >= 4 {
                let (l_value, fish, r_value) = self.explode();
                (Some(l_value), fish, Some(r_value), true)
            } else {
                (None, self.clone(), None, false)
            };
        }

        match self {
            Snailfish::Number(_) => (None, self, None, false),
            Snailfish::Pair(left, right) => {
                let (prev_external, l_fish, next_internal, left_has_exploded) = left.traverse_and_explode(level + 1, has_exploded);

                let right = if let Some(next) = next_internal {
                    right.add_next(next)
                } else {
                    *right
                };

                let (prev_internal, right_fish, next_external, right_has_exploded) = right.traverse_and_explode(level + 1, has_exploded || left_has_exploded);
                let left_fish = if let Some(prev) = prev_internal {
                    l_fish.add_prev(prev)
                } else {
                    l_fish
                };

                let fish = Snailfish::Pair(Box::from(left_fish), Box::from(right_fish));
                (prev_external, fish, next_external, left_has_exploded || right_has_exploded)
            }
        }
    }

    pub fn parse(s: &str) -> Snailfish {

        if !s.starts_with('[') && !s.ends_with(']'){
            return Snailfish::Number(s.trim().parse().unwrap())
        }

        let stripped = if s.ends_with(',') {
            s.strip_suffix(',').unwrap()
        } else {
            s
        };

        let half_unwrapped = if stripped.starts_with('[') {
            stripped.strip_prefix('[').unwrap()
        } else {
            stripped
        };

        let unwrapped = if half_unwrapped.ends_with(']') {
            half_unwrapped.strip_suffix(']').unwrap()
        } else {
            half_unwrapped
        };

        if unwrapped.chars().all(|c| c.is_numeric()) {
            return Snailfish::Number(unwrapped.parse().unwrap())
        }

        match (unwrapped.starts_with('['), unwrapped.ends_with(']')) {
            (false, false) => {
                let (a, b) = unwrapped.split_once(',').unwrap();
                let a: Snailfish = a.parse().unwrap();
                let b: Snailfish = b.parse().unwrap();
                Snailfish::Pair(Box::from(a), Box::from(b))
            }
            (false, true) => {
                let (a, b) = unwrapped.split_once(',').unwrap();
                let a: Snailfish = a.parse().unwrap();
                let b: Snailfish = Snailfish::parse(b);
                Snailfish::Pair(Box::from(a), Box::from(b))
            }
            (true, false) => {
                let (a, b) = unwrapped.rsplit_once(',').unwrap();
                let a: Snailfish = Snailfish::parse(a);
                let b: Snailfish = b.parse().unwrap();
                Snailfish::Pair(Box::from(a), Box::from(b))
            }
            (true, true) => {
                let mut openings = 0;
                let mut closings = 0;
                let mut fish = None;

                for (i, char) in unwrapped.chars().enumerate() {
                    match char {
                        '[' => openings += 1,
                        ']' => closings += 1,
                        _ => {}
                    }

                    if openings == closings && openings != 0 {
                        let (a, b) = unwrapped.split_at(i+2);
                        let a: Snailfish = Snailfish::parse(a);
                        let b: Snailfish = Snailfish::parse(b);
                        fish = Some(Snailfish::Pair(Box::from(a), Box::from(b)));
                        break;
                    }
                }

                fish.unwrap()
            }
        }
    }

    pub fn calculate_magnitude(&self) -> usize {
        match self {
            Snailfish::Number(value) => *value,
            Snailfish::Pair(left, right) => {
                3*left.calculate_magnitude() + 2*right.calculate_magnitude()
            }
        }
    }
}

impl fmt::Display for Snailfish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match self {
            Snailfish::Number(value) => format!("{:?}", value),
            Snailfish::Pair(left, right) => format!("[{},{}]", left, right),
        };
        write!(f, "{}", printable)
    }
}

impl FromStr for Snailfish {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fish = Snailfish::parse(s);

        Ok(fish)
    }
}

#[cfg(test)]
mod tests {
    use crate::year2021::day18::*;

    #[test]
    fn test_parse() {
        let parameters = vec![
            (
                "[1,2]", Snailfish::new_pair(1, 2)),
            (
                "[1,[2,3]]",
                Snailfish::Pair(
                    Box::from(Snailfish::Number(1)),
                    Box::from(Snailfish::new_pair(2, 3)),
                )
            ),
            (
                "[[1,2],3]",
                Snailfish::Pair(
                    Box::from(Snailfish::new_pair(1, 2)),
                    Box::from(Snailfish::Number(3)),
                )
            ),
            (
                "[[1,2],[3,4]]",
                Snailfish::Pair(
                    Box::from(Snailfish::new_pair(1, 2)),
                    Box::from(Snailfish::new_pair(3, 4)),
                )
            ),
        ];


        for (input, expected) in parameters {
            let pivot = Snailfish::parse(input);
            assert_eq!(pivot, expected)
        }
    }

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
    fn test_traverse_and_split_on_small_fish() {
        let small_fish = Snailfish::Number(5);
        let (has_splitted, fish) = small_fish.traverse_and_split(false);

        let expected = Snailfish::Number(5);
        assert!(!has_splitted);
        assert_eq!(fish, expected);
    }

    #[test]
    fn test_traverse_and_split_on_big_fish() {
        let big_fish = Snailfish::Number(11);
        let (has_splitted, fish) = big_fish.traverse_and_split(false);

        let expected = Snailfish::new_pair(5, 6);
        assert!(has_splitted);
        assert_eq!(fish, expected);
    }

    #[test]
    fn test_traverse_and_split_on_nested_fish() {
        let leaf = Snailfish::new_pair(5, 17);
        let number = Snailfish::Number(3);
        let nested_fish = Snailfish::Pair(Box::from(number), Box::from(leaf));

        let (has_splitted, fish) = nested_fish.traverse_and_split(false);

        let expected = Snailfish::Pair(
            Box::from(Snailfish::Number(3)),
            Box::from(
                Snailfish::Pair(
                    Box::from(Snailfish::Number(5)),
                    Box::from(
                        Snailfish::Pair(
                            Box::from(Snailfish::Number(8)),
                            Box::from(Snailfish::Number(9)),
                        )
                    ),
                )
            ),
        );
        assert!(has_splitted);
        assert_eq!(fish, expected);
    }
}
