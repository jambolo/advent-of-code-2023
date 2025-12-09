use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::time::Instant;
use common::load;

const MIN_RUN: usize = if cfg!(feature="part2") { 4 } else { 1 };
const MAX_RUN: usize = if cfg!(feature="part2") { 10 } else { 3 };

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right, Up, Left, Down
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Right => Left,
            Up => Down,
            Left => Right,
            Down => Up,
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct Node {
    position: (usize, usize),
    f: i32,     // f = g + h
    g: i32,     // cost from start
    dir: Option<Direction>,
}

impl Eq for Node {

}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}

fn manhattan_distance(start: (usize, usize), goal: (usize, usize)) -> i32 {
    let dx = (goal.0 as i32 - start.0 as i32).abs();
    let dy = (goal.1 as i32 - start.1 as i32).abs();
    dx + dy
}


fn main() {
    println!("Day 17, part {}", if cfg!(feature="part2") { "2" } else { "1" });
    let timer = Instant::now();

    let map = load::numbers_map().unwrap();
    let start: (usize, usize) = (0, 0);
    let goal: (usize, usize) = (map[0].len() - 1, map.len() - 1);

//    let shortest_unrestricted_distance_map = build_shortest_unrestricted_distance_map(start, goal, &map);
////    print_map(&shortest_unrestricted_distance_map);
//
//    let h = |start: (usize, usize), _: (usize, usize)| {
//        shortest_unrestricted_distance_map[start.1][start.0]
//    };

    let h = |start: (usize, usize), goal: (usize, usize) | -> i32 {
        manhattan_distance(start, goal)
    };

    let d = shortest_path(start, goal, h, &map);
    let elapsed = timer.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Shortest path: {}", d);
}

#[cfg(debug_assertions)]
fn print_map(map: &Vec<Vec<i32>>) {
    for row in map {
        for cell in row {
            if *cell == std::i32::MAX {
                print!("  ");
            } else {
                print!("{:02} ", cell);
            }
        }
        println!();
    }
}
fn build_shortest_unrestricted_distance_map(
    start: (usize, usize),
    goal: (usize, usize),
    map: &Vec<Vec<i32>>
) -> Vec<Vec<i32>> {
    struct Node {
        position: (usize, usize),
        cost: i32,
    }
    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(other.cost.cmp(some.cost))
        }
    }
    
    fn get_neighbors(current: &(usize, usize), map: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
        let mut neighbors: Vec<(usize, usize)> = Vec::new();
        if current.0 > 0 {
            neighbors.push((current.0 - 1, current.1));
        }
        if current.0 < map[0].len() - 1 {
            neighbors.push((current.0 + 1, current.1));
        }
        if current.1 > 0 {
            neighbors.push((current.0, current.1 - 1));
        }
        if current.1 < map.len() - 1 {
            neighbors.push((current.0, current.1 + 1));
        }
        neighbors
    }

    let mut shortest_unrestricted_distance_map = vec![vec![std::i32::MAX; map[0].len()]; map.len()];
    shortest_unrestricted_distance_map[goal.1][goal.0] = map[goal.1][goal.0];

//    let mut open: BinaryHeap<Node> = BinaryHeap::new();
//    open.push(Node { (goal.0, goal.1), map[goal.1][goal.0] });
    let open = Vec::new();
    open.push(goal);
    while let Some(current) = open.pop() {
        let current_cost = shortest_unrestricted_distance_map[current.1][current.0];
        let neighbors: Vec<(usize, usize)> = get_neighbors(&current, &map);
        for n in &neighbors {
            let neighbor_cost = current_cost + map[n.1][n.0];
            if neighbor_cost < shortest_unrestricted_distance_map[n.1][n.0] {
                shortest_unrestricted_distance_map[n.1][n.0] = neighbor_cost;
                open.push(Reverse(*n));
            }
        }
    }
    shortest_unrestricted_distance_map[start.0][start.1] -= map[start.1][start.0];
    shortest_unrestricted_distance_map
}

fn shortest_path(
    start: (usize, usize),
                         goal: (usize, usize),
    h: impl Fn((usize, usize), (usize, usize)) -> i32,
    map: &Vec<Vec<i32>>) -> i32
{
    let mut open: BinaryHeap<Node> = BinaryHeap::new();
    
    // Push the start node to the open set
    open.push(Node { position: start, f: 0 + h(start, goal), g: 0, dir: None });

    let mut lowest_h = std::i32::MAX;
    while let Some(Reverse(current)) = open.pop() {
        if current.position == goal {
            return current.g;
        }
        if current.f - current.g < lowest_h {
            lowest_h = current.f - current.g;
            println!("Lowest h: {} ({})", lowest_h, manhattan_distance(current.position, goal) );
        }

        let neighbors: Vec<Node> = get_neighbors(&current, &map);
        for mut n in neighbors {
            n.f = n.g + h(n.position, goal);
            // If the neighbor is already in the open set, but its new g value is lower, then we replace the existing node 
            if let Some(existing) = find_node(&open, n.position, n.dir) {
                if n.g < existing.g {
                    open.retain(|&Reverse(c)|
                        c.position != n.position || ((c.dir.unwrap().0 == 0) != (n.dir.unwrap().0 == 0))
                    );
                    open.push(Reverse(n));
                }
            } else {
                // If the neighbor is not in the open set, we add it
                open.push(Reverse(n));
            }
        }
    }
    std::i32::MAX
}

fn find_node(open: &BinaryHeap<Reverse<Node>>, position: (usize, usize), dir:Option<(i32, i32)>) -> Option<Node> {
    open
        .iter()
        .find(|&&Reverse(node)| node.position == position && ((node.dir.unwrap().0 == 0) == (dir.unwrap().0 == 0)))
        .map(|&Reverse(node)| node)
}

fn get_neighbors(current: &Node, map: &Vec<Vec<i32>>) -> Vec<Node> {
    let x = current.position.0;
    let y = current.position.1;
    let width = map[0].len();
    let height = map.len();

    let mut neighbors = Vec::new();

    // Push left movements only if not on the left edge, and coming from nowhere, above, or below.
    if x >= MIN_RUN {
        if let Some((_, dir_y)) = current.dir {
            if dir_y != 0 {
                neighbors.extend(left_run(current, &map));
            }
        } else {
            // If `from` is `None`, we can push left without additional checks
            neighbors.extend(left_run(current, &map));
        }
    }
    // Push right only if not on the right edge, and coming from nowhere, above, or below
    if x + MIN_RUN < width {
        if let Some((_, dir_y)) = current.dir {
            if dir_y != 0 {
                neighbors.extend(right_run(current, map));
            }
        } else {
            neighbors.extend(right_run(current, map));
        }
    }
    // Push up only if not on the top edge, and coming from nowhere, left, or right
    if y >= MIN_RUN {
        if let Some((dir_x, _)) = current.dir {
            if dir_x != 0 {
                neighbors.extend(up_run(current, map));
            }
        } else {
            neighbors.extend(up_run(current, map));
        }
    }
    // Push down only if not on the bottom edge, and coming from nowhere, left, or right
    if y + MIN_RUN < height {
        if let Some((dir_x, _)) = current.dir {
            if dir_x != 0 {
                neighbors.extend(down_run(current, map));
            }
        } else {
            neighbors.extend(down_run(current, map));
        }
        }
    neighbors
    }

fn down_run(current: &Node, map: &Vec<Vec<i32>>) -> Vec<Node> {
    let x = current.position.0;
    let y = current.position.1;
    let height = map.len();
    assert!(y + MIN_RUN < height);

    let mut accumulated_g = current.g + (1..MIN_RUN).map(|i| map[y + i][x]).sum::<i32>();
    let mut run: Vec<Node> = Vec::new();
    for ny in (y + MIN_RUN)..=std::cmp::min(y + MAX_RUN, height - 1) {
        accumulated_g += map[ny][x];
        let down = Node {
            position: (x, ny),
            f: 0, // Filled in later
            g: accumulated_g,
            dir: Some((0, 1)),
                };
        run.push(down);
    }
    run
            }
fn up_run(current: &Node, map: &Vec<Vec<i32>>) -> Vec<Node> {
    let x = current.position.0;
    let y = current.position.1;
    assert!(y >= MIN_RUN);

    let mut accumulated_g = current.g + (1..MIN_RUN).map(|i| map[y - i][x]).sum::<i32>();
    let mut run: Vec<Node> = Vec::new();
    for ny in (y.saturating_sub(MAX_RUN)..=(y - MIN_RUN)).rev() {
        accumulated_g += map[ny][x];
            let up = Node {
            position: (x, ny),
            f: 0, // Filled in later
            g: accumulated_g,
            dir: Some((0, -1)),
            };
        run.push(up);
        }
    run
    }

fn right_run(current: &Node, map: &Vec<Vec<i32>>) -> Vec<Node> {
    let x = current.position.0;
    let y = current.position.1;
    let width = map[0].len();
    assert!(x + MIN_RUN < width);

    let mut accumulated_g = current.g + (1..MIN_RUN).map(|i| map[y][x + i]).sum::<i32>();
    let mut run: Vec<Node> = Vec::new();
    for nx in (x + MIN_RUN)..=std::cmp::min(x + MAX_RUN, width - 1) {
        accumulated_g += map[y][nx];
        let right = Node {
            position: (nx, y),
            f: 0, // Filled in later
            g: accumulated_g,
            dir: Some((1, 0)),
                };
        run.push(right);
    }
    run
            }

fn left_run(current: &Node, map: &Vec<Vec<i32>>) -> Vec<Node> {
    let x = current.position.0;
    let y = current.position.1;
    assert!(x >= MIN_RUN);

    let mut accumulated_g = current.g + (1..MIN_RUN).map(|i| map[y][x - i]).sum::<i32>();
    let mut run: Vec<Node> = Vec::new();
    for nx in (x.saturating_sub(MAX_RUN)..=(x - MIN_RUN)).rev() {
        accumulated_g += map[y][nx];
        let left = Node {
            position: (nx, y),
            f: 0, // Filled in later
            g: accumulated_g,
            dir: Some((-1, 0)),
            };
        run.push(left);
    }
    run
}
