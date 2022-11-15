use std::cmp::min;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use fancy_regex::Regex;
use crate::utils::get_input;

/*
struct RaceResult {
    reindeer: Reindeer,
    state: ReindeerState,
    distance: usize,
    points: usize,
}



enum ReindeerState {
    Resting(usize),
    Flying(usize),
}
*/

struct Reindeer {
    name: String,
    speed: usize,
    flying_time: usize,
    resting_time: usize,
}

impl Reindeer {
    fn fly(&self, duration: usize) -> usize {
        let mut elapsed = 0;
        let mut distance = 0;

        let lap_time = self.flying_time + self.resting_time;

        while elapsed + lap_time < duration {
            distance += self.speed * self.flying_time;
            elapsed += lap_time;
        };

        let remaining = min(duration - elapsed, self.flying_time);
        distance += remaining * self.speed;

        distance
    }
}

impl Display for Reindeer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} km/s, {} s flying, {} s resting", self.name, self.speed, self.flying_time, self.resting_time)
    }
}

impl FromStr for Reindeer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r#"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds."#
        ).unwrap();

        let capture = re.captures(s).unwrap().unwrap();

        Ok(Reindeer {
            name: capture.get(1).unwrap().as_str().to_string(),
            speed: capture.get(2).unwrap().as_str().parse().unwrap(),
            flying_time: capture.get(3).unwrap().as_str().parse().unwrap(),
            resting_time: capture.get(4).unwrap().as_str().parse().unwrap(),
        })
    }
}

const COMPETITION_TIME: usize = 2503;

pub fn solve_1(filename: &str) -> String {
    let herd: Vec<Reindeer> = get_input(filename);

    herd
        .iter()
        .map(|reindeer| reindeer.fly(COMPETITION_TIME))
        .max()
        .unwrap()
        .to_string()
}

pub fn solve_2(filename: &str) -> String {
    let _herd: Vec<Reindeer> = get_input(filename);

    for _tick in 0..COMPETITION_TIME {}

    todo!()
}