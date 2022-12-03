use std::time::Duration;
use crate::PuzzlePart;
use chrono::{NaiveDate};



#[derive(Debug)]
pub struct Solution {
    pub date: NaiveDate,
    pub part: PuzzlePart,
    pub result: String,
    pub is_correct: bool,
    pub duration: Duration,
}
