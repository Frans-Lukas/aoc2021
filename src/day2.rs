
#[aoc(day2, part1)]
pub fn part1_chars(input: &str) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    for line in input.lines() {
        let input: Vec<&str> = line.split(" ").collect();
        let dir = input[0];
        let amp: i32 = input[1].parse().unwrap();

        match dir {
            "forward" => { horizontal += amp; }
            "down" => { depth += amp; }
            "up" => { depth -= amp; }
            _ => {}
        }
    }
    depth * horizontal
}

#[aoc(day2, part2)]
pub fn part2_chars(input: &str) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in input.lines() {
        let input: Vec<&str> = line.split(" ").collect();
        let dir = input[0];
        let amp: i32 = input[1].parse().unwrap();

        match dir {
            "forward" => {
                horizontal += amp;
                depth += aim * amp;
            }
            "down" => { aim += amp; }
            "up" => { aim -= amp; }
            _ => {}
        }
    }
    depth * horizontal
}