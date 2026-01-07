use std::collections::HashMap;
use common::load;

fn main() {
    println!("Day 25, part {}", if cfg!(feature="part2") { "2" } else { "1" });

    // Load the data
    let lines = load::lines().unwrap();
    let mut graph = parse_graph(&lines);
    println!("Graph: {:?} entries", graph.len());

    // Create a mapping from node pairs to their edge id
    let (edge_map, edge_id_map) = create_edge_maps(&graph);
    println!("Edge map: {:?} entries", edge_map.len());
    println!("Edge id map: {:?} entries", edge_id_map.len());

    // Ideally, you would find the three edges that when removed would split the graph into two disjoint graphs.
    // However, the cost of finding the three edges is too high, so we try a different approach. Since the farthest
    // node from any node is guaranteed (?) to go through at least one of the three edges, disconnecting the
    // path is guaranteed to remove at least one of the three edges. Do that three times and the graph is split.
    // Since every node is guaranteed to have more than 3 neighbors, (otherwise, the problem is trivial), we just
    // find the longest path from the first node three times. In case, removing the path removes more than one of
    // the three edges, we check if the graph is split after each iteration. This all sounds good, but I don't know
    // it correct in theory.
    
    let node0 = *graph.keys().next().unwrap();
    let mut pass = 1;
    while !any_node_unreachable_from(&graph, node0) {
        println!("Pass {}", pass);
        let longest_path: Vec<usize> = find_path_from_farthest_node(&graph, node0);
        for w in longest_path.windows(2) {
            remove_edge(&mut graph, &(w[0], w[1]));
        }
        pass += 1;
    }
    let a = count_reachable_nodes(&graph, node0);
    let b = graph.len() - a;
    println!("Part 1: {}", a * b);
}

fn parse_graph(lines: &Vec<String>) -> HashMap<usize, Vec<usize>> {
    let mut graph = HashMap::new();
    let mut node_name_map = Vec::new();
    let mut node_id_map = HashMap::new();

    for line in lines {
        let (key, neighbors) = line.split_once(":").unwrap();
        let key_id = register_node(key.trim(), &mut node_id_map, &mut node_name_map);

        // Add the neighbors for the key and add key as a neighbor for each neighbor (expecting duplicates)
        for neighbor in neighbors.trim().split_whitespace() {
            let neighbor_id = register_node(neighbor, &mut node_id_map, &mut node_name_map);
            graph.entry(key_id).or_insert_with(Vec::new).push(neighbor_id);
            graph.entry(neighbor_id).or_insert_with(Vec::new).push(key_id);
        }
    }

    println!("Node name map: {:?} entries", node_name_map.len());

    // For each key, sort the neighbors and remove duplicates
    for neighbors in graph.values_mut() {
        neighbors.sort_unstable();
        neighbors.dedup();
    }
    graph
}

fn register_node<'a>(
    name: &'a str,
    node_id_map: &mut HashMap<&'a str, usize>,
    node_name_map: &mut Vec<&'a str>
) -> usize {
    if let Some(&id) = node_id_map.get(name) {
        id
    } else {
        let id = node_name_map.len();
        node_name_map.push(name);
        node_id_map.insert(name, id);
        id
    }
}

fn create_edge_maps(graph: &HashMap<usize, Vec<usize>>) -> (Vec<(usize, usize)>, HashMap<(usize, usize), usize>) {
    let mut edge_map = Vec::new();
    let mut edge_id_map = HashMap::new();
    for (&node, neighbors) in graph {
        for &neighbor in neighbors {
            if !edge_id_map.contains_key(&(node, neighbor)) {
                let edge_id = edge_map.len();
                edge_map.push((node, neighbor));
                // Note: The edge is stored twice, once for each direction
                edge_id_map.insert((node, neighbor), edge_id);
                edge_id_map.insert((neighbor, node), edge_id);
            }
        }
    }
    (edge_map, edge_id_map)
}

/// Returns the path from the farthest node back to the start node.
fn find_path_from_farthest_node(graph: &HashMap<usize, Vec<usize>>, start: usize) -> Vec<usize> {
    let mut visited = vec![false; graph.len()];
    let mut next_span = vec![start];
    let mut predecessors = vec![0; graph.len()];
    let mut farthest_node = start;
    while !next_span.is_empty() {
        let span = std::mem::take(&mut next_span);
        for &node in &span {
            for &neighbor in &graph[&node] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    predecessors[neighbor] = node;
                    next_span.push(neighbor);
                    farthest_node = neighbor;
                }
            }
        }
    }
    // Reconstruct the path from the farthest node to the start. Note that the path is reversed.
    let mut longest_path = Vec::new();
    let mut node = farthest_node;
    while node != start {
        longest_path.push(node);
        node = predecessors[node];
    }
    longest_path.push(start);

    longest_path
}

fn remove_edge(graph: &mut HashMap<usize, Vec<usize>>, edge: &(usize, usize)) {
    graph.get_mut(&edge.0).unwrap().retain(|&n| n != edge.1);
    graph.get_mut(&edge.1).unwrap().retain(|&n| n != edge.0);
}

fn count_reachable_nodes(graph: &HashMap<usize, Vec<usize>>, start: usize) -> usize {
    let mut visited = vec![false; graph.len()];
    let mut stack = vec![start];
    while let Some(node) = stack.pop() {
        if !visited[node] {
            visited[node] = true;
            stack.extend(graph[&node].iter());
        }
    }
    visited.iter().filter(|&&v| v).count()
}

fn any_node_unreachable_from(graph: &HashMap<usize, Vec<usize>>, start: usize) -> bool {
    count_reachable_nodes(graph, start) != graph.len()
}
