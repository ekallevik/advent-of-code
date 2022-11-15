use std::collections::VecDeque;
use instruction::Instruction;
use crate::utils::get_input;

mod source;
mod instruction;

fn parse_instructions(filename: &str) -> Vec<Instruction> {
    let input: Vec<String> = get_input(filename);

    input.iter().map(|line| line.parse().unwrap()).collect()
}

pub fn solve_1(filename: &str) -> String {

    let instructions = parse_instructions(filename);

    let mut input: VecDeque<u32> = VecDeque::new();


    let smallest = "13579246899998";
    //let biggest = "9999999999999";

    let num = smallest
        .chars()
        .filter_map(|x| x.to_digit(10))
        .collect::<Vec<u32>>();

    for n in num {
        input.push_back(n);
    }

    println!("Deq: {:?}", input);

    //input.push_back(15);

    run_instructions(&instructions, input);


    "12".to_string()
}

fn run_instructions(instructions: &[Instruction], mut input: VecDeque<u32>) {

    let mut registry = [0, 0, 0, 0];
    for op in instructions {
        println!("Apply op: {:?}", op);

        op.apply(&mut registry, &mut input)
    }

    println!("Reg: {:?}", registry);


}

pub fn solve_2(filename: &str) -> String {
    filename.to_string()
}
