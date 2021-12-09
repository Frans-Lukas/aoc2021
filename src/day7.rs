
#[aoc(day7, part1)]
pub fn part1_chars(input: &str) -> i32 {
    let mut crab_pos: Vec<i32> = Vec::new();
    let mut max_crab = 0;
    for line in input.split(',') {
        let num: i32 = line.parse().unwrap();
        if num > max_crab {
            max_crab = num;
        }
        crab_pos.push(num);
    }
    let mut min_sum = 1000000;
    for i in 0..max_crab {
        let mut sum = 0;
        for n in crab_pos.iter() {
            sum += (i - n).abs();
        }
        if sum < min_sum {
            min_sum = sum;
        }
    }
    min_sum
}

#[aoc(day7, part2)]
pub fn part2_chars(input: &str) -> i64 {
    let mut crab_pos: Vec<i64> = Vec::new();
    let mut max_crab: i64 = 0;
    for line in input.split(',') {
        let num: i64 = line.parse().unwrap();
        if num > max_crab {
            max_crab = num;
        }
        crab_pos.push(num);
    }
    let mut min_sum: i64 = std::i64::MAX;
    for i in 0..max_crab {
        let mut sum: i64 = 0;
        for n in crab_pos.iter() {
            let distance = (i - n).abs();
            let fuel_cost = ((distance as f32 / 2.0) * (distance as f32 + 1.0)) as i64;
            sum += fuel_cost;
        }
        if sum < min_sum {
            min_sum = sum;
        }
    }
    min_sum
}
