use common::load;
use itertools::Itertools;
use nalgebra::Vector3;

// Example data
//const BOUNDS: ((f64, f64), (f64, f64)) = ((7.0, 27.0), (7.0, 27.0));

// Real data
const BOUNDS: ((f64, f64), (f64, f64)) = ((200000000000000.0, 400000000000000.0), (200000000000000.0, 400000000000000.0));

#[derive(Debug)]
struct Stone {
    position: (f64, f64, f64),
    velocity: (f64, f64, f64),
}

#[derive(Debug)]
struct StoneI64 {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
}

fn main() {
    println!("=== Day 24, part {} ===", if cfg!(feature = "part2") { "2" } else { "1" });

    let lines = load::lines().unwrap();
    let stones = parse_stones(&lines);
    let stones_i64 = parse_stones_i64(&lines);

    let result = if cfg!(feature = "part2") { part2(&stones_i64) } else { part1(&stones) };
    println!("Result: {}", result);
}

fn part1(stones: &[Stone]) -> usize {
    stones
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| intersects_xy(a, b).is_some())
        .count()
}

fn part2(stones: &[StoneI64]) -> usize {
    // Build the f64 data from the i64 data, offsetting the positions by the minimum position to reduce the size of the numbers
    // Stones 2, 3, an 4 give the best results.
    let min_x = stones[2..=4].iter().map(|s| s.position.0).min().unwrap();
    let min_y = stones[2..=4].iter().map(|s| s.position.1).min().unwrap();
    let min_z = stones[2..=4].iter().map(|s| s.position.2).min().unwrap();
    let stones: Vec<Stone> = stones[2..=4].iter().map(|s| {
        Stone {
            position: (
                (s.position.0 - min_x) as f64,
                (s.position.1 - min_y) as f64,
                (s.position.2 - min_z) as f64,
            ),
            velocity: (
                s.velocity.0 as f64,
                s.velocity.1 as f64,
                s.velocity.2 as f64,
            ),
        }
    }).collect();

    let (x, y, z) = solve(&stones[0], &stones[1], &stones[2]);
    (x.round() as i64 + min_x + y.round() as i64 + min_y + z.round() as i64 + min_z) as usize
}

fn solve(stone0: &Stone, stone1: &Stone, stone2: &Stone) -> (f64, f64, f64) {
    let v0 = Vector3::new(stone0.velocity.0, stone0.velocity.1, stone0.velocity.2);
    let v1 = Vector3::new(stone1.velocity.0, stone1.velocity.1, stone1.velocity.2);
    let v2 = Vector3::new(stone2.velocity.0, stone2.velocity.1, stone2.velocity.2);

    let p0 = Vector3::new(stone0.position.0, stone0.position.1, stone0.position.2);
    let p1 = Vector3::new(stone1.position.0, stone1.position.1, stone1.position.2);
    let p2 = Vector3::new(stone2.position.0, stone2.position.1, stone2.position.2);

    let v10 = v1 - v0;
    let v20 = v2 - v0;

    let p10 = p1 - p0;
    let p20 = p2 - p0;

    let v10x = v10.cross_matrix();
    let v20x = v20.cross_matrix();
    let p10x = p10.cross_matrix();
    let p20x = p20.cross_matrix();

    // Construct a 6x6 matrix and a 6x1 vector for the linear system
    let mut a = nalgebra::Matrix6::zeros();
    a.fixed_view_mut::<3, 3>(0, 0).copy_from(&v10x);
    a.fixed_view_mut::<3, 3>(0, 3).copy_from(&p10x);
    a.fixed_view_mut::<3, 3>(3, 0).copy_from(&v20x);
    a.fixed_view_mut::<3, 3>(3, 3).copy_from(&p20x);

    let vp0 = v0.cross(&p0);
    let vp1 = v1.cross(&p1);
    let vp2 = v2.cross(&p2);

    let vp10 = vp1 - vp0;
    let vp20 = vp2 - vp0;

    let mut b = nalgebra::Vector6::zeros();
    b.fixed_view_mut::<3, 1>(0, 0).copy_from(&vp10);
    b.fixed_view_mut::<3, 1>(3, 0).copy_from(&vp20);

    // Solve the linear system a * x = b
//    let x = a.lu().solve(&b).unwrap();
    let x = a.full_piv_lu().solve(&b).unwrap();

    // Return the position
    (x[0], x[1], x[2])
}

fn parse_stones(lines: &[String]) -> Vec<Stone> {
    lines.iter().map(|line| {
        let pv: Vec<&str> = line.split("@").collect();
        let position: (f64, f64, f64) = pv[0].split(",")
            .map(|s| s.trim().parse().unwrap())
            .collect_tuple::<(_,_,_)>().unwrap();
        let velocity: (f64, f64, f64) = pv[1].split(",")
            .map(|s| s.trim().parse().unwrap())
            .collect_tuple::<(_,_,_)>().unwrap();
        Stone {
            position,
            velocity,
        }
    }).collect()
}

fn parse_stones_i64(lines: &[String]) -> Vec<StoneI64> {
    lines.iter().map(|line| {
        let pv: Vec<&str> = line.split("@").collect();
        let position: (i64, i64, i64) = pv[0].split(",")
            .map(|s| s.trim().parse().unwrap())
            .collect_tuple::<(_,_,_)>().unwrap();
        let velocity: (i64, i64, i64) = pv[1].split(",")
            .map(|s| s.trim().parse().unwrap())
            .collect_tuple::<(_,_,_)>().unwrap();
        StoneI64 {
            position,
            velocity,
        }
    }).collect()
}

fn intersects_xy(s1: &Stone, s2: &Stone) -> Option<(f64, f64)> {
    let p1 = (s1.position.0, s1.position.1);
    let p2 = (s2.position.0, s2.position.1);
    let v1 = normalize_xy(s1.velocity.0, s1.velocity.1);
    let v2 = normalize_xy(s2.velocity.0, s2.velocity.1);

    let det = v2.0 * v1.1 - v1.0 * v2.1;

    // Check for parallel rays
    if det.abs() < f32::EPSILON as f64 {
        return None;
    }

    let t1 = ((p1.0 - p2.0) * v2.1 - (p1.1 - p2.1) * v2.0) / det;
    let t2 = ((p1.0 - p2.0) * v1.1 - (p1.1 - p2.1) * v1.0) / det;

    let x1 = p1.0 + t1 * v1.0;
    let y1 = p1.1 + t1 * v1.1;
    if t1 > 0.0 && t2 > 0.0 && y1 >= BOUNDS.1 .0 && y1 <= BOUNDS.1 .1 && x1 >= BOUNDS.0 .0 && x1 <= BOUNDS.0 .1 {
        Some((x1, y1))
    } else {
        None
    }
}

fn normalize_xy(x: f64, y: f64) -> (f64, f64) {
    let length = (x * x + y * y).sqrt();
    (x / length, y / length)
}
