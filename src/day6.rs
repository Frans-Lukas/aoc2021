use std::cmp::max;
use std::collections::HashMap;

#[aoc(day6, part1)]
pub fn part1_chars(input: &str) -> i64 {
    day6(input, 80)
}
struct FishDay {
    curr_amount: i64,
    next_amount: i64,
}

#[aoc(day6, part2)]
pub fn part2_chars(input: &str) -> i64 {
    day6(input, 256)
}

fn day6(input: &str, num_days: i32) -> i64 {
    let mut fish_tank: Vec<FishDay> = Vec::new();
    for i in 0..9 {
        fish_tank.insert(
            i,
            FishDay {
                curr_amount: 0,
                next_amount: 0,
            },
        );
    }

    for line in input.split(',') {
        let num: i32 = line.parse().unwrap();
        fish_tank.get_mut(num as usize).unwrap().curr_amount += 1;
    }

    for _ in 0..num_days {
        for j in 0..9 {
            let mut curr_amount = 0;
            {
                curr_amount = fish_tank.get_mut(j as usize).unwrap().curr_amount;
            }
            let mut next_day: usize = j as usize;
            if next_day > 0 {
                next_day -= 1;
            } else if next_day == 0 {
                next_day = 6;
                let new_fish_day = 8 as usize;
                fish_tank.get_mut(new_fish_day).unwrap().next_amount += curr_amount;
            }
            fish_tank.get_mut(next_day).unwrap().next_amount += curr_amount;
            fish_tank.get_mut(j as usize).unwrap().curr_amount = 0;
        }
        for day in fish_tank.iter_mut() {
            day.curr_amount += day.next_amount;
            day.next_amount = 0;
        }
    }

    let mut sum = 0;
    for day in fish_tank.iter() {
        sum += day.curr_amount;
    }
    sum
}
