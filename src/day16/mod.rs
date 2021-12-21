use crate::utils::{get_input, get_input_array, get_input_string};

use std::str::Chars;

fn parse_input(filename: &str) -> Vec<char> {
    let input = get_input_string(filename);
    let bits = convert_to_binary(input.chars());
    bits
}

pub fn solve_1(filename: &str) -> String {
    let bits = parse_input(filename);
    let (acc_version, _, _) = decode(&bits);

    acc_version.to_string()
}

fn decode(bits: &[char]) -> (usize, usize, Vec<char>) {
    let (version, bits) = split_and_decode(bits, 3);
    let (packet_type, bits) = split_and_decode(bits, 3);

    println!(
        "Version={}, Type={}, content (bits: {}):",
        version,
        packet_type,
        bits.len()
    );
    println!("{:?}\n", chars_to_string(bits));

    let (sum, value, rest) = match packet_type {
        4 => decode_literal(bits),
        _ => decode_operator(bits, packet_type),
    };

    (version + sum, value, rest)
}

fn decode_operator(bits: &[char], packet_type: usize) -> (usize, usize, Vec<char>) {
    let (mode, bits) = split_and_decode(bits, 1);
    println!("Decoding with mode={} (bits={})", mode, bits.len());

    let (acc, values, rest) = match mode {
        0 => decode_by_total_size(bits),
        1 => decode_by_number_of_packets(bits),
        _ => panic!("Should never happen"),
    };

    let result = match packet_type {
        0 => values.into_iter().sum(),
        1 => values.into_iter().product(),
        2 => values.into_iter().min().unwrap(),
        3 => values.into_iter().max().unwrap(),
        5 => {
            if values[0] > values[1] {
                1
            } else {
                0
            }
        }
        6 => {
            if values[0] < values[1] {
                1
            } else {
                0
            }
        }
        7 => {
            if values[0] == values[1] {
                1
            } else {
                0
            }
        }
        _ => panic!("Should not happen!"),
    };

    (acc, result, rest)
}

// todo: clean up in owned vs not owned
fn decode_by_total_size(bits: &[char]) -> (usize, Vec<usize>, Vec<char>) {
    let (length, bits) = split_and_decode(bits, 15);
    println!("Size={} (bits={}):", length, bits.len());
    println!("{}\n", chars_to_string(bits));

    let (sub_packets, bits) = bits.split_at(length);

    let mut packets = sub_packets.to_vec();
    let mut acc_version = 0;
    let mut values = vec![];
    while !packets.is_empty() {
        println!("Packets: {}", chars_to_string(&packets));
        let (version, value, rest) = decode(&packets);
        packets = rest;
        values.push(value);
        acc_version += version;
    }

    (acc_version, values, Vec::from(bits))
}

fn decode_by_number_of_packets(bits: &[char]) -> (usize, Vec<usize>, Vec<char>) {
    let (number, bits) = split_and_decode(bits, 11);
    println!("{} packet(s) (bits={})", number, bits.len());
    println!("{:?}\n", chars_to_string(bits));

    let mut clone = Vec::from(bits);

    let mut acc_version = 0;
    let mut values = vec![];

    for _ in 0..number {
        let (version, value, rest) = decode(&clone);
        acc_version += version;
        values.push(value);
        clone = rest;
    }

    (acc_version, values, clone)
}

fn decode_literal(bits: &[char]) -> (usize, usize, Vec<char>) {
    let mut last_group = false;
    let mut values: Vec<char> = vec![];

    // todo: cleanup ugly code
    let mut rest = bits;

    while !last_group {
        let (group, a) = rest.split_at(5);
        rest = a;
        let (mode, value) = split_and_decode(group, 1);

        values.append(&mut value.to_vec());

        if mode == 0 {
            last_group = true;
        }
    }

    let value = chars_to_number(&values);
    println!(
        "Decoded literal {:?} to {:?}\n",
        chars_to_string(bits),
        value
    );

    (0, value, Vec::from(rest))
}

fn split_and_decode(bits: &[char], split: usize) -> (usize, &[char]) {
    let (target, rest) = bits.split_at(split);
    let target = chars_to_number(target);

    (target, rest)
}

pub fn solve_2(filename: &str) -> String {
    let bits = parse_input(filename);
    let (_, value, _) = decode(&bits);

    value.to_string()
}

fn chars_to_number(chars: &[char]) -> usize {
    usize::from_str_radix(chars_to_string(chars).as_str(), 2).unwrap()
}

fn chars_to_string(chars: &[char]) -> String {
    chars.iter().collect()
}

fn convert_to_binary(input: Chars) -> Vec<char> {
    input.into_iter().flat_map(hex_to_binary).collect()
}

fn hex_to_binary(hex: char) -> Vec<char> {
    let binary = match hex {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("Failed conversion"),
    };

    binary.chars().collect()
}
