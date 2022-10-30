use std::cmp::{max, min};
use std::fmt;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Line(isize, isize);

impl Line {

    pub fn new(start: isize, end: isize) -> Option<Line> {
        match start <= end {
            true => Some(Line(start, end)),
            false => None
        }
    }

    pub fn length(&self) -> usize {
        (self.1 - self.0 + 1) as usize
    }

    pub fn as_range(&self) -> RangeInclusive<isize> {
        self.0..=self.1
    }

    pub fn contains(&self, point: isize) -> bool {
        self.0 <= point && point <= self.1
    }

    pub fn overlaps(&self, other: &Line) -> bool {
        (self.contains(other.0)
            || self.contains(other.1))
            || (other.contains(self.0)
            || other.contains(self.1))
    }

    pub fn intersection(&self, other: &Line) -> Option<Line> {
        let start = max(self.0, other.0);
        let end = min(self.1, other.1);

        Line::new(start, end)
    }

    // todo: change to tuples?
    pub fn diff(&self, other: &Line) -> Vec<Line> {

        let intersection = self.intersection(other);

        if let Some(intersection) = intersection {
            let diffs = vec![
                Line::new(self.0, intersection.0-1),
                Line::new(intersection.1+1, self.1)
            ];

            diffs
                .into_iter()
                .filter_map(|line| line)
                .collect()
        } else {
            vec![]
        }


    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:>3}, {:>3})", self.0, self.1)
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once("..").unwrap();

        Ok(Line::new(start.parse::<isize>().unwrap(), end.parse::<isize>().unwrap()).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::line::Line;
    #[test]
    fn test_line_intersection() {

        let line = Line(0, 10);
        let other = Line(5, 15);

        let intersection = line.intersection(&other).unwrap();
        let expected = Line(5, 10);

        assert_eq!(intersection, expected)

    }

    #[test]
    fn test_line_left_diff() {
        let line = Line(0, 10);
        let other = Line(5, 10);

        let diff = line.diff(&other);
        let expected = vec![Line(0, 5)];

        assert_eq!(diff, expected);
    }

    #[test]
    fn test_line_right_diff() {
        let line = Line(0, 10);
        let other = Line(0, 5);

        let diff = line.diff(&other);
        let expected = vec![Line(5, 10)];

        assert_eq!(diff, expected);
    }

    #[test]
    fn test_line_diff_against_superset() {
        let line = Line(0, 10);
        let other = Line(-20, 20);

        let diff = line.diff(&other);
        let expected = vec![];

        assert_eq!(diff, expected);
    }

    #[test]
    fn test_line_diff_against_subset() {
        let line = Line(0, 10);
        let other = Line(2, 8);

        let diff = line.diff(&other);
        let expected = vec![Line(0, 2), Line(8, 10)];

        assert_eq!(diff, expected);
    }
}
