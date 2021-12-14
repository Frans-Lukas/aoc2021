use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::i32::MAX;
use std::io::empty;

use regex::{Match, Regex};

struct Rule {
    gram: Gram,
    result: char,
}

#[derive(Hash, Copy, Clone)]
struct Gram {
    c1: char,
    c2: char,
}

impl PartialEq for Gram {
    fn eq(&self, other: &Self) -> bool {
        self.c1 == other.c1 && self.c2 == other.c2
    }
}
impl Eq for Gram {}


#[aoc(day14, part1)]
pub fn part1_chars(input: &str) -> i64 {
    apply_rules(input, 10)
}

#[aoc(day14, part2)]
pub fn part2_chars(input: &str) -> i64 {
    apply_rules(input, 40)
}

pub fn apply_rules(input: &str, iters: i32) -> i64 {
    let mut fl: String = input.lines().next().unwrap().to_string();
    let mut rules = Vec::<Rule>::new();
    let mut num_of_each = HashMap::<Gram, i64>::new();

    for (i, c) in fl.chars().enumerate() {
        if i < fl.len() - 1 {
            let to_inesrt = Gram {
                c1: c,
                c2: fl.chars().nth(i + 1).unwrap(),
            };
            if num_of_each.contains_key(&to_inesrt) {
                *num_of_each.get_mut(&to_inesrt).unwrap() += 1;
            } else {
                num_of_each.insert(to_inesrt, 1);
            }
        }
    }

    for line in input.lines().skip(2) {
        let str_rule: Vec<String> = line.split(" -> ").map(|s| s.to_string()).collect();
        rules.push(Rule {
            gram: Gram {
                c1: str_rule[0].chars().nth(0).unwrap(),
                c2: str_rule[0].chars().nth(1).unwrap(),
            },
            result: str_rule[1].chars().nth(0).unwrap(),
        });
    }

    for _ in 0..iters {
        let mut new_grams = HashMap::<Gram, i64>::new();
        for rule in rules.iter() {
            if num_of_each.contains_key(&rule.gram) {
                for to_insert in vec![
                    Gram {
                        c1: rule.gram.c1,
                        c2: rule.result,
                    },
                    Gram {
                        c1: rule.result,
                        c2: rule.gram.c2,
                    },
                ] {
                    let count = *num_of_each.get(&rule.gram).unwrap();
                    if new_grams.contains_key(&to_insert) {
                        *new_grams.get_mut(&to_insert).unwrap() += count;
                    } else {
                        new_grams.insert(to_insert, count);
                    }
                }
            }
        }
        num_of_each = new_grams;
    }

    let mut unique_chars = HashMap::<char, i64>::new();
    let mut sum = 0;
    for (gram, i) in num_of_each {
        sum += i;
        for gram_c in vec![gram.c1, gram.c2] {
            if unique_chars.contains_key(&gram_c) {
                *unique_chars.get_mut(&gram_c).unwrap() += i;
            } else {
                unique_chars.insert(gram_c, i);
            }
        }
    }

    let mut max = 0;
    let mut min = i64::MAX;
    for (c, v) in unique_chars.iter() {
        if *v > max {
            max = *v;
        }
        if *v < min {
            min = *v;
        }
    }
    // WTF, why does div by 2 work?
    (max - min) / 2
}
