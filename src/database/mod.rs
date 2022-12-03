use std::time::Duration;
use chrono::NaiveDate;
use rusqlite::{Connection, Error, OptionalExtension, Row};
use crate::utils::PuzzlePart;
use crate::utils::solution::Solution;

pub fn get_mission(conn: &Connection, year: i32) -> String {

    let params = [year];

    // language=sqlite
    let sql = "select mission from event where year = ?1";

    conn.query_row(sql, params, |r| r.get(0)).unwrap() // todo: could be an anyhow?
}

pub fn save(conn: &Connection, solution: &Solution) -> rusqlite::Result<usize> {

    // language=sqlite
    let sql = "insert into solution (date, part, result, is_correct, duration, processed_at)
                    values (?1, ?2, ?3, ?4, ?5, ?6)";

    conn.execute(sql, (
        &solution.date,
        &solution.part,
        &solution.result,
        &solution.is_correct,
        &(solution.duration.as_micros() as u64),
        &chrono::Utc::now()
    ))
}

pub fn get_correct_solution(conn: &Connection, date: NaiveDate, part: &PuzzlePart) -> Option<Solution> {

    // language=sqlite
    let sql = "
                select *
                from solution
                where date = ?1
                  and part = ?2
                  and is_correct
                order by duration asc
                  ";

    conn.query_row(sql, (&date, &part), parse_solution)
        .optional()
        .unwrap()
}

fn parse_solution(row: &Row) -> Result<Solution, Error> {
    Ok(Solution {
        date: row.get(0)?,
        part: PuzzlePart::FirstTest,
        result: row.get(2)?,
        is_correct: row.get::<usize, usize>(3)? == 1,
        duration: Duration::from_micros(row.get(4)?),
    })
}

