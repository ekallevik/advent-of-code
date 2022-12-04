use anyhow::Result;
use itertools::PeekingNext;
use paris::info;
use crate::utils::get_input_string;

pub fn solve_1(filename: &str) -> Result<String> {
    let input = get_input_string(filename);
    let output = iterate_look_and_say(input, 40);

    Ok(output.len().to_string())
}

pub fn solve_2(filename: &str) -> Result<String> {
    let input = get_input_string(filename);
    let output = iterate_look_and_say(input, 50);

    Ok(output.len().to_string())
}

fn iterate_look_and_say(mut input: String, iterations: usize) -> String {
    for i in 0..iterations {
        info!("iteration: {i}");
        input = look_and_say(&input);
    }

    input
}

fn look_and_say(input: &str) -> String {

    let mut chars = input.chars();
    let mut output = "".to_string();

    while let Some(char) = chars.next() {

        let mut counter = 1;

        while chars.peeking_next(|p| *p == char).is_some() {
            counter += 1;
        }

        output.push_str(counter.to_string().as_str());
        output.push(char);
    }

    output
}