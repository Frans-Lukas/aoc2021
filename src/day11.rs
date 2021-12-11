struct Cell {
    value: i32,
    was_activated: bool,
}

#[aoc(day11, part1)]
pub fn part1_chars(input: &str) -> i32 {
    let mut fish_map: Vec<Vec<Cell>> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        fish_map.push(Vec::new());
        for digit in line.chars() {
            fish_map.get_mut(i).unwrap().push(Cell {
                value: digit.to_digit(10).unwrap() as i32,
                was_activated: false,
            });
        }
    }
    let mut count = 0;
    for _ in 0..100 {
        let mut exploded = true;
        for x in 0..fish_map.len() {
            for y in 0..fish_map[x].len() {
                let mut exploded_fish;
                {
                    exploded_fish = fish_map.get_mut(x).unwrap().get_mut(y).unwrap();
                }
                exploded_fish.value += 1;
            }
        }
        while exploded {
            exploded = false;
            for x in 0..fish_map.len() {
                for y in 0..fish_map[x].len() {
                    if fish_map[x][y].value > 9 {
                        let mut exploded_fish;
                        {
                            exploded_fish = fish_map.get_mut(x).unwrap().get_mut(y).unwrap();
                        }
                        if !exploded_fish.was_activated {
                            exploded_fish.was_activated = true;
                            spread_from(&mut fish_map, x as i32, y as i32);
                            count += 1;
                            exploded = true;
                        }
                    }
                }
            }
        }
        for x in 0..fish_map.len() {
            for y in 0..fish_map[x].len() {
                if fish_map[x][y].was_activated {
                    let exploded_fish = fish_map.get_mut(x).unwrap().get_mut(y).unwrap();
                    exploded_fish.value = 0;
                    exploded_fish.was_activated = false;
                }
            }
        }
    }
    count
}

struct Point {
    x: i32,
    y: i32,
}

fn spread_from(fish_map: &mut Vec<Vec<Cell>>, x: i32, y: i32) {
    let coords = vec![
        Point { x: 1, y: 1 },
        Point { x: -1, y: 1 },
        Point { x: 1, y: -1 },
        Point { x: -1, y: -1 },
        Point { x: 0, y: 1 },
        Point { x: 1, y: 0 },
        Point { x: -1, y: 0 },
        Point { x: 0, y: -1 },
    ];
    for coord in coords {
        if !coord_is_oob(fish_map, x + coord.x, y + coord.y) {
            fish_map
                .get_mut((x + coord.x) as usize)
                .unwrap()
                .get_mut((y + coord.y) as usize)
                .unwrap()
                .value += 1;
        }
    }
}

fn coord_is_oob(fish_map: &Vec<Vec<Cell>>, x: i32, y: i32) -> bool {
    x < 0 || y < 0 || x >= fish_map.len() as i32 || y >= fish_map[x as usize].len() as i32
}

#[aoc(day11, part2)]
pub fn part2_chars(input: &str) -> i32 {
    let mut fish_map: Vec<Vec<Cell>> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        fish_map.push(Vec::new());
        for digit in line.chars() {
            fish_map.get_mut(i).unwrap().push(Cell {
                value: digit.to_digit(10).unwrap() as i32,
                was_activated: false,
            });
        }
    }
    let mut count = 0;
    let mut all_have_flashed = false;
    while !all_have_flashed {
        let mut exploded = true;
        for x in 0..fish_map.len() {
            for y in 0..fish_map[x].len() {
                let mut exploded_fish;
                {
                    exploded_fish = fish_map.get_mut(x).unwrap().get_mut(y).unwrap();
                }
                exploded_fish.value += 1;
            }
        }
        while exploded {
            exploded = false;
            for x in 0..fish_map.len() {
                for y in 0..fish_map[x].len() {
                    if fish_map[x][y].value > 9 {
                        let mut exploded_fish;
                        {
                            exploded_fish = fish_map.get_mut(x).unwrap().get_mut(y).unwrap();
                        }
                        if !exploded_fish.was_activated {
                            exploded_fish.was_activated = true;
                            spread_from(&mut fish_map, x as i32, y as i32);
                            exploded = true;
                        }
                    }
                }
            }
        }
        all_have_flashed = true;
        for x in 0..fish_map.len() {
            for y in 0..fish_map[x].len() {
                if fish_map[x][y].was_activated {
                    let exploded_fish = fish_map.get_mut(x).unwrap().get_mut(y).unwrap();
                    exploded_fish.value = 0;
                    exploded_fish.was_activated = false;
                } else {
                    all_have_flashed = false;
                }
            }
        }
        count += 1;
    }
    count
}
