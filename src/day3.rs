use std::cmp::max;
use std::collections::HashMap;

#[aoc(day3, part1)]
pub fn part1_chars(input: &str) -> i32 {
    let mut gamma_rate: Vec<i32> = Vec::new();
    let mut bits: Vec<&str> = Vec::new();
    let mut epsilon_rate: Vec<i32> = Vec::new();
    for line in input.lines() {
        bits.push(line);
    }
    for i in 0..bits[0].len() {
        let mut num_ones = 0;
        for bit_p in bits.iter() {
            if bit_p.chars().nth(i).unwrap() == '1' {
                num_ones += 1;
            }
        }
        if num_ones >= (bits.len() - num_ones) {
            gamma_rate.push(1);
            epsilon_rate.push(0);
        } else {
            gamma_rate.push(0);
            epsilon_rate.push(1);
        }
    }

    let e_rate: String = epsilon_rate.into_iter().map(|d| char::from_digit(d as u32, 10).unwrap()).collect();
    let g_rate: String = gamma_rate.into_iter().map(|d| char::from_digit(d as u32, 10).unwrap()).collect();

    let intval_e = isize::from_str_radix(&e_rate, 2).unwrap();
    let intval_g = isize::from_str_radix(&g_rate, 2).unwrap();

    (intval_e * intval_g) as i32
}

#[aoc(day3, part2)]
pub fn part2_chars(input: &str) -> i32 {
    let mut bits: Vec<&str> = Vec::new();
    for line in input.lines() {
        bits.push(line);
    }
    let mut og_r = 0;
    let mut co2_r = 0;
    let mut og_vec = bits.to_vec();
    let mut co2_vec = bits.to_vec();

    for i in 0..bits[0].len() {
        let mut most_common_og = find_most_common_bit(&mut og_vec, i);
        let mut most_common_co2 = find_most_common_bit(&mut co2_vec, i);
        og_vec = og_vec.into_iter().filter(|s| s.chars().nth(i).unwrap().to_digit(10).unwrap() == most_common_og).collect::<Vec<&str>>();
        co2_vec = co2_vec.into_iter().filter(|s| s.chars().nth(i).unwrap().to_digit(10).unwrap() != most_common_co2).collect::<Vec<&str>>();

        if og_vec.len() == 1 {
            og_r = isize::from_str_radix(og_vec[0], 2).unwrap();
        }
        if co2_vec.len() == 1 {
            co2_r = isize::from_str_radix(co2_vec[0], 2).unwrap();
        }
    }
    (og_r * co2_r) as i32
}

fn find_most_common_bit(og_vec: &mut Vec<&str>, i: usize) -> u32{
    let mut num_ones = 0;
    for bit_p in og_vec.iter() {
        if bit_p.chars().nth(i).unwrap() == '1' {
            num_ones += 1;
        }
    }
    let mut to_return = 0;
    if num_ones >= (og_vec.len() - num_ones) {
        to_return = 1;
    }
    to_return
}