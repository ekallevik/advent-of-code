use std::fmt::{Display, Formatter};
use itertools::Itertools;

pub struct Grid<T: Display> {
    pub row_size: usize,
    pub col_size: usize,
    pub elements: Vec<T>,
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        for r in 0..self.row_size {
            for c in 0..self.col_size {
                write!(f, " {}", self.get(r, c)).expect("TODO: panic message");
            }
            writeln!(f).expect("TODO: panic message");
        }

        Ok(())
    }
}

pub fn from_vec<T: Display>(values: Vec<Vec<T>>) -> Grid<T> {

    let col_size = values.len();
    let row_size = values[0].len();

    let elements = values.into_iter().flatten().collect_vec();

    Grid {
        row_size,
        col_size,
        elements
    }
}


impl<T: Display> Grid<T> {

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.elements[row * self.row_size + col]
    }

    pub fn get_col(&self, col: usize) -> Vec<&T> {
        (0..self.row_size)
            .into_iter()
            .map(|i| self.get(i, col))
            .collect()
    }

    pub fn get_row(&self, row: usize) -> Vec<&T> {
        let offset = row * self.row_size;
        self.elements[offset..offset + self.row_size].iter().collect_vec()
    }

    pub fn iterate(&self) -> Vec<(usize, usize)> {
        (0..self.row_size)
            .into_iter()
            .flat_map(|r| (0..self.col_size).into_iter().map(|c| (r, c)).collect_vec())
            .collect_vec()
    }

}

