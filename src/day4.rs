use std::cmp::max;
use std::collections::HashMap;

#[aoc(day4, part1)]
pub fn part1_chars(input: &str) -> i32 {
    let mut game_numbers: Vec<i32> = Vec::new();
    let mut bingo_grids: Vec<BingoGrid> = Vec::new();
    let mut bingo_grid_index = 0;
    let mut str_grid: Vec<&str> = vec![];
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            game_numbers = line.split(',').map(|x| x.parse().unwrap()).collect();
        } else if line.len() == 0 {
            if bingo_grid_index > 0 {
                bingo_grids[bingo_grid_index - 1].parse_grid(str_grid);
            }
            bingo_grid_index += 1;
            bingo_grids.push(BingoGrid { grid: vec![] });
            str_grid = Vec::new();
        } else {
            str_grid.push(line);
        }
    }
    bingo_grids[bingo_grid_index - 1].parse_grid(str_grid);
    for num in game_numbers {
        for grid in bingo_grids.iter_mut() {
            let score = grid.search_bingo_num(num);
            if score > 0 {
                return score;
            }
        }
    }
    unreachable!()
}

#[aoc(day4, part2)]
pub fn part2_chars(input: &str) -> i32 {
    let mut game_numbers: Vec<i32> = Vec::new();
    let mut bingo_grids: Vec<BingoGrid> = Vec::new();
    let mut bingo_grid_index = 0;
    let mut str_grid: Vec<&str> = vec![];
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            game_numbers = line.split(',').map(|x| x.parse().unwrap()).collect();
        } else if line.len() == 0 {
            if bingo_grid_index > 0 {
                bingo_grids[bingo_grid_index - 1].parse_grid(str_grid);
            }
            bingo_grid_index += 1;
            bingo_grids.push(BingoGrid { grid: vec![] });
            str_grid = Vec::new();
        } else {
            str_grid.push(line);
        }
    }
    bingo_grids[bingo_grid_index - 1].parse_grid(str_grid);
    let mut last_score = 0;
    for num in game_numbers {
        for grid in bingo_grids.iter_mut() {
            let score = grid.search_bingo_num(num);
            if score > 0 {
                last_score = score;
                let mut sum = 0;
                for line in grid.grid.iter() {
                    for cell in line {
                        if !cell.is_found {
                            sum += cell.number;
                        }
                    }
                }
            }
        }
        bingo_grids.retain(|mut grid| !grid.has_victory());
    }
    last_score
}

struct BingoCell {
    number: i32,
    is_found: bool,
}

struct BingoGrid {
    grid: Vec<Vec<BingoCell>>,
}

impl BingoGrid {
    pub fn parse_grid(&mut self, str_grid: Vec<&str>) {
        for line in str_grid {
            let mut new_bingo_line: Vec<BingoCell> = Vec::new();
            for num in line
                .split(' ')
                .filter(|l| l.len() >= 1)
                .map(|n| n.parse().unwrap())
            {
                new_bingo_line.push(BingoCell {
                    number: num,
                    is_found: false,
                })
            }
            self.grid.push(new_bingo_line);
        }
    }
    pub fn search_bingo_num(&mut self, bingo_num: i32) -> (i32) {
        for line in self.grid.iter_mut() {
            for mut cell in line {
                if cell.number == bingo_num {
                    cell.is_found = true;
                }
            }
        }
        {
            if self.has_victory() {
                let vic_score: i32 = self.calculate_score(bingo_num);
                return vic_score;
            }
        }
        0
    }

    fn has_victory(&self) -> bool {
        if self.grid.len() == 0 {
            return false;
        }
        self.all_horizontals() || self.all_verticals() //|| self.diagonals()
    }

    fn all_horizontals(&self) -> bool {
        for line in self.grid.iter() {
            let mut victory = true;
            for cell in line {
                if !cell.is_found {
                    victory = false;
                }
            }
            if victory {
                return victory;
            }
        }
        false
    }

    fn all_verticals(&self) -> bool {
        for i in 0..self.grid.len() {
            let mut victory = true;
            for j in 0..self.grid[0].len() {

                if !self.grid[j][i].is_found {
                    victory = false;
                }
            }
            if victory {
                return victory;
            }
        }
        false
    }

    fn diagonals(&self) -> bool {
        if self.top_left_to_bottom_right() || self.top_right_to_bottom_left() {
            for line in self.grid.iter() {
                println!();
                for cell in line.iter() {
                    print!("{} ", cell.number);
                }
            }

            let one = self.grid[0][4].is_found;
            let two = self.grid[1][3].is_found;
            let three = self.grid[2][2].is_found;
            let four = self.grid[3][1].is_found;
            let five = self.grid[4][0].is_found;
            println!("diagonals vic!");
            return true;
        }

        false
    }

    fn top_right_to_bottom_left(&self) -> bool {
        self.grid[0][4].is_found
            && self.grid[1][3].is_found
            && self.grid[2][2].is_found
            && self.grid[3][1].is_found
            && self.grid[4][0].is_found
    }
    fn top_left_to_bottom_right(&self) -> bool {
        self.grid[0][0].is_found
            && self.grid[1][1].is_found
            && self.grid[2][2].is_found
            && self.grid[3][3].is_found
            && self.grid[4][4].is_found
    }
    fn calculate_score(&self, vic_num: i32) -> i32 {
        let mut sum = 0;
        for line in self.grid.iter() {
            for cell in line {
                if !cell.is_found {
                    sum += cell.number;
                }
            }
        }
        sum * vic_num
    }
}
