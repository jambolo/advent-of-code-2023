use std::cmp::Ordering;
use std::collections::BinaryHeap;
use common::load;

const MIN_RUN: usize = if cfg!(feature="part2") { 4 } else { 1 };
const MAX_RUN: usize = if cfg!(feature="part2") { 10 } else { 3 };

fn main() {
    println!("Day 17, part {}", if cfg!(feature="part2") { "2" } else { "1" });

    let map = load::numbers_map().unwrap();
    let start: (usize, usize) = (0, 0);
    let goal: (usize, usize) = (map.len() - 1, map[0].len() - 1); // (row, column)

    // The heuristic is a map of the lowest unrestricted costs from each cell to the goal
    let lowest_unrestricted_costs = build_lowest_unrestricted_costs_map(&map, goal);
    let h = |(r, c): (usize, usize)| { lowest_unrestricted_costs[r][c] };

    let cost = shortest_path(start, goal, h, &map);
    println!("Shortest path: {}", cost);
}

// Builds a map of the shortest unrestricted distances from each cell to the goal using Dijkstra's algorithm
fn build_lowest_unrestricted_costs_map(edge_costs: &Vec<Vec<i32>>, goal: (usize, usize)) -> Vec<Vec<i32>> {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Node {
        location: (usize, usize),
        cost: i32,
    }
    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost) // Reverse order for min-heap
        }
    }
    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    
    fn get_neighbors(&(r, c): &(usize, usize), &(height, width): &(usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors: Vec<(usize, usize)> = Vec::new();
        if c >= 1 {
            neighbors.push((r, c - 1));
        }
        if c + 1 < width {
            neighbors.push((r, c + 1));
        }
        if r >= 1 {
            neighbors.push((r - 1, c));
        }
        if r + 1 < height {
            neighbors.push((r + 1, c));
        }
        neighbors
    }

    let width = edge_costs[0].len();
    let height = edge_costs.len();
    let mut costs_to_goal = vec![vec![std::i32::MAX; width]; height];

    let mut open = BinaryHeap::new();
    open.push(Node { location: (goal.0, goal.1), cost: 0 });    // Start from the goal

    while let Some(node) = open.pop() {
        if node.cost < costs_to_goal[node.location.0][node.location.1] {
            costs_to_goal[node.location.0][node.location.1] = node.cost;
            let neighbor_cost = node.cost + edge_costs[node.location.0][node.location.1];
            let neighbors: Vec<(usize, usize)> = get_neighbors(&node.location, &(height, width));
            for &(r, c) in &neighbors {
                if neighbor_cost < costs_to_goal[r][c] {
                    open.push(Node { location: (r, c), cost: neighbor_cost });
                }
            }
        }
    }
    costs_to_goal
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    position: (usize, usize, usize), // (r, c, d)
    f: i32,     // f = g + h
    g: i32,     // cost from start
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f) // Reverse order for min-heap
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(start: (usize, usize), goal: (usize, usize), h: impl Fn((usize, usize)) -> i32, map: &Vec<Vec<i32>>) -> i32
{
    let width = map[0].len();
    let height = map.len();
    let mut f = vec![vec![vec![std::i32::MAX; 2]; width]; height]; // f[r][c][d]
    let mut open: BinaryHeap<Node> = BinaryHeap::new();
    
    // Push the start node to the open set (both directions)
    open.push(Node { position: (start.0, start.1, 0), f: 0 + h(start), g: 0 });
    open.push(Node { position: (start.0, start.1, 1), f: 0 + h(start), g: 0 });

    while let Some(node) = open.pop() {
        if node.position.0 == goal.0 && node.position.1 == goal.1 {
            return node.g;
        }
        if node.f < f[node.position.0][node.position.1][node.position.2] {
            f[node.position.0][node.position.1][node.position.2] = node.f;
            let neighbors: Vec<Node> = get_neighbors(&node, &map);
            for mut n in neighbors {
                let neighbor_location = (n.position.0, n.position.1);
                n.f = n.g + h(neighbor_location);
                open.push(n);
            }
        }
    }
    std::i32::MAX
}

fn get_neighbors(node: &Node, map: &Vec<Vec<i32>>) -> Vec<Node> {
    let r = node.position.0;
    let c = node.position.1;
    let d = node.position.2;
    let width = map[0].len();
    let height = map.len();

    let mut neighbors = Vec::new();

    // Move left or right if not coming from left or right
    if d != 0 {
        // Push left movements only if not on the left edge.
        if c >= MIN_RUN {
        neighbors.extend(left_run(node, map));
        }
        // Push right only if not on the right edge
        if c + MIN_RUN < width {
            neighbors.extend(right_run(node, map, width));
        }
    }

    // Move up or down if not coming from up or down
    if d != 1 {
        // Push up only if not on the top edge
        if r >= MIN_RUN {
            neighbors.extend(up_run(node, map));
        }
        // Push down only if not on the bottom edge
        if r + MIN_RUN < height {
            neighbors.extend(down_run(node, map, height));
        }
    }

    neighbors
}

fn down_run(node: &Node, map: &Vec<Vec<i32>>, height: usize) -> Vec<Node> {
    let r  = node.position.0;
    let c = node.position.1;
    assert!(r + MIN_RUN < height);

    let mut accumulated_g = node.g + (1..MIN_RUN).map(|i| map[r + i][c]).sum::<i32>();
    let mut run: Vec<Node> = Vec::new();
    for nr in (r + MIN_RUN)..=std::cmp::min(r + MAX_RUN, height - 1) {
        accumulated_g += map[nr][c];
        let down = Node {
            position: (nr, c, 1),
            f: 0, // Filled in later
            g: accumulated_g,
        };
        run.push(down);
    }

    run
}

fn up_run(node: &Node, map: &Vec<Vec<i32>>) -> Vec<Node> {
    let r = node.position.0;
    let c = node.position.1;
    assert!(r >= MIN_RUN);

    let mut accumulated_g = node.g + (1..MIN_RUN).map(|i| map[r - i][c]).sum::<i32>();
    let mut run: Vec<Node> = Vec::new();
    for nr in (r.saturating_sub(MAX_RUN)..=(r - MIN_RUN)).rev() {
        accumulated_g += map[nr][c];
        let up = Node {
            position: (nr, c, 1),
            f: 0, // Filled in later
            g: accumulated_g,
        };
        run.push(up);
    }

    run
}

fn right_run(node: &Node, map: &Vec<Vec<i32>>, width: usize) -> Vec<Node> {
    let r = node.position.0;
    let c = node.position.1;
    assert!(c + MIN_RUN < width);

    let mut accumulated_g = node.g + (1..MIN_RUN).map(|i| map[r][c + i]).sum::<i32>();
    let mut run: Vec<Node> = Vec::new();
    for nc in (c + MIN_RUN)..=std::cmp::min(c + MAX_RUN, width - 1) {
        accumulated_g += map[r][nc];
        let right = Node {
            position: (r, nc, 0),
            f: 0, // Filled in later
            g: accumulated_g,
        };
        run.push(right);
    }

    run
}

fn left_run(node: &Node, map: &Vec<Vec<i32>>) -> Vec<Node> {
    let r = node.position.0;
    let c = node.position.1;
    assert!(c >= MIN_RUN);

    let mut accumulated_g = node.g + (1..MIN_RUN).map(|i| map[r][c - i]).sum::<i32>();
    let mut run: Vec<Node> = Vec::new();
    for nc in (c.saturating_sub(MAX_RUN)..=(c - MIN_RUN)).rev() {
        accumulated_g += map[r][nc];
        let left = Node {
            position: (r, nc, 0),
            f: 0, // Filled in later
            g: accumulated_g,
        };
        run.push(left);
    }
    run
}
