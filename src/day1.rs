use std::collections::HashMap;
use std::cmp::max;

#[aoc(day1, part1)]
pub fn part1_chars(input: &str) -> i32 {
    let mut num_inc = 0;
    let mut prev = 0;
    for line in input.lines() {
        let num: i32 = line.parse().unwrap();
        if num > prev && prev != 0 {
            num_inc += 1;
        }
        prev = num;
    }
    num_inc
}


// #[aoc(day1, part2)]
// pub fn part2_chars(input: &str) -> i32 {
//     let mut add_num: Vec<Vec<i32>> = Vec::new();
//     let mut window_size = 3;
//     // index = 0 A      0
//     // index = 1 A B    0 1
//     // index = 2 A B C  0 1 2
//     // index = 3   B C    1 2
//     // index = math.max(0, i - 3).. i
//     for (i, line) in input.lines().enumerate() {
//         let mut index = i as i32;
//         let mut num: i32 = line.parse().unwrap();
//         for addor in add_num.iter_mut() {
//             if index < addor[0] {
//                 *addor.get_mut(1).unwrap() += num;
//             }
//         }
//         add_num.push(vec![(index + window_size), num]);
//     }
//
//
//     let mut num_inc = 0;
//     let mut prev = 0;
//
//     for addor in &add_num {
//         let num = addor[1];
//         if num > prev && prev != 0 {
//             num_inc += 1;
//         }
//         prev = num;
//     }
//
//     num_inc
// }

#[aoc(day1, part2)]
pub fn part2_chars(input: &str) -> i32 {
    let mut sums: Vec<i32> = Vec::new();
    let mut window_size = 3;
    // index = 0 A      0
    // index = 1 A B    0 1
    // index = 2 A B C  0 1 2
    // index = 3   B C    1 2
    // for i in math.max(0, i - 3).. i
    for (j, line) in input.lines().enumerate() {
        let mut num: i32 = line.parse().unwrap();
        let start_index: i32 = max(0, j as i32-window_size);
        let end_index: i32 = j as i32;
        for k in start_index..end_index {
            if !sums.contains(&(k as i32)) {
                sums.push(0);
            }
            *sums.get_mut(k as usize).unwrap()+= num;
        }
    }


    let mut num_inc = 0;
    let mut prev = 0;

    for addor in &sums {
        let num = *addor;
        if num > prev && prev != 0 {
            num_inc += 1;
        }
        prev = num;
    }

    num_inc
}