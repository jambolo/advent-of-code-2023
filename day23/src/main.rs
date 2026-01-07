use std::collections::HashMap;
use common::load;

#[derive(Debug)]
struct Node {
    x: usize,
    y: usize,
    edges: HashMap<usize, i32>,
}

impl Node {
    fn new(x: usize, y: usize) -> Node {
        Node { x, y, edges: HashMap::new() }
    }
    fn add_edge(&mut self, to: usize, cost: i32) {
        self.edges.insert(to, cost);
    }
    fn find_edge(&self, to: usize) -> Option<i32> {
        self.edges.get(&to).map(|c| *c)
    }
}

#[derive(Debug)]
struct Path {
    _nodes: Vec<usize>,
    cost: i32,
}

fn main() {
    println!("Day 23, part {}", if cfg!(feature="part2") { "2" } else { "1" });

    // Load the map
    let map = load::map().unwrap();

    let start = (1, 0);
    let goal = (map.len() - 2, map[0].len() - 1);

    // Let's build a graph depth-first
    let mut nodes = Vec::new();
    nodes.push(Node::new(start.0, start.1));
    nodes.push(Node::new(goal.0, goal.1));

    follow_path(&map, 0, (0, 1), false, &mut nodes);


    // Now we have a graph, let's find all of the paths
    let paths = enumerate_paths(&nodes, 0, 1);
    let max_cost = paths.iter().map(|p| p.cost).max().unwrap();
    println!("Max cost: {}", max_cost);
}

fn follow_path(map: &Vec<Vec<char>>, from: usize, mut dir: (i32, i32), mut directed: bool, nodes: &mut Vec<Node>) {
    let from_node = &nodes[from];
    let mut pos = ((from_node.x as i32 + dir.0) as usize, (from_node.y as i32 + dir.1) as usize);
    let mut cost = 1;
    let mut next_dir;
    loop {
        // If this position is a node, add edges and stop following the path
        if let Some(to) = find_node_at(pos, nodes) {
            debug_assert!(from_node.find_edge(to).is_none());
            nodes[from].add_edge(to, cost);
            if !directed {
                let to_node = &nodes[to];
                if let Some(existing_cost) = to_node.find_edge(from) {
                    debug_assert!(existing_cost == cost);
                } else {
                    nodes[to].add_edge(from, cost);
                }
            }
            break;
        }
        let c = map[pos.1][pos.0];
        match c {
            '>' => {
                if cfg!(feature="part2") {
                    let directions = next_directions(&map, pos, dir);
                    if is_node(&map, pos) {
                        // This is a new node
                        let to = add_node(nodes, pos, from, cost, directed);
                        for d in directions {
                            follow_path(map, to, d, false, nodes);
                        }
                        break; // reachded node so done with this path
                    } else if directions.len() == 0 {
                        break;  // dead end
                    }
                    debug_assert!(directions.len() == 1);
                    next_dir = directions[0];
                } else {
                    if dir.0 == -1 && dir.1 == 0 {
                        break;  // blocked
                    } else {
                        next_dir = (1, 0);
                        if is_node(&map, pos) {
                            // This is a new node
                            let to = add_node(nodes, pos, from, cost, true);
                            follow_path(map, to, next_dir, true, nodes);
                            break; // reached node so done with this path
                        } else {
                            directed = true; // the path is now directed
                        }
                    }
                }
            },
            '^' => {
                if cfg!(feature="part2") {
                    let directions = next_directions(&map, pos, dir);
                    if is_node(&map, pos) {
                        // This is a new node
                        let to = add_node(nodes, pos, from, cost, directed);
                        for d in directions {
                            follow_path(map, to, d, false, nodes);
                        }
                        break; // reachded node so done with this path
                    } else if directions.len() == 0 {
                        break;  // dead end
                    }
                    debug_assert!(directions.len() == 1);
                    next_dir = directions[0];
                } else {
                    if dir.0 == 0 && dir.1 == 1 {
                        break;  // blocked
                    } else {
                        next_dir = (0, -1);
                        if is_node(&map, pos) {
                            // This is a new node
                            let to = add_node(nodes, pos, from, cost, true);
                            follow_path(map, to, next_dir, true, nodes);
                            break; // reached node so done with this path
                        } else {
                            directed = true; // the path is now directed
                        }
                    }
                }
            },
            '<' => {
                if cfg!(feature="part2") {
                    let directions = next_directions(&map, pos, dir);
                    if is_node(&map, pos) {
                        // This is a new node
                        let to = add_node(nodes, pos, from, cost, directed);
                        for d in directions {
                            follow_path(map, to, d, false, nodes);
                        }
                        break; // reachded node so done with this path
                    } else if directions.len() == 0 {
                        break;  // dead end
                    }
                    debug_assert!(directions.len() == 1);
                    next_dir = directions[0];
                } else {
                    if dir.0 == 1 && dir.1 == 0 {
                        break;  // blocked
                    } else {
                        next_dir = (-1, 0);
                        if is_node(&map, pos) {
                            // This is a new node
                            let to = add_node(nodes, pos, from, cost, true);
                            follow_path(map, to, next_dir, true, nodes);
                            break; // reached node so done with this path
                        } else {
                            directed = true; // the path is now directed
                        }
                    }
                }
            },
            'v' => {
                if cfg!(feature="part2") {
                    let directions = next_directions(&map, pos, dir);
                    if is_node(&map, pos) {
                        // This is a new node
                        let to = add_node(nodes, pos, from, cost, directed);
                        for d in directions {
                            follow_path(map, to, d, false, nodes);
                        }
                        break; // reachded node so done with this path
                    } else if directions.len() == 0 {
                        break;  // dead end
                    }
                    debug_assert!(directions.len() == 1);
                    next_dir = directions[0];
                } else {
                    if dir.0 == 0 && dir.1 == -1 {
                        break;  // blocked
                    } else {
                        next_dir = (0, 1);
                        if is_node(&map, pos) {
                            // This is a new node
                            let to = add_node(nodes, pos, from, cost, true);
                            follow_path(map, to, next_dir, true, nodes);
                            break; // reached node so done with this path
                        } else {
                            directed = true; // the path is now directed
                        }
                    }
                }
            },
            '.' => {
                let directions = next_directions(&map, pos, dir);
                if is_node(&map, pos) {
                    // This is a new node
                    let to = add_node(nodes, pos, from, cost, directed);
                    for d in directions {
                        follow_path(map, to, d, false, nodes);
                    }
                    break; // reachded node so done with this path
                } else if directions.len() == 0 {
                    break;  // dead end
                }
                debug_assert!(directions.len() == 1);
                next_dir = directions[0];
            },
            _ => {
                panic!("Unexpected: {} at ({}, {})", c, pos.0, pos.1);
            }
        }
        pos.0 = (pos.0 as i32 + next_dir.0) as usize;
        pos.1 = (pos.1 as i32 + next_dir.1) as usize;
        dir = next_dir;
        cost += 1;
    }
}

fn add_node(nodes: &mut Vec<Node>, pos: (usize, usize), from: usize, cost: i32, directed:bool) -> usize {
    nodes.push(Node::new(pos.0, pos.1));
    let to = nodes.len() - 1;
    nodes[from].add_edge(to, cost);
    if !directed {
        nodes[to].add_edge(from, cost);
    }
    to
}

fn next_directions(map: &Vec<Vec<char>>, pos: (usize, usize), dir: (i32, i32)) -> Vec<(i32, i32)> {
    let width = map[0].len();
    let height = map.len();
    let mut directions = Vec::new();
    // Right
    if dir.0 != -1 && pos.0 < width - 1 && map[pos.1][pos.0 + 1] != '#' && map[pos.1][pos.0 + 1] != '<' {
        directions.push((1, 0));
    }
    // Up
    if dir.1 != 1 && pos.1 > 0 && map[pos.1 - 1][pos.0] != '#' && map[pos.1 - 1][pos.0] != 'v' {
        directions.push((0, -1));
    }
    // Left
    if dir.0 != 1 && pos.0 > 0 && map[pos.1][pos.0 - 1] != '#' && map[pos.1][pos.0 - 1] != '>' {
        directions.push((-1, 0));
    }
    // Down
    if dir.1 != -1 && pos.1 < height - 1 && map[pos.1 + 1][pos.0] != '#' && map[pos.1 + 1][pos.0] != '^' {
        directions.push((0, 1));
    }
    directions
}

fn find_node_at(pos: (usize, usize), nodes: &Vec<Node>) -> Option<usize> {
    nodes.iter().enumerate().find(|(_, node)| node.x == pos.0 && node.y == pos.1).map(|(i, _)| i)
}

fn is_node(map: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    let width = map[0].len();
    let height = map.len();
    let mut path_count = 0;
    // Right
    if pos.0 < width - 1 && map[pos.1][pos.0 + 1] != '#' {
        path_count += 1;
    }
    // Up
    if pos.1 > 0 && map[pos.1 - 1][pos.0] != '#' {
        path_count += 1;
    }
    // Left
    if pos.0 > 0 && map[pos.1][pos.0 - 1] != '#' {
        path_count += 1;
    }
    // Down
    if pos.1 < height - 1 && map[pos.1 + 1][pos.0] != '#' {
        path_count += 1;
    }
    path_count > 2
}

fn enumerate_paths(nodes: &Vec<Node>, from: usize, goal: usize) -> Vec<Path> {
    let mut paths = Vec::new();
    let mut path = Vec::new();
    path.push(from);
    enumerate_paths_rec(nodes, from, goal, path.clone(), 0,  &mut paths);
    paths
}

fn enumerate_paths_rec(nodes: &Vec<Node>, from: usize, goal: usize, mut path: Vec<usize>, total_cost: i32, paths: &mut Vec<Path>) {
    let from_node = &nodes[from];
    for (next, cost) in &from_node.edges {
        if *next == goal {
            path.push(goal);
            paths.push(Path { _nodes: path.clone(), cost: total_cost + cost });
            path.pop();
        } else if !path.contains(next) {
            path.push(*next);
            enumerate_paths_rec(nodes, *next, goal, path.clone(), total_cost + cost, paths);
            path.pop();
        }
    }
}