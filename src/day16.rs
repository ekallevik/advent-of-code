use crate::utils::get_input_array;

use std::str::Chars;

pub fn solve_1(filename: &str) -> String {
    //let input: Vec<char> = get_input(filename);

    let inputs = vec![
        //"D2FE28",
        //"38006F45291200",
        //"EE00D40C823060",
        //"8A004A801A8002F478",
        //"620080001611562C8802118E34",
        //"C0015000016115A2E0802F182340",
        //"A0016C880162017C3686B18A3D4780",
        "00537390040124EB240B3EDD36B68014D4C9ECCCE7BDA54E62522A300525813003560004223BC3F834200CC108710E98031C94C8B4BFFF42398309DDD30EEE00BCE63F03499D665AE57B698F9802F800824DB0CE1CC23100323610069D8010ECD4A5CE5B326098419C319AA2FCC44C0004B79DADB1EB48CE5EB7B2F4A42D9DF0AA74E66468C0139341F005A7BBEA5CA65F3976200D4BC01091A7E155991A7E155B9B4830056C01593829CC1FCD16C5C2011A340129496A7EFB3CA4B53F7D92675A947AB8A016CD631BE15CD5A17CB3CEF236DBAC93C4F4A735385E401804AA86802D291ED19A523DA310006832F07C97F57BC4C9BBD0764EE88800A54D5FB3E60267B8ED1C26AB4AAC0009D8400854138450C4C018855056109803D11E224112004DE4DB616C493005E461BBDC8A80350000432204248EA200F4148FD06C804EE1006618419896200FC1884F0A00010A8B315A129009256009CFE61DBE48A7F30EDF24F31FCE677A9FB018F6005E500163E600508012404A72801A4040688010A00418012002D51009FAA0051801CC01959801AC00F520027A20074EC1CE6400802A9A004A67C3E5EA0D3D5FAD3801118E75C0C00A97663004F0017B9BD8CCA4E2A7030C0179C6799555005E5CEA55BC8025F8352A4B2EC92ADF244128C44014649F52BC01793499EA4CBD402697BEBD18D713D35C9344E92CB67D7DFF05A60086001610E21A4DD67EED60A8402415802400087C108DB068001088670CA0DCC2E10056B282D6009CFC719DB0CD3980026F3EEF07A29900957801AB8803310A0943200042E3646789F37E33700BE7C527EECD13266505C95A50F0C017B004272DCE573FBB9CE5B9CAE7F77097EC830401382B105C0189C1D92E9CCE7F758B91802560084D06CC7DD679BC8048AF00400010884F18209080310FE0D47C94AA00"
        //"C200B40A82"
        //"04005AC33890"
        //"880086C3E88112"
    ];

    for input in inputs {
        let bits = convert_to_binary(input.chars());
        println!(
            "Decoding: (bits: {}), \n{}\n",
            bits.len(),
            chars_to_string(&bits)
        );

        let (acc_version, value, remainder) = decode(&bits);
        println!("Acc Version: \n{:?}", acc_version);
        println!("Value: \n{:?}", value);
        println!("Remainder: \n{:?}", chars_to_string(&remainder));
    }

    filename.to_string()
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
    let _input: Vec<Vec<usize>> = get_input_array(filename);

    unimplemented!("Not implemented yet");
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

// todo: return type here
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
