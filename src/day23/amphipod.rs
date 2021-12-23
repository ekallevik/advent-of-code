use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::day23;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Amphipod {
    Amber(Vec<usize>),
    Bronze(Vec<usize>),
    Copper(Vec<usize>),
    Dessert(Vec<usize>),
}

impl day23::Amphipod {
    pub fn cost(&self) -> usize {

        match self {
            Amphipod::Amber(moves) => (*moves).iter().sum::<usize>(),
            Amphipod::Bronze(moves) => 10*(*moves).iter().sum::<usize>(),
            Amphipod::Copper(moves) => 100*(*moves).iter().sum::<usize>(),
            Amphipod::Dessert(moves) => 1000*(*moves).iter().sum::<usize>(),
        }
    }

    pub fn is_movable(&self) -> bool {

        let can_move = |moves: &[usize]| moves.len() < 2;

        match self {
            Amphipod::Amber(moves) => can_move(moves),
            Amphipod::Bronze(moves) => can_move(moves),
            Amphipod::Copper(moves) => can_move(moves),
            Amphipod::Dessert(moves) => can_move(moves),
        }
    }

    pub fn has_moved(&self) -> bool {

        match self {
            Amphipod::Amber(moves) => !moves.is_empty(),
            Amphipod::Bronze(moves) => !moves.is_empty(),
            Amphipod::Copper(moves) => !moves.is_empty(),
            Amphipod::Dessert(moves) => !moves.is_empty(),
        }
    }

    pub fn add_steps(&mut self, steps: usize) {
        match self {
            Amphipod::Amber(moves) => moves.push(steps),
            Amphipod::Bronze(moves) => moves.push(steps),
            Amphipod::Copper(moves) => moves.push(steps),
            Amphipod::Dessert(moves) => moves.push(steps),
        }
    }
}

impl FromStr for Amphipod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Amphipod::Amber(vec![])),
            "B" => Ok(Amphipod::Bronze(vec![])),
            "C" => Ok(Amphipod::Copper(vec![])),
            "D" => Ok(Amphipod::Dessert(vec![])),
            _ => Result::Err(())
        }
    }
}

impl Display for Amphipod {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Amphipod::Amber(moves) => write!(f, "Amber {:?}", moves),
            Amphipod::Bronze(moves) => write!(f, "Bronze {:?}", moves),
            Amphipod::Copper(moves) => write!(f, "Copper {:?}", moves),
            Amphipod::Dessert(moves) => write!(f, "Dessert {:?}", moves)
        }
    }
}