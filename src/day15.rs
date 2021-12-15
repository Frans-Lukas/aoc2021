use std::cmp::max;
use std::collections::{HashMap, HashSet};

use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::Graph;

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

#[aoc(day15, part1)]
pub fn part1_chars(input: &str) -> i32 {
    let mut danger_map = HashMap::<Point, NodeIndex>::new();
    let mut height = 0;
    let mut width = 0;

    let mut danger_graph = Graph::new();
    for line in input.lines() {
        width = 0;
        for c in line.chars() {
            let risk_level = c.to_digit(10).unwrap() as i32;
            let node = danger_graph.add_node(risk_level);
            danger_map.insert(
                Point {
                    x: width,
                    y: height,
                },
                node,
            );
            width += 1;
        }
        height += 1;
    }

    let directions: Vec<[i32; 2]> = vec![[0, 1], [1, 0], [-1, 0], [0, -1]];
    for x in 0..width {
        for y in 0..height {
            let mut node_index = danger_map.get(&Point { x, y }).unwrap();
            for dir in directions.iter() {
                let neighbour_point = Point {
                    x: x + dir[0],
                    y: y + dir[1],
                };
                if danger_map.contains_key(&neighbour_point) {
                    let neighbour_index = danger_map.get(&neighbour_point).unwrap();
                    danger_graph.add_edge(
                        *node_index,
                        *neighbour_index,
                        *danger_graph.node_weight(*neighbour_index).unwrap(),
                    );
                }
            }
        }
    }

    let start = *danger_map.get(&Point { x: 0, y: 0 }).unwrap();
    let goal = *danger_map
        .get(&Point {
            x: width - 1,
            y: height - 1,
        })
        .unwrap();
    let path = petgraph::algo::astar(
        &danger_graph,
        start,
        |finish| finish == goal,
        |e| *e.weight(),
        |_| 0,
    );

    path.unwrap().0 as i32
}

fn calc_risk_level(old_weight: i32, exp: i32) -> i32 {
    let mut new_weight = old_weight + exp;
    if new_weight >= 10 {
        return new_weight % 10 + 1;
    }
    return new_weight;
}

#[aoc(day15, part2)]
pub fn part2_chars(input: &str) -> i32 {
    let mut danger_map = HashMap::<Point, NodeIndex>::new();
    let mut height = 0;
    let mut width = 0;

    let mut danger_graph = Graph::new();
    for line in input.lines() {
        width = 0;
        for c in line.chars() {
            let risk_level = c.to_digit(10).unwrap() as i32;
            let node = danger_graph.add_node(risk_level);
            danger_map.insert(
                Point {
                    x: width,
                    y: height,
                },
                node,
            );
            width += 1;
        }
        height += 1;
    }
    //
    // println!("w: {}, h: {}", width, height);
    // for x in 0..width {
    //     println!();
    //     for y in 0..height {
    //         // println!("{},{}", x, y);
    //         let ni = danger_map.get(&Point { y, x }).unwrap();
    //         print!("{} ", danger_graph.node_weight(*ni).unwrap())
    //     }
    // }
    // println!();
    let mut goal = *danger_map
        .get(&Point {
            x: width - 1,
            y: height - 1,
        })
        .unwrap();
    for x in 0..width * 5 {
        for y in 0..height * 5 {
            // println!("{}. {}, x/w: {}", x %width, y %height, x/width);
            let risk_level = calc_risk_level(
                *danger_graph
                    .node_weight(
                        *danger_map
                            .get(&Point {
                                x: x % width,
                                y: y % height,
                            })
                            .unwrap(),
                    )
                    .unwrap(),
                x / width + y / height,
            );
            let node = danger_graph.add_node(risk_level);
            // println!("x:{}, y:{}", x, y);
            danger_map.insert(Point { x, y }, node);
            goal = node;
        }
    }
    //
    // println!("w: {}, h: {}", width, height);
    // for x in 0..width * 5 {
    //     println!();
    //     for y in 0..height * 5 {
    //         // println!("{},{}", x, y);
    //         if danger_map.contains_key(&Point { x, y }) {
    //             let ni = danger_map.get(&Point { x, y }).unwrap();
    //             print!("{} ", danger_graph.node_weight(*ni).unwrap())
    //         } else {
    //             print!("  ");
    //         }
    //     }
    // }
    // println!();

    let directions: Vec<[i32; 2]> = vec![[0, 1], [1, 0], [-1, 0], [0, -1]];
    for x in 0..width * 5 {
        for y in 0..height * 5 {
            // println!("x: {}, y: {}", x, y);
            // if danger_map.contains_key(&Point { x, y }) {
            let mut node_index = danger_map.get(&Point { x, y }).unwrap();
            for dir in directions.iter() {
                let neighbour_point = Point {
                    x: x + dir[0],
                    y: y + dir[1],
                };
                if danger_map.contains_key(&neighbour_point) {
                    let neighbour_index = danger_map.get(&neighbour_point).unwrap();
                    danger_graph.add_edge(
                        *node_index,
                        *neighbour_index,
                        *danger_graph.node_weight(*neighbour_index).unwrap(),
                    );
                }
            }
            // }
        }
    }

    let start = *danger_map.get(&Point { x: 0, y: 0 }).unwrap();
    let path = petgraph::algo::astar(
        &danger_graph,
        start,
        |finish| finish == goal,
        |e| *e.weight(),
        |_| 0,
    );

    path.unwrap().0 as i32
}
