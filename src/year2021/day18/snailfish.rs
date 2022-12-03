use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Snailfish {
    Number(usize),
    Pair(Box<Snailfish>, Box<Snailfish>),
}

impl Snailfish {

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
                left.get_number_value().is_some() && right.get_number_value().is_some()
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

        let stripped = s.strip_suffix(',').unwrap_or(s);
        let half_unwrapped = stripped.strip_prefix('[').unwrap_or(stripped);
        let unwrapped = half_unwrapped.strip_suffix(']').unwrap_or(half_unwrapped);


        let fish = if unwrapped.parse::<f64>().is_ok() {
            Snailfish::Number(unwrapped.parse().unwrap())
        } else {
            match (unwrapped.starts_with('['), unwrapped.ends_with(']')) {
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
                            let a: Snailfish = a.parse()?;
                            let b: Snailfish = b.parse()?;
                            fish = Some(Snailfish::Pair(Box::from(a), Box::from(b)));
                            break;
                        }
                    }

                    fish.unwrap()
                }
                (true, false) => {
                    let (a, b) = unwrapped.rsplit_once(',').unwrap();
                    let a: Snailfish = a.parse()?;
                    let b: Snailfish = b.parse()?;
                    Snailfish::Pair(Box::from(a), Box::from(b))
                }
                _ => {
                    let (a, b) = unwrapped.split_once(',').unwrap();
                    let a: Snailfish = a.parse()?;
                    let b: Snailfish = b.parse()?;
                    Snailfish::Pair(Box::from(a), Box::from(b))
                }
            }
        };

        Ok(fish)
    }
}
