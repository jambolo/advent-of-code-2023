use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use common::load;

#[derive(Debug, Clone, Copy)]
struct Node {
    position: (usize, usize),
    f: f32,     // f = g + h
    g: f32,     // cost from start
    from: Option<(usize, usize)>,
    count: i32, // number of times in the same direction
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f.partial_cmp(&other.f).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn main() {
    println!("Day 17, part {}", if cfg!(feature="part2") { "2" } else { "1" });

    let map = load::numbers_map().unwrap();
    let start: (usize, usize) = (0, 0);
    let goal: (usize, usize) = (map[0].len() - 1, map.len() - 1);

    fn h(start: (usize, usize), goal: (usize, usize)) -> f32 {
        (start.0.abs_diff(goal.0) + start.1.abs_diff(goal.1)) as f32
    }

    let d = shortest(start, goal, h, &map);
    println!("Shortest path: {}", d);
}

fn shortest(start: (usize, usize),
                         goal: (usize, usize),
                         h: fn(from: (usize, usize), to: (usize, usize)) -> f32,
                         map: &Vec<Vec<i32>>) -> f32
{
    let mut open: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
    
    open.push(Reverse(Node { position: start, f: 0.0 + h(start, goal), g: 0.0, from: None, count: 0, }));

    let mut lowest_h = std::f32::INFINITY;
    while let Some(Reverse(current)) = open.pop() {
        if current.position == goal {
            return current.g;
        }
        if current.f - current.g < lowest_h {
            lowest_h = current.f - current.g;
        }

        let neighbors: Vec<Node> = get_neighbors(&current, &map);
        for mut n in neighbors {
            // Set the neighbor's g and f values
            n.g = current.g + map[n.position.1][n.position.0] as f32;
            n.f = n.g + h(n.position, goal);
            // If the neighbor is already in the open set, but its g value is lower, then we replace the existing node 
            if let Some(existing) = find_node(&open, n.position, n.from, n.count) {
                if n.g < existing.g {
                    open.retain(|&Reverse(c)| c.position != n.position || c.from != n.from || c.count != n.count);
                    open.push(Reverse(n));
                }
            } else {
                // If the neighbor is not in the open set, we add it
                open.push(Reverse(n));
            }
        }
    }
    std::f32::INFINITY
}

fn find_node(open: &BinaryHeap<Reverse<Node>>, position: (usize, usize), from: Option<(usize, usize)>, count: i32 ) -> Option<Node> {
    open
        .iter()
        .find(|&&Reverse(node)| node.position == position && node.from == from && node.count == count)
        .map(|&Reverse(node)| node)
}

fn get_neighbors(current: &Node, map: &Vec<Vec<i32>>) -> Vec<Node> {
    let mut neighbors = Vec::new();
    let x = current.position.0;
    let y = current.position.1;
    let from = current.from;
    let count = current.count;
    let width = map[0].len();
    let height = map.len();

    // Push left only if not on the left edge, and coming from nowhere, above, below, or right with a count < 3
    if x > 0 {
        if let Some((from_x, _)) = from {
            if from_x == x || x < from_x && count < 3 {
                let left = Node {
                    position: (x - 1, y),
                    f: 0.0, // Filled in later
                    g: 0.0, // Filled in later
                    from: Some(current.position),
                    count: if x < from_x { count + 1 } else { 1 }, // Increment count if moving left again
                };
                neighbors.push(left);
            }
        } else {
            // If `from` is `None`, we can push left without additional checks
            let left = Node {
                position: (x - 1, y),
                f: 0.0, // Filled in later
                g: 0.0, // Filled in later
                from: Some(current.position),
                count: 1,
            };
            neighbors.push(left);
        }
    }
    // Push right only if not on the right edge, and coming from nowhere, above, below, or left with a count < 3
    if x < width - 1 {
        if let Some((from_x, _)) = from {
            if from_x == x || from_x < x && count < 3 {
                let right = Node {
                    position: (x + 1, y),
                    f: 0.0, // Filled in later
                    g: 0.0, // Filled in later
                    from: Some(current.position),
                    count: if from_x < x { count + 1 } else { 1 }, // Increment count if moving right again
                };
                neighbors.push(right);
            }
        } else {
            let right = Node {
                position: (x + 1, y),
                f: 0.0, // Filled in later
                g: 0.0, // Filled in later
                from: Some(current.position),
                count: 1,
            };
            neighbors.push(right);
        }
    }
    // Push up only if not on the top edge, and coming from nowhere, left, right, or below with a count < 3
    if y > 0 {
        if let Some((_, from_y)) = from {
            if from_y == y || y < from_y && count < 3 {
                let up = Node {
                    position: (x, y - 1),
                    f: 0.0, // Filled in later
                    g: 0.0, // Filled in later
                    from: Some(current.position),
                    count: if y < from_y { count + 1 } else { 1 }, // Increment count if moving up again
                };
                neighbors.push(up);
            }
        } else {
            let up = Node {
                position: (x, y - 1),
                f: 0.0, // Filled in later
                g: 0.0, // Filled in later
                from: Some(current.position),
                count: 1,
            };
            neighbors.push(up);
        }
    }
    // Push down only if not on the bottom edge, and coming from nowhere, left, right, or above with a count < 3
    if y < height - 1 {
        if let Some((_, from_y)) = from {
            if from_y == y || from_y < y && count < 3 {
                let down = Node {
                    position: (x, y + 1),
                    f: 0.0, // Filled in later
                    g: 0.0, // Filled in later
                    from: Some(current.position),
                    count: if from_y < y { count + 1 } else { 1 }, // Increment count if moving down again
                };
                neighbors.push(down);
            }
        } else {
            let down = Node {
                position: (x, y + 1),
                f: 0.0, // Filled in later
                g: 0.0, // Filled in later
                from: Some(current.position),
                count: 1,
            };
            neighbors.push(down);
        }
    }
    neighbors
}
