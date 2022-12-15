use std::collections::{HashMap};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use anyhow::{bail, Result};
use scan_fmt::scan_fmt;

use crate::utils::get_input;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position(isize, isize);

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:2}, {:2})", self.0, self.1)
    }
}

impl Position {
    fn get_manhattan(&self, other: &Position) -> isize {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

#[derive()]
struct Signal {
    sensor: Position,
    beacon: Position,
}

impl Signal {
    fn get_reach(&self) -> isize {
        self.sensor.get_manhattan(&self.beacon)
    }

    fn get_x_bounds(&self) -> (isize, isize) {
        if self.beacon.0 > self.sensor.0 {
            (self.sensor.0, self.beacon.0)
        } else {
            (self.beacon.0, self.sensor.0)
        }
    }

    fn get_y_bounds(&self) -> (isize, isize) {
        if self.beacon.1 > self.sensor.1 {
            (self.sensor.1, self.beacon.1)
        } else {
            (self.beacon.1, self.sensor.1)
        }
    }
}

enum Device {
    Sensor,
    Beacon,
    Void,
}

impl Display for Device {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Device::Sensor => write!(f, "S"),
            Device::Beacon => write!(f, "B"),
            Device::Void => write!(f, "#"),
        }
    }
}

impl FromStr for Signal {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (s_x, s_y, b_x, b_y) = scan_fmt!(s, "Sensor at x={}, y={}: closest beacon is at x={}, y={}", isize, isize, isize, isize).unwrap();
        let sensor = Position(s_x, s_y);
        let beacon = Position(b_x, b_y);
        Ok(Signal {
            sensor,
            beacon,
        })
    }
}

pub fn solve_1(filename: &str) -> Result<String> {
    let signals: Vec<Signal> = get_input(filename);

    let mut grid = HashMap::new();

    let target_row = if filename.contains("real") {
        2000000
    } else {
        10
    };

    let mut min_width = signals[0].sensor.0;
    let mut max_width = signals[0].sensor.0;

    for signal in signals {
        println!("Adding sensor at {}", signal.sensor);
        let sensor = signal.sensor;
        let beacon = signal.beacon;

        let sensor_reach = sensor.get_manhattan(&beacon) as isize;
        let target_distance = sensor.get_manhattan(&Position(sensor.0, target_row));

        if sensor_reach >= target_distance {
            let delta_y = (target_row - sensor.1).abs();
            let x_manhattan = (sensor_reach - delta_y).abs();

            let min_sensor_x = sensor.0 - x_manhattan;
            let max_sensor_x = sensor.0 + x_manhattan;

            if min_sensor_x < min_width {
                min_width = min_sensor_x;
            }

            if max_sensor_x > max_width {
                max_width = max_sensor_x;
            }

            for sensor_x in min_sensor_x..=max_sensor_x {
                let position = Position(sensor_x, target_row);
                grid.insert(position, Device::Void);
            }
        }

        grid.insert(sensor, Device::Sensor);
        grid.insert(beacon, Device::Beacon);
    }

    let mut count = 0;
    for x in min_width..=max_width {
        if let Some(device) = grid.get(&Position(x, target_row)) {
            match device {
                Device::Beacon => {}
                Device::Sensor | Device::Void => { count += 1 }
            }
        }
    }

    Ok(count.to_string())
}



pub fn solve_2(filename: &str) -> Result<String> {
    let signals: Vec<Signal> = get_input(filename);

    let real_limit = 4_000_000;

    let lower = 0;
    let upper = if filename.contains("real") {
        real_limit
    } else {
        20
    };

    for y in lower..=upper {
        let mut line = Line::new();

        for signal in &signals {
            let reach = signal.get_reach();
            let sensor = signal.sensor.clone();

            let nearest_row_position = Position(sensor.0, y);
            let row_distance = sensor.get_manhattan(&nearest_row_position);

            if row_distance <= reach {
                let delta_y = (y as isize - sensor.1).abs();
                let x_manhattan = (reach - delta_y).abs();

                let min_sensor_x = lower.max(sensor.0 - x_manhattan);
                let max_sensor_x = upper.min(sensor.0 + x_manhattan);

                let x_segment = Segment(min_sensor_x, max_sensor_x);

                line.add(x_segment);
            }
        }

        line.collapse();
        if line.0.len() != 1 {
            let answer = real_limit * line.get_gap() + y;
            return Ok(answer.to_string());
        }
    }

    bail!("Could not find a solution")
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
struct Segment(isize, isize);

impl Segment {
    fn connected(&self, other: &Segment) -> bool {
        ((self.0 - 1)..=self.1 + 1).contains(&other.0)
            || ((self.0 - 1)..=self.1 + 1).contains(&other.1)
    }

    fn combine(&self, other: &Segment) -> Segment {
        let min = self.0.min(other.0);
        let max = self.1.max(other.1);
        Segment(min, max)
    }

    fn is_complete(&self, upper: isize) -> bool {
        self.0 == 0 && self.1 >= upper
    }


}


struct Line(Vec<Segment>);

impl Line {
    fn new() -> Self {
        Line(vec![])
    }

    fn get_gap(&mut self) -> isize {
        let mut segments = &mut self.0;
        segments.sort();
        segments[0].1 + 1
    }

    fn collapse(&mut self) {
        while self.reduce() {}
    }

    fn reduce(&mut self) -> bool {
        if self.0.len() <= 1 {
            return false;
        }

        self.0.sort();

        let first = self.0.pop().unwrap();
        let second = self.0.pop().unwrap();

        if first.connected(&second) || second.connected(&first) {
            self.0.push(first.combine(&second));
            true
        } else {
            self.0.push(first);
            self.0.push(second);
            false
        }
    }

    fn is_complete(&mut self, limit: isize) -> bool {
        let first = &self.0[0];
        let second = &self.0[1];

        let combined = first.combine(second);

        combined.0 == 0 && combined.1 >= limit
    }

    fn add(&mut self, segment: Segment) {

        //self.0.push(segment);
        //self.collapse();

        let mut has_combined = false;
        self.0.sort();

        for (i, existing) in self.0.iter().enumerate() {
            if segment.connected(&existing) || existing.connected(&segment) {
                self.0[i] = segment.combine(&existing);

                if let Some(next) = self.0.get(i + 1) {
                    if segment.connected(next) {
                        self.0[i] = self.0[i].combine(next);
                        self.0.remove(i + 1);
                    }
                }

                has_combined = true;
                break;
            }
        }

        if !has_combined {
            self.0.push(segment);
        }


    }
}