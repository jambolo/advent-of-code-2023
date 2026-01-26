use common::load;

// Example data
//const BOUNDS: ((f64, f64), (f64, f64)) = ((7.0, 27.0), (7.0, 27.0));

// Real data
const BOUNDS: ((f64, f64), (f64, f64)) = ((200000000000000.0, 400000000000000.0), (200000000000000.0, 400000000000000.0));

#[derive(Debug)]
struct Stone {
    position: (f64, f64, f64),
    velocity: (f64, f64, f64),
}

fn main() {
    println!("Day 24, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    // Load the data
    let lines = load::lines().unwrap();
    let stones = parse_stones(&lines);

    // Find all intersections
    let intersection_count = stones
        .iter()
        .enumerate()
        .flat_map(|(i, stone_i)| {
            stones
                .iter()
                .skip(i + 1)
                .filter_map(move |stone_j| intersects_xy(stone_i, stone_j))
        })
        .count();
    println!("Intersection count: {}", intersection_count);
}

fn parse_stones(lines: &Vec<String>) -> Vec<Stone> {
    let mut stones = Vec::new();
    for line in lines {
        let pv: Vec<&str> = line.split("@").collect();
        let position: Vec<f64> = pv[0].split(",").map(|s| s.trim().parse().unwrap()).collect();
        let velocity: Vec<f64> = pv[1].split(",").map(|s| s.trim().parse().unwrap()).collect();
        stones.push(Stone {
            position: (position[0], position[1], position[2]),
            velocity: (velocity[0], velocity[1], velocity[2]),
        });
    }
    stones
}

//fn collides_xy(s1: &Stone, s2: &Stone) -> Option<(f64, (f64, f64))> {
//
//    let p1_x = s1.position.0;
//    let p1_y = s1.position.1;
//    let p2_x = s2.position.0;
//    let p2_y = s2.position.1;
//    let v1_x = s1.velocity.0;
//    let v1_y = s1.velocity.1;
//    let v2_x = s2.velocity.0;
//    let v2_y = s2.velocity.1;
//
//    // Check for parallel movement
//    if (v1_x * v2_x + v1_y * v2_y).abs() < std::f64::EPSILON {
//        return None;
//    }
//
//    let dv_x = v1_x - v2_x;
//    let dv_y = v1_y - v2_y;
//    if dv_x.abs() > dv_y.abs()
//    {
//        let t = (p2_x - p1_x) / dv_x;
//        let y = p1_y + t * v1_y;
//        let x = p1_x + t * v1_x;
//        if t > 0.0 && y >= BOUNDS.1.0 && y <= BOUNDS.1.1 && x >= BOUNDS.0.0 && x <= BOUNDS.0.1 {
//            Some((t, (x, y)))
//        } else {
//            None
//        }
//    }
//    let t = if dv_x.abs() > dv_y.abs() { (p2_x - p1_x) / dv_x } else { (p2_y - p1_y) / dv_y };
//    let y1 = p1_y + t * v1_y;
//    let x1 = p1_x + t * v1_x;
//    if t > 0.0 && y >= BOUNDS.1.0 && y <= BOUNDS.1.1 && x >= BOUNDS.0.0 && x <= BOUNDS.0.1 {
//        Some((t, (x, y)))
//    } else {
//        None
//    }
//}

fn intersects_xy(s1: &Stone, s2: &Stone) -> Option<(f64, f64)> {
    let p1 = (s1.position.0, s1.position.1);
    let p2 = (s2.position.0, s2.position.1);
    let v1 = normalize_xy(s1.velocity.0, s1.velocity.1);
    let v2 = normalize_xy(s2.velocity.0, s2.velocity.1);

    // Check for parallel rays
    let det = v2.0 * v1.1 - v1.0 * v2.1;
    if det.abs() < std::f32::EPSILON as f64 {
        return None;
    }

    let t1 = ((p1.0 - p2.0) * v2.1 - (p1.1 - p2.1) * v2.0) / det;
    let t2 = ((p1.0 - p2.0) * v1.1 - (p1.1 - p2.1) * v1.0) / det;

    let x = p1.0 + t1 * v1.0;
    let y = p1.1 + t1 * v1.1;
    if t1 > 0.0 && t2 > 0.0 && y >= BOUNDS.1 .0 && y <= BOUNDS.1 .1 && x >= BOUNDS.0 .0 && x <= BOUNDS.0 .1 {
        Some((x, y))
    } else {
        None
    }
}

fn normalize_xy(x: f64, y: f64) -> (f64, f64) {
    let l = (x * x + y * y).sqrt();
    (x / l, y / l)
}
