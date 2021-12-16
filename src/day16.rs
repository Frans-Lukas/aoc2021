use std::cmp::min;
use std::collections::HashMap;

#[aoc(day16, part1)]
pub fn part1_chars(input: &str) -> i64 {
    let mut bit_line: String = "".to_string();
    for line in input.lines() {
        for hex_char in line.chars() {
            bit_line.push_str(hex_char_to_bits(hex_char).as_str());
        }
    }
    bit_line.trim_end_matches('0');
    let mut current_index = 0;
    let mut packet_id_sum = 0;
    parse_packets(
        &bit_line,
        &mut current_index,
        usize::MAX,
        usize::MAX,
        &mut packet_id_sum,
    );

    packet_id_sum as i64
}

fn parse_packets(
    bit_line: &String,
    current_index: &mut usize,
    remaining_bits: usize,
    remaining_packets: usize,
    packet_id_sums: &mut usize,
) -> i64 {
    let mut sum = -1;
    if *current_index + 3 > remaining_bits {
        *current_index += 3;
        return sum;
    }
    let mut packet_version = str_to_decimal(&bit_line[*current_index..*current_index + 3]);
    //println!("packet_version: {}", packet_version);
    *packet_id_sums += packet_version;
    *current_index += 3;

    if *current_index + 3 > remaining_bits {
        *current_index += 3;
        return sum;
    }
    let mut packet_type_id = str_to_decimal(&bit_line[*current_index..*current_index + 3]);

    //println!("packet_type_id: {}", packet_type_id);
    *current_index += 3;

    if packet_type_id != 4 {
        if *current_index + 1 > remaining_bits {
            *current_index += 1;
            return sum;
        }
        let length_type_id = str_to_decimal(&bit_line[*current_index..*current_index + 1]);
        //println!("length_type_id: {}", length_type_id);

        //println!("IS OPERATOR PACKAGE");
        *current_index += 1;
        if length_type_id == 0 {
            if *current_index + 15 > remaining_bits {
                *current_index += 15;
                return sum;
            }
            //println!("IS LENGTH LIMITED");
            let mut packet_length = str_to_decimal(&bit_line[*current_index..*current_index + 15]);
            //println!(
            //     "length bitstr: {}",
            //     &bit_line[*current_index..*current_index + 15]
            // );
            *current_index += 15;
            packet_length += *current_index;
            //println!("packet_length: {}", packet_length);
            let mut results = Vec::<i64>::new();
            while packet_length > *current_index {
                //println!();
                //println!("{} > {}", packet_length, *current_index);
                results.push(parse_packets(
                    &bit_line,
                    current_index,
                    packet_length,
                    remaining_packets,
                    packet_id_sums,
                ));
            }
            sum = calc_sum(results, packet_type_id);
            //println!("{} > {}", packet_length, *current_index);
        } else {
            if *current_index + 11 > remaining_bits {
                *current_index += 11;
                return sum;
            }
            //println!("IS PACKET LIMITED");
            let num_packets = str_to_decimal(&bit_line[*current_index..*current_index + 11]);
            *current_index += 11;
            let mut curr_pack_num = 0;
            //println!("num_packets: {}", num_packets);
            let mut results = Vec::<i64>::new();
            while num_packets > curr_pack_num {
                curr_pack_num += 1;
                //println!();
                results.push(parse_packets(
                    bit_line,
                    current_index,
                    remaining_bits,
                    num_packets,
                    packet_id_sums,
                ));
            }
            sum = calc_sum(results, packet_type_id);
        }
    } else {
        //println!("IS LITERAL VALUE");
        let mut sum_str: String = "".to_string();
        loop {

            if *current_index + 1 > remaining_bits {
                *current_index += 1;
                //println!("end HERE");
                return sum;
            }
            let first_bit = &bit_line[*current_index..*current_index + 1];
            *current_index += 1;

            if *current_index + 4 > remaining_bits {
                *current_index += 4;
                //println!("end HERE 222");
                return sum;
            }
            sum_str.push_str(&bit_line[*current_index..*current_index + 4]);
            // let value = str_to_decimal(&bit_line[*current_index..*current_index+4]);
            // sum += value;
            *current_index += 4;
            sum = str_to_decimal(sum_str.as_str()) as i64;

            if first_bit == "0" {

                //println!(
                //     "READ value: {} = {}",
                //     sum_str,
                //     sum
                // );
                break;
            }
        }
    }
    sum
}

fn calc_sum(results: Vec<i64>, sum_type: usize) -> i64 {
    match sum_type {
        0 => {
            return results.iter().sum();
        }
        1 => {
            let mut sum = 1;
            results.iter().for_each(|v| sum *= v);
            return sum;
        }
        2 => {
            return *results.iter().min().unwrap();
        }
        3 => {
            return *results.iter().max().unwrap();
        }
        5 => {
            if results[0] > results[1] {
                return 1;
            }
            return 0;
        }
        6 => {
            if results[0] < results[1] {
                return 1;
            }
            return 0;
        }
        7 => {
            if results[0] == results[1] {
                return 1;
            }
            return 0;
        }
        _ => {}
    }
    0
}

fn str_to_decimal(str: &str) -> usize {
    let mut sum: usize = 0;
    let mut multiplier: usize = 1;
    let mut str_array: Vec<char> = str.chars().collect();
    str_array.reverse();
    for c in str_array.iter() {
        sum += c.to_digit(10).unwrap() as usize * multiplier;
        multiplier *= 2;
    }
    sum
}

fn packet_version(str: &str) {}

fn hex_char_to_bits(c: char) -> String {
    match c {
        '0' => "0000".to_string(),
        '1' => "0001".to_string(),
        '2' => "0010".to_string(),
        '3' => "0011".to_string(),
        '4' => "0100".to_string(),
        '5' => "0101".to_string(),
        '6' => "0110".to_string(),
        '7' => "0111".to_string(),
        '8' => "1000".to_string(),
        '9' => "1001".to_string(),
        'A' => "1010".to_string(),
        'B' => "1011".to_string(),
        'C' => "1100".to_string(),
        'D' => "1101".to_string(),
        'E' => "1110".to_string(),
        'F' => "1111".to_string(),
        _ => "abcd".to_string(),
    }
}

#[aoc(day16, part2)]
pub fn part2_chars(input: &str) -> i64 {
    let mut bit_line: String = "".to_string();
    for line in input.lines() {
        for hex_char in line.chars() {
            bit_line.push_str(hex_char_to_bits(hex_char).as_str());
        }
    }
    bit_line.trim_end_matches('0');
    let mut current_index = 0;
    let mut packet_id_sum = 0;
    let sum = parse_packets(
        &bit_line,
        &mut current_index,
        usize::MAX,
        usize::MAX,
        &mut packet_id_sum,
    );

    sum
}
