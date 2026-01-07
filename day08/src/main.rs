use common::load;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    println!("Day 8, part {}", if cfg!(feature="part2") { "2" } else { "1" });
    let lines = load::lines().unwrap();

    // Load the path
    let path: Vec<char> = lines[0].chars().collect();

    // Load the graph
    let mut graph: HashMap<String, (String, String)> = HashMap::new();
    let graph_regex = Regex::new(r"(\w+)\s*=\s*\((\w+),\s*(\w+)\)").unwrap();
    for line in &lines[2..] {
        if let Some(captures) = graph_regex.captures(line) {
            let node = captures.get(1).map(|m| m.as_str().to_string());
            let left = captures.get(2).map(|m| m.as_str().to_string());
            let right = captures.get(3).map(|m| m.as_str().to_string());
            graph.insert(node.unwrap(), (left.unwrap(), right.unwrap()));
        }
    }

    // Find the node names ending in 'A'
    let mut ghosts: Vec<String> = Vec::new();
    for node_name in graph.keys() {
        if node_name.chars().nth(2).unwrap() == 'A' {
            ghosts.push(node_name.clone());
        }
    }

    // Put the ghosts at their starting nodes
    let mut ghost_node_names: Vec<&String> = Vec::new();
    for node_name in &ghosts {
        ghost_node_names.push(node_name)
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

        let mut count:usize = 0;
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
    
    let mut product:i64 = 1;
    for stat in stats {
        debug_assert!(stat.second == stat.first * 2);
        debug_assert!(stat.first % 293 == 0);
        product *= (stat.first / 293) as i64;
    }
    product *= 293;
    println!("Product: {}", product);
}

fn step<'a>(graph: &'a HashMap<String, (String, String)>, node_name: &String, direction: char) -> &'a String {
    let node = graph.get(node_name).unwrap();
    if direction == 'L' {
        return &node.0;
    } else {
        return &node.1;
    }
}
