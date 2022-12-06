use anyhow::{anyhow, Result};
use itertools::Itertools;
use crate::utils::get_input_string;

pub fn solve_1(filename: &str) -> Result<String> {
    let signal = get_input_string(filename);
    let packet_length = 4;

    scan_message(&signal, packet_length)
}

pub fn solve_2(filename: &str) -> Result<String> {
    let signal = get_input_string(filename);
    let message_length = 14;

    scan_message(&signal, message_length)
}

fn scan_message(signal: &str, message_length: usize) -> Result<String> {
    for i in message_length..signal.len() {
        let message = signal[i-message_length..i].chars();
        if &message.unique().count() == &message_length {
            return Ok(i.to_string())
        }
    }

   Err(anyhow!("Could not find any valid message"))
}