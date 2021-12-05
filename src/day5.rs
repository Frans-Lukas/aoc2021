use std::cmp::max;
use std::collections::HashMap;

#[aoc(day5, part1)]
pub fn part1_chars(input: &str) -> i32 {
    let mut found_points: HashMap<Point, i32> = HashMap::new();
    let mut counter = 0;
    for line in input.lines() {
        let str_points: Vec<&str> = line.split("->").map(|p| p.trim()).collect();
        let p1: Vec<&str> = str_points[0].split(',').collect();
        let p2: Vec<&str> = str_points[1].split(',').collect();
        let start_p = Point {
            x: p1[0].parse().unwrap(),
            y: p1[1].parse().unwrap(),
        };
        let end_p = Point {
            x: p2[0].parse().unwrap(),
            y: p2[1].parse().unwrap(),
        };

        if start_p.x == end_p.x || start_p.y == end_p.y {
            for x in start_p.sx(&end_p)..start_p.lx(&end_p) + 1 {
                for y in start_p.sy(&end_p)..start_p.ly(&end_p) + 1 {
                    insert_and_count(&mut found_points, &mut counter, x, y);
                }
            }
        }
    }

    counter
}

#[aoc(day5, part2)]
pub fn part2_chars(input: &str) -> i32 {
    let mut found_points: HashMap<Point, i32> = HashMap::new();
    let mut counter = 0;
    for line in input.lines() {
        let str_points: Vec<&str> = line.split("->").map(|p| p.trim()).collect();
        let p1: Vec<&str> = str_points[0].split(',').collect();
        let p2: Vec<&str> = str_points[1].split(',').collect();
        let mut start_p = Point {
            x: p1[0].parse().unwrap(),
            y: p1[1].parse().unwrap(),
        };
        let mut end_p = Point {
            x: p2[0].parse().unwrap(),
            y: p2[1].parse().unwrap(),
        };

        if start_p.is_dia(&end_p) {
            while start_p != end_p{
                insert_and_count(&mut found_points, &mut counter, start_p.x, start_p.y);
                start_p.step_closer(&end_p);
            }
            insert_and_count(&mut found_points, &mut counter, start_p.x, start_p.y);
        } else if start_p.x == end_p.x || start_p.y == end_p.y {
            for x in start_p.sx(&end_p)..start_p.lx(&end_p) + 1 {
                for y in start_p.sy(&end_p)..start_p.ly(&end_p) + 1 {
                    insert_and_count(&mut found_points, &mut counter, x, y);
                }
            }
        }
    }

    counter
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn overlaps(self, other: &Line) {}
}

#[derive(Debug, Hash, Clone, Copy)]
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

impl Point {
    pub fn sx(self, p: &Point) -> i32 {
        if self.x < p.x {
            return self.x;
        }
        p.x
    }
    pub fn sy(self, p: &Point) -> i32 {
        if self.y < p.y {
            return self.y;
        }
        p.y
    }
    pub fn lx(self, p: &Point) -> i32 {
        if self.x > p.x {
            return self.x;
        }
        p.x
    }
    pub fn ly(self, p: &Point) -> i32 {
        if self.y > p.y {
            return self.y;
        }
        p.y
    }

    pub fn is_dia(self, p: &Point) -> bool {
        (self.x - p.x).abs() == (self.y - p.y).abs()
    }

    pub fn step_closer(&mut self, p: &Point) {
        if self.x > p.x {
            self.x -= 1;
        } else if self.x < p.x {
            self.x += 1;
        }

        if self.y > p.y {
            self.y -= 1;
        } else if self.y < p.y {
            self.y += 1;
        }
    }
}


fn insert_and_count(
    found_points: &mut HashMap<Point, i32>,
    mut counter: &mut i32,
    sx: i32,
    sy: i32,
) {
    match found_points.get_mut(&Point { x: sx, y: sy }) {
        Some(p) => {
            if *p == 1 {
                *counter += 1;
            }
            *p += 1;
        }
        None => {
            found_points.insert(Point { x: sx, y: sy }, 1);
        }
    }
}
