use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::i32::MAX;
use std::io::empty;

use regex::{Match, Regex};

struct Rule {
    gram: Gram,
    result: char,
}

#[aoc(day14, part1)]
pub fn part1_chars(input: &str) -> i32 {
    // let mut fl: String = input.lines().next().unwrap().to_string();
    // println!("{}", fl);
    // let mut rules = Vec::<Rule>::new();
    // for line in input.lines().skip(2) {
    //     let str_rule: Vec<String> = line.split(" -> ").map(|s| s.to_string()).collect();
    //     rules.push(Rule {
    //         gram: Gram {
    //             c1: str_rule[0].chars().nth(0).unwrap(),
    //             c2: str_rule[0].chars().nth(1).unwrap(),
    //         },
    //         result: str_rule[1].chars().nth(0).unwrap(),
    //     });
    //     // println!("{} -> {}", rules.last().unwrap().start, rules.last().unwrap().result);
    // }
    // let mut unique_chars = HashMap::<char, i32>::new();
    //
    // for i in 0..10 {
    //     println!("{}", i);
    //     let mut result = fl.clone();
    //     let mut hits = Vec::<(MyMatch, char)>::new();
    //     for rule in rules.iter() {
    //         // println!("{} -> {}", rule.start, rule.result);
    //         let matches = m_regx(fl.as_str(), rule.start.as_str());
    //         for hit in matches {
    //             hits.push((hit, rule.result));
    //         }
    //     }
    //     hits.sort_by(|(a, _), (b, _)| a.start.partial_cmp(&b.start).unwrap());
    //     for (i, (hit, c)) in hits.iter().enumerate() {
    //         // println!(
    //         //     "hit: {}, start: {}, char: {}",
    //         //     hit.text,
    //         //     hit.start + 1 + i,
    //         //     c
    //         // );
    //         // println!("{}", result);
    //         result.insert(hit.start + i + 1, *c);
    //     }
    //     // println!("{}", result);
    //     fl = result;
    // }
    // for c in fl.chars() {
    //     if unique_chars.contains_key(&c) {
    //         *unique_chars.get_mut(&c).unwrap() += 1;
    //     } else {
    //         unique_chars.insert(c, 1);
    //     }
    // }
    // let mut max = 0;
    // let mut min = MAX;
    // for (c, v) in unique_chars.iter() {
    //     if *v > max {
    //         max = *v;
    //     }
    //     if *v < min {
    //         min = *v;
    //     }
    // }

    0
}

pub struct MyMatch {
    text: String,
    start: usize,
}

pub fn m_regx(search_string: &str, regex: &str) -> Vec<MyMatch> {
    let c1: char = regex.chars().nth(0).unwrap();
    let c2: char = regex.chars().nth(1).unwrap();
    let mut found_matches = Vec::<MyMatch>::new();
    for (i, c) in search_string.chars().enumerate() {
        if i < search_string.len() - 1 && c == c1 && search_string.chars().nth(i + 1).unwrap() == c2
        {
            // println!("rx: {} -> {}{}", regex, c1, c2);
            found_matches.push(MyMatch {
                text: search_string[i..i + 2].to_string(),
                start: i,
            });
        }
    }
    found_matches
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


#[aoc(day14, part2)]
pub fn part2_chars(input: &str) -> i32 {
    let mut fl: String = input.lines().next().unwrap().to_string();
    println!("{}", fl);
    let mut rules = Vec::<Rule>::new();
    let mut num_of_each = HashMap::<Gram, i32>::new();
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
        // println!("{} -> {}", rules.last().unwrap().start, rules.last().unwrap().result);
    }

    for i in 0..3 {
        let mut new_grams = HashMap::<Gram, i32>::new();
        for rule in rules.iter() {
            if num_of_each.contains_key(&rule.gram) {
                let to_insert1 = Gram {
                    c1: rule.gram.c1,
                    c2: rule.result,
                };
                if new_grams.contains_key(&to_insert1) {
                    *new_grams.get_mut(&to_insert1).unwrap() += 1;
                } else {
                    new_grams.insert(
                        to_insert1,
                        *num_of_each.get(&rule.gram).unwrap(),
                    );
                }
                let to_insert2 = Gram {
                    c1: rule.result,
                    c2: rule.gram.c2,
                };

                if new_grams.contains_key(&to_insert2) {
                    *new_grams.get_mut(&to_insert2).unwrap() += 1;
                } else {
                    new_grams.insert(
                        to_insert2,
                        *num_of_each.get(&rule.gram).unwrap(),
                    );
                }
            }
        }
        num_of_each = new_grams;

        println!();
        for (gram, v) in num_of_each.iter() {
            print!("{}{}: {},", gram.c1, gram.c2, v);
        }
        println!();
    }

    println!();
    for (gram, v) in num_of_each.iter() {
        print!("{}{}: {},", gram.c1, gram.c2, v);
    }
    println!();

    let mut unique_chars = HashMap::<char, i32>::new();
    for (gram, i) in num_of_each {
        if unique_chars.contains_key(&gram.c1) {
            *unique_chars.get_mut(&gram.c1).unwrap() += i;
        } else {
            unique_chars.insert(gram.c1, i);
        }
        if unique_chars.contains_key(&gram.c2) {
            *unique_chars.get_mut(&gram.c2).unwrap() += i;
        } else {
            unique_chars.insert(gram.c2, i);
        }
    }
    let mut max = 0;
    let mut min = MAX;
    for (c, v) in unique_chars.iter() {

        if *v > max {
            max = *v;
        }
        if *v < min {
            min = *v;
        }
    }
    println!("max: {}", max);
    max - min
}
