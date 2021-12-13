use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};

use petgraph::{Graph, Undirected};
use petgraph::algo::all_simple_paths;
use petgraph::dot::Dot;
use petgraph::graph::{Node, node_index, NodeIndex, UnGraph, WalkNeighbors};
use petgraph::visit::{DfsPostOrder, EdgeRef, IntoNodeIdentifiers, NodeRef};

#[aoc(day12, part1)]
pub fn part1_chars(input: &str) -> i32 {
    let mut graph = UnGraph::<&str, i32>::new_undirected();
    let mut graph_hm: HashMap<String, NodeIndex> = HashMap::new();
    for line in input.lines() {
        let sl: Vec<&str> = line.split('-').collect();
        let start = sl[0];
        let destination = sl[1];
        if !graph_hm.contains_key(start) {
            let ni = graph.add_node(start);
            graph_hm.insert(start.to_string(), ni);
        }
        if !graph_hm.contains_key(destination) {
            let ni = graph.add_node(destination);
            graph_hm.insert(destination.to_string(), ni);
        }
        graph.add_edge(
            *graph_hm.get(start).unwrap(),
            *graph_hm.get(destination).unwrap(),
            1,
        );
    }

    let mut unique_paths = HashSet::<String>::new();
    for ni in graph.node_indices() {
        if *graph.node_weight(ni).unwrap() == "start" {
            print_permutations(
                &graph,
                ni,
                Vec::new(),
                &MultVisit {
                    count: 1,
                    weight: "".to_string(),
                },
                &mut unique_paths,
            );
        }
    }

    unique_paths.len() as i32
}

struct MultVisit {
    count: i32,
    weight: String,
}

fn print_permutations(
    graph: &Graph<&str, i32, Undirected>,
    node_index: NodeIndex,
    visited_ids: Vec<&str>,
    multi_visit: &MultVisit,
    unique_paths: &mut HashSet<String>,
) {
    if *graph.node_weight(node_index).unwrap() == "end" {
        unique_paths.insert(visited_ids.iter().map(|x| *x).collect());
        return;
    }

    let mut edges = graph.neighbors(node_index);
    while let Some(to_visit) = edges.next() {
        if graph
            .node_weight(to_visit)
            .unwrap()
            .chars()
            .any(|x| x.is_uppercase())
            || !visited_ids.contains(graph.node_weight(to_visit).unwrap())
            || (graph.node_weight(to_visit).unwrap().to_string() == multi_visit.weight
                && visited_ids
                    .iter()
                    .filter(|visited_weight| multi_visit.weight == visited_weight.to_string())
                    .count()
                    < multi_visit.count as usize)
        {
            let mut new_visited_ids: Vec<&str> = visited_ids.clone();
            new_visited_ids.push(graph.node_weight(node_index).unwrap());
            print_permutations(graph, to_visit, new_visited_ids, multi_visit, unique_paths);
        }
    }
}

#[aoc(day12, part2)]
pub fn part2_chars(input: &str) -> i32 {
    // let mut graph = UnGraph::<&str, i32>::new_undirected();
    // let mut graph_hm: HashMap<String, NodeIndex> = HashMap::new();
    // for line in input.lines() {
    //     let sl: Vec<&str> = line.split('-').collect();
    //     let start = sl[0];
    //     let destination = sl[1];
    //     if !graph_hm.contains_key(start) {
    //         let ni = graph.add_node(start);
    //         graph_hm.insert(start.to_string(), ni);
    //     }
    //     if !graph_hm.contains_key(destination) {
    //         let ni = graph.add_node(destination);
    //         graph_hm.insert(destination.to_string(), ni);
    //     }
    //     graph.add_edge(
    //         *graph_hm.get(start).unwrap(),
    //         *graph_hm.get(destination).unwrap(),
    //         1,
    //     );
    // }
    // let mut unique_paths = HashSet::<String>::new();
    // for ni in graph.node_indices() {
    //     if *graph.node_weight(ni).unwrap() == "start" {
    //         for node in graph.raw_nodes() {
    //             if node.weight != "start" && node.weight != "end" {
    //                 print_permutations(
    //                     &graph,
    //                     ni,
    //                     Vec::new(),
    //                     &MultVisit {
    //                         count: 2,
    //                         weight: node.weight.to_string(),
    //                     },
    //                     &mut unique_paths,
    //                 );
    //             }
    //         }
    //     }
    // }
    //
    // unique_paths.len() as i32
    0
}
