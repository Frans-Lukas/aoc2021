use std::char::from_u32;
use std::cmp::max;
use std::collections::{HashMap, HashSet};

#[aoc(day8, part1)]
pub fn part1_chars(input: &str) -> i32 {
    let mut count = 0;
    for line in input.lines() {
        let real_input: Vec<&str> = line.split('|').collect();
        let right_side = real_input[1];
        for num in right_side.split(' ') {
            let nl = num.len();
            if nl == 2 || nl == 4 || nl == 3 || nl == 7 {
                count += 1;
            }
        }
    }
    count
}

#[aoc(day8, part2)]
pub fn part2_chars(input: &str) -> i32 {
    let mut count = 0;
    let mut char_vecs: HashMap<i32, Vec<char>> = HashMap::new();
    for line in input.lines() {
        let real_input: Vec<&str> = line.split('|').collect();
        let left_side = real_input[0];
        for nl in left_side.split(' ') {
            if nl.len() == 2 {
                char_vecs.insert(1, nl.chars().collect());
            } else if nl.len() == 4 {
                char_vecs.insert(4, nl.chars().collect());
            } else if nl.len() == 3 {
                char_vecs.insert(7, nl.chars().collect());
            } else if nl.len() == 7 {
                char_vecs.insert(8, nl.chars().collect());
            }
        }

        for nl in left_side.split(' ') {
            let mut il: Vec<char> = nl.chars().collect();
            if il.len() == 6
                && contains_all(&il, &char_vecs.get(&1).unwrap())
                && contains_all(&il, &char_vecs.get(&7).unwrap())
                && !contains_all(&il, &char_vecs.get(&4).unwrap())
            {
                char_vecs.insert(0, nl.chars().collect());
            } else if il.len() == 6
                && contains_all(&il, &char_vecs.get(&1).unwrap())
                && contains_all(&il, &char_vecs.get(&7).unwrap())
                && contains_all(&il, &char_vecs.get(&4).unwrap())
            {
                char_vecs.insert(9, nl.chars().collect());
            } else if il.len() == 5 && contains_all(&il, &char_vecs.get(&7).unwrap()) {
                char_vecs.insert(3, nl.chars().collect());
            } else if il.len() == 6
                && !contains_all(&il, &char_vecs.get(&1).unwrap())
                && !contains_all(&il, &char_vecs.get(&7).unwrap())
                && !contains_all(&il, &char_vecs.get(&4).unwrap())
            {
                char_vecs.insert(6, nl.chars().collect());
            } else if il.len() == 5 && intersect_count(&il, &char_vecs.get(&4).unwrap()) == 3 {
                char_vecs.insert(5, nl.chars().collect());
            } else if il.len() == 5 && intersect_count(&il, &char_vecs.get(&4).unwrap()) == 2 {
                char_vecs.insert(2, nl.chars().collect());
            }
        }
        let right_side = real_input[1];
        let mut resulting_number = 0;
        let mut mult = 1000;
        for num in right_side.split(' ') {
            if num.len() > 1 {
                for (i, v) in char_vecs.iter() {
                    let mut il: Vec<char> = num.chars().collect();
                    if num.len() == v.len() && contains_all(&il, v) {
                        resulting_number += mult * i;
                        mult = mult / 10;
                    }
                }
            }
        }
        count += resulting_number;
        println!("num str: {}", resulting_number);
    }
    // for (i, v) in char_vecs.iter() {
    //     print!("{}: ", i);
    //     for c in v {
    //         print!("{}", *c);
    //     }
    //     println!();
    // }
    count
}

fn contains_all(first: &Vec<char>, second: &Vec<char>) -> bool {
    for c in second {
        if !first.contains(c) {
            return false;
        }
    }
    true
}

fn intersect_count(first: &Vec<char>, second: &Vec<char>) -> i32 {
    let mut sum = 0;
    for c in second {
        if first.contains(c) {
            sum += 1;
        }
    }
    sum
}

fn intersect(hm: &HashMap<char, Vec<char>>, key: char, chars: Vec<char>) {
    // let mut intersection
}

fn all_chars_found(lcc: &HashMap<char, Vec<char>>) -> bool {
    for (c, hs) in lcc.iter() {
        if hs.len() != 1 {
            return false;
        }
    }
    true
}
