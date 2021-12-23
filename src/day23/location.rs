use std::fmt;
use crate::day23::Amphipod;
use crate::day23::location::Location::Hallway;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Location {
    // todo: change to Option<Amphipod>?
    Hallway(Vec<Amphipod>, usize),
    Room(Vec<Amphipod>, Amphipod, usize),
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Location::Hallway(pods, pos) => {
                if let Some(pod) = pods.first() {
                    write!(f, "Hall {}: {}", pos, pod)
                } else {
                    write!(f, "Hall {}: Ø", pos)
                }
            }
            Location::Room(pods, target, _) => {
                let target = match target {
                    Amphipod::Amber(_) => "A",
                    Amphipod::Bronze(_) => "B",
                    Amphipod::Copper(_) => "C",
                    Amphipod::Dessert(_) => "D"
                };

                match pods.len() {
                    0 => write!(f, "Room {}: Ø", target),
                    1 => write!(f, "Room {}: {}", target, pods.first().unwrap()),
                    2 => write!(f, "Room {}: {}, {}", target, pods.first().unwrap(), pods.last().unwrap()),
                    _ => write!(f, "THIS IS A BUG")
                }
            }
        }
    }
}

impl Location {
    pub fn new_hallway(position: usize) -> Location {
        Hallway(vec![], position)
    }
    // todo: cleanup
    pub fn new_room(bottom: char, top: char, target: char, position: usize) -> Location {
        let bottom = bottom.to_string().parse().unwrap();
        let top = top.to_string().parse().unwrap();
        let target = target.to_string().parse().unwrap();

        Location::Room(vec![bottom, top], target, position)
    }

    pub fn available(&self, pod: &Amphipod) -> bool {
        match self {
            Hallway(_, _) => self.available_spaces() > 0 && !pod.has_moved(),
            Location::Room(_, target, _) => {
                self.available_spaces() > 0 && (*pod == *target)
            }
        }
    }

    pub fn available_spaces(&self) -> usize {
        match self {
            Location::Hallway(pods, _) => 1 - pods.len(),
            Location::Room(pods, _, _) => 2 - pods.len(),
        }
    }

    pub fn position(&self) -> usize {
        match self {
            Location::Hallway(_, pos) => *pos,
            Location::Room(_, _, pos) => *pos,
        }
    }

    pub fn get_score(&self) -> usize {
        match self {
            Hallway(_, _) => 0,
            Location::Room(pods, target, _) => {
                if *target != *pods.first().unwrap() {
                    panic!("Should not happen")
                }

                pods.iter().map(|pod| pod.cost()).sum()
            }
        }
    }

    pub fn push_and_update(&mut self, mut pod: Amphipod, accumulated_steps: usize) -> usize {
        if self.available_spaces() == 0 {
            panic!("Should not happen");
        }

        let spaces = self.available_spaces();
        match self {
            Location::Hallway(pods, _) => {
                pod.add_steps(accumulated_steps);
                pods.push(pod);
                0
            }
            Location::Room(pods, _, _) => {
                pod.add_steps(accumulated_steps + spaces);
                pods.push(pod);
                self.available_spaces() + 1
            }
        };

        0
    }

    pub fn pop(&mut self) -> (Option<Amphipod>, usize) {
        println!("Before pop: {}", self.clone());
        match self {
            Location::Hallway(pods, _) => (pods.pop(), 0),
            Location::Room(pods, _, _) => {
                let steps = 3 - pods.len();

                let popped = pods.pop();
                println!("Popped: {:?}", popped);
                println!("After pop: {}", &self);

                (popped, steps)
            }
        }
    }

    pub fn peek(&self) -> Option<&Amphipod> {
        match self {
            Location::Hallway(pods, _) => pods.last(),
            Location::Room(pods, _, _) => pods.last(),
        }
    }

    pub fn bottom_is_correct(&self) -> bool {
        match self {
            Location::Hallway(_, _) => false,
            Location::Room(pods, target, _) => {
                if let Some(bottom) = pods.first() {
                    *bottom == *target
                } else {
                    false
                }
            }
        }
    }

    pub fn current_occupancy_is_correct(&self) -> bool {
        match self {
            Location::Hallway(_, _) => false,
            Location::Room(pods, target, _) => {
                if let Some(top) = pods.last() {
                    self.bottom_is_correct() && *top == *target
                } else {
                    // todo: should be redundant
                    self.bottom_is_correct()
                }
            }
        }
    }

    pub fn has_movable(&self) -> bool {
        if let Some(pod) = self.peek() {
            // todo: double-op but should be correct
            !self.current_occupancy_is_correct() && pod.is_movable()
        } else {
            false
        }
    }




    /*
        pub fn move_pod(&mut self, target: &mut Location) {

            if !self.has_movable() {
                panic!("Should not happen")
            }

            if !target.available_spaces() == 0 {
                panic!("Cannot move into full space")
            }

            let ((pod, steps), from) = match &self {
                Location::Hallway(pods, position) => (self.pop(), position),
                Location::Room(pods, _, position) => (self.pop(), position),
            };

            let pod = pod.expect("Already checked");
        }

     */
}
