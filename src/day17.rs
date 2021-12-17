use std::cmp::max;
use std::collections::HashSet;

struct TargetZone {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

#[aoc(day17, part1)]
pub fn part1_chars(input: &str) -> i32 {
    let input_arr: Vec<&str> = input.split(' ').collect();
    let xlist: Vec<&str> = input_arr[2]
        .trim_start_matches("x=")
        .trim_end_matches(",")
        .split("..")
        .collect();
    let ylist: Vec<&str> = input_arr[3].trim_start_matches("y=").split("..").collect();
    let target_zone = TargetZone {
        x1: xlist[0].parse().unwrap(),
        x2: xlist[1].parse().unwrap(),
        y1: ylist[0].parse().unwrap(),
        y2: ylist[1].parse().unwrap(),
    };
    let mut max_y_pos = i32::MIN;
    for x in 0..1000 {
        for y in -1000..1000 {
            // let mut tx = 7;
            // let mut ty = 2;
            let mut x_speed = x.clone();
            let mut y_speed = y.clone();
            let (result, entered_zone) = simulate_speed(&mut x_speed, &mut y_speed, &target_zone);
            // println!("{} {}", entered_zone, result);
            if result > max_y_pos && entered_zone {
                max_y_pos = result;
            }
        }
    }
    max_y_pos
}
#[derive(Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {}

fn simulate_speed(x_speed: &mut i32, y_speed: &mut i32, tz: &TargetZone) -> (i32, bool) {
    let mut current_pos = Point { x: 0, y: 0 };

    let mut has_entered_zone = false;
    let mut has_left_zone = false;

    let mut max_y = current_pos.y;
    while current_pos.x <= tz.x2 && current_pos.y >= tz.y1{
        current_pos.x += *x_speed;
        current_pos.y += *y_speed;

        if current_pos.y > max_y {
            max_y = current_pos.y.clone();
        }
        check_bounds(
            tz,
            &mut current_pos,
            &mut has_entered_zone,
            &mut has_left_zone,
        );
        if *x_speed > 0 {
            *x_speed -= 1;
        } else if *x_speed < 0 {
            *x_speed += 1;
        }
        *y_speed -= 1;
    }
    (max_y, has_entered_zone)
}


fn distance(p1: &Point, p2: &TargetZone) -> i32 {

    ((p1.x - p2.x1).pow(2) as f32 + (p1.y - p2.y1).pow(2) as f32).sqrt() as i32
}

fn check_bounds(
    tz: &TargetZone,
    current_pos: &mut Point,
    has_entered_zone: &mut bool,
    has_left_zone: &mut bool,
) {
    if inside_tz(&current_pos, tz) {
        *has_entered_zone = true;
    } else if *has_entered_zone {
        *has_left_zone = true;
    }
}

fn inside_tz(pos: &Point, tz: &TargetZone) -> bool {
    pos.x >= tz.x1 && pos.x <= tz.x2 && pos.y >= tz.y1 && pos.y <= tz.y2
}

#[aoc(day17, part2)]
pub fn part2_chars(input: &str) -> i32 {
    let input_arr: Vec<&str> = input.split(' ').collect();
    let xlist: Vec<&str> = input_arr[2]
        .trim_start_matches("x=")
        .trim_end_matches(",")
        .split("..")
        .collect();
    let ylist: Vec<&str> = input_arr[3].trim_start_matches("y=").split("..").collect();
    let target_zone = TargetZone {
        x1: xlist[0].parse().unwrap(),
        x2: xlist[1].parse().unwrap(),
        y1: ylist[0].parse().unwrap(),
        y2: ylist[1].parse().unwrap(),
    };
    let mut count = 0;
    for x in 0..1000 {
        for y in -1000..1000 {
            let mut x_speed = x.clone();
            let mut y_speed = y.clone();

            let (result, entered_zone) = simulate_speed(&mut x_speed, &mut y_speed, &target_zone);
            if entered_zone {
                count += 1;
            }
        }
    }
    count
}
