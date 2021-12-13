use std::collections::HashMap;

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

enum FoldInstruction {
    XFold(i32),
    YFold(i32),
}

#[aoc(day13, part1)]
pub fn part1_chars(input: &str) -> i32 {
    let mut map = HashMap::<Point, bool>::new();
    let mut fold_instructions = Vec::<FoldInstruction>::new();
    for line in input.lines() {
        if line.is_empty() || line.contains("fold") {
            if line.contains("fold") {
                let fold_instruction: Vec<&str> = line.split('=').collect();
                let value: i32 = fold_instruction[1].parse().unwrap();
                if fold_instruction[0].contains('x') {
                    fold_instructions.push(FoldInstruction::XFold(value))
                } else {
                    fold_instructions.push(FoldInstruction::YFold(value))
                }
            }
        } else {
            let coords: Vec<&str> = line.split(',').collect();
            let x = coords[0].parse().unwrap();
            let y = coords[1].parse().unwrap();
            map.insert(Point { x, y }, true);
        }
    }
    let first_instruction = fold_instructions.get(0).unwrap();
    map = map
        .iter()
        .map(|(point, _)| {
            let mut returnP = Point {
                x: point.x,
                y: point.y,
            };
            match first_instruction {
                FoldInstruction::XFold(fx) => {
                    if point.x > *fx {
                        returnP.x = *fx - (point.x - *fx);
                    }
                }
                FoldInstruction::YFold(fy) => {
                    if point.y > *fy {
                        returnP.y = *fy - (point.y - *fy);
                    }
                }
            }
            (returnP, true)
        })
        .collect();

    map.len() as i32
}

#[aoc(day13, part2)]
pub fn part2_chars(input: &str) -> i32 {
    let mut map = HashMap::<Point, bool>::new();
    let mut fold_instructions = Vec::<FoldInstruction>::new();
    for line in input.lines() {
        if line.is_empty() || line.contains("fold") {
            if line.contains("fold") {
                let fold_instruction: Vec<&str> = line.split('=').collect();
                let value: i32 = fold_instruction[1].parse().unwrap();
                if fold_instruction[0].contains('x') {
                    fold_instructions.push(FoldInstruction::XFold(value))
                } else {
                    fold_instructions.push(FoldInstruction::YFold(value))
                }
            }
        } else {
            let coords: Vec<&str> = line.split(',').collect();
            let x = coords[0].parse().unwrap();
            let y = coords[1].parse().unwrap();
            map.insert(Point { x, y }, true);
        }
    }
    for instruction in fold_instructions {
        map = map
            .iter()
            .map(|(point, _)| {
                let mut new_p = Point {
                    x: point.x,
                    y: point.y,
                };
                match instruction {
                    FoldInstruction::XFold(fx) => {
                        if point.x > fx {
                            new_p.x = fx - (point.x - fx);
                        }
                    }
                    FoldInstruction::YFold(fy) => {
                        if point.y > fy {
                            new_p.y = fy - (point.y - fy);
                        }
                    }
                }
                (new_p, true)
            })
            .collect();
    }
    let mut max_x = 0;
    let mut max_y = 0;
    for (p, b) in &map {
        if p.x > max_x {
            max_x = p.x;
        }
        if p.y > max_y {
            max_y = p.y;
        }
    }

    for y in 0..max_y + 1 {
        println!();
        for x in 0..max_x + 1 {
            if map.contains_key(&Point { x, y }) {
                print!("# ");
            } else {
                print!(". ");
            }
        }
    }
    println!();
    map.len() as i32
}
