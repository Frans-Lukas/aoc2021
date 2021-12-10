
#[aoc(day9, part1)]
pub fn part1_chars(input: &str) -> i32 {
    let mut height_map: Vec<Vec<i32>> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        height_map.push(Vec::new());
        for c in line.chars() {
            height_map
                .get_mut(i)
                .unwrap()
                .push(c.to_digit(10).unwrap() as i32);
        }
    }

    let mut sum = 0;
    for (i, row) in height_map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if !has_lower_neighbours(&height_map, i, j) {
                sum += cell + 1;
            }
        }
    }

    sum
}

fn has_lower_neighbours(height_map: &Vec<Vec<i32>>, x: usize, y: usize) -> bool {
    let mut has_lower = false;
    let curr = height_map[x][y];
    if height_map.len() > x + 1 && height_map[x + 1][y] <= curr {
        has_lower = true
    }
    if x > 0 && height_map[x - 1][y] <= curr {
        has_lower = true
    }
    if height_map[x].len() > y + 1 && height_map[x][y + 1] <= curr {
        has_lower = true
    }
    if y > 0 && height_map[x][y - 1] <= curr {
        has_lower = true
    }
    has_lower
}

#[aoc(day9, part2)]
pub fn part2_chars(input: &str) -> i32 {
    let mut height_map: Vec<Vec<i32>> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        height_map.push(Vec::new());
        for c in line.chars() {
            height_map
                .get_mut(i)
                .unwrap()
                .push(c.to_digit(10).unwrap() as i32);
        }
    }

    let mut results: Vec<i32> = Vec::new();
    for (i, row) in height_map.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if !has_lower_neighbours(&height_map, i, j) {
                let mut basin_count = 0;
                let mut taken_positions: Vec<Position> = Vec::new();
                results.push(find_basin(
                    &height_map,
                    i,
                    j,
                    &mut basin_count,
                    &mut taken_positions,
                ));
            }
        }
    }
    results.sort();
    results.reverse();
    results[0] * results[1] * results[2]
}

struct Position {
    x: usize,
    y: usize,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Position {}

fn find_basin(
    height_map: &Vec<Vec<i32>>,
    x: usize,
    y: usize,
    basin_size: &mut i32,
    taken_positions: &mut Vec<Position>,
) -> i32 {
    let curr = height_map[x][y];
    if height_map.len() > x + 1 && height_map[x + 1][y] > curr && height_map[x + 1][y] < 9 {
        if !taken_positions.contains(&Position { x: x + 1, y }) {
            taken_positions.push(Position { x: x + 1, y });
            *basin_size = find_basin(height_map, x + 1, y, basin_size, taken_positions);
        }
    }
    if x > 0 && height_map[x - 1][y] > curr && height_map[x - 1][y] < 9 {
        if !taken_positions.contains(&Position { x: x - 1, y }) {
            taken_positions.push(Position { x: x - 1, y });
            *basin_size = find_basin(height_map, x - 1, y, basin_size, taken_positions);
        }
    }
    if height_map[x].len() > y + 1 && height_map[x][y + 1] > curr && height_map[x][y + 1] < 9 {
        if !taken_positions.contains(&Position { x, y: y + 1 }) {
            taken_positions.push(Position { x, y: y + 1 });
            *basin_size = find_basin(height_map, x, y + 1, basin_size, taken_positions);
        }
    }
    if y > 0 && height_map[x][y - 1] > curr && height_map[x][y - 1] < 9 {
        if !taken_positions.contains(&Position { x, y: y - 1 }) {
            taken_positions.push(Position { x, y: y - 1 });
            *basin_size = find_basin(height_map, x, y - 1, basin_size, taken_positions);
        }
    }
    *basin_size + 1
}
