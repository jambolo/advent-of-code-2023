use common::load;
use regex::Regex;
use std::collections::HashMap;

type Graph = HashMap<String, (String, String)>;

fn main() {
    println!("=== Day 8, part {} ===", if cfg!(feature = "part2") { "2" } else { "1" });
    let lines = load::lines().unwrap();

    // Load the path
    let path: Vec<char> = lines[0].chars().collect();

    // Load the graph
    let mut graph: Graph = HashMap::new();
    let graph_regex = Regex::new(r"(\w+)\s*=\s*\((\w+),\s*(\w+)\)").unwrap();
    for line in &lines[2..] {
        if let Some(captures) = graph_regex.captures(line) {
            let node = captures.get(1).map(|m| m.as_str().to_string());
            let left = captures.get(2).map(|m| m.as_str().to_string());
            let right = captures.get(3).map(|m| m.as_str().to_string());
            graph.insert(node.unwrap(), (left.unwrap(), right.unwrap()));
        }
    }

    let result = if cfg!(feature = "part2") { part2(&graph, &path) } else { part1(&graph, &path) };
    println!("Result: {}", result);
}

fn part2(graph: &Graph, path: &[char]) -> i64 {
    // Find the node names ending in 'A'
    let mut ghosts: Vec<String> = Vec::new();
    for node_name in graph.keys() {
        if node_name.chars().nth(2).unwrap() == 'A' {
            ghosts.push(node_name.clone());
        }
    }

    struct Stat {
        end: String,
        first: i32,
        second: i32,
    }

    let mut stats: Vec<Stat> = Vec::new();

    for ghost in ghosts {
        let mut stat = Stat {
            end: String::new(),
            first: 0,
            second: 0,
        };

        let mut count: usize = 0;
        let mut done = false;
        let mut node_name = &ghost;
        while !done {
            let direction = path[count % path.len()];
            node_name = step(&graph, node_name, direction);
            count += 1;
            if node_name.chars().nth(2).unwrap() == 'Z' {
                if stat.first == 0 {
                    stat.end = node_name.clone();
                    stat.first = count as i32;
                } else {
                    stat.second = count as i32;
                    done = true;
                }
            }
        }
        stats.push(stat);
    }

    let mut product: i64 = 1;
    for stat in stats {
        debug_assert!(stat.second == stat.first * 2);
        debug_assert!(stat.first % 293 == 0);
        product *= (stat.first / 293) as i64;
    }

    product * 293
}

fn part1(graph: &Graph, path: &[char]) -> i64 {
    // Part 1: Walk from AAA to ZZZ
    let mut count: usize = 0;
    let mut node_name = "AAA";
    while node_name != "ZZZ" {
        let direction = path[count % path.len()];
        node_name = step(&graph, node_name, direction);
        count += 1;
    }

    count as i64
}

fn step<'a>(graph: &'a Graph, node_name: &str, direction: char) -> &'a String {
    let node = graph.get(node_name).unwrap();
    if direction == 'L' { &node.0 } else { &node.1 }
}
