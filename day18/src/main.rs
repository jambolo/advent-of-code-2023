use common::load;
use regex::Regex;

#[derive(Debug)]
struct Step {
    #[allow(dead_code)]
    direction: char,
    #[allow(dead_code)]
    distance: i32,
    #[allow(dead_code)]
    color: u32,
}

fn main() {
    println!("Day 18, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    let lines = load::lines().unwrap();
    let steps: Vec<Step> = parse_steps(&lines);

    #[cfg(not(feature = "part2"))]
    part1(&steps);

    #[cfg(feature = "part2")]
    part2(&steps);
}

#[cfg(not(feature = "part2"))]
fn part1(steps: &[Step]) {
    let extents = compute_extents(steps);
//    println!("Extents: {:?}", extents);
    let width = (extents.0).1 - (extents.0).0 + 1;
    let height = (extents.1).1 - (extents.1).0 + 1;
    let start: (usize, usize) = ((-(extents.0).0).try_into().unwrap(), (-(extents.1).0).try_into().unwrap());
//    println!("Width: {}, Height: {}, Start: {:?}", width, height, start);

    let mut map = create_map(width as usize, height as usize, start, steps);

    let interior_point = find_interior_point(&map);
    flood_fill(&mut map, interior_point);

    let volume = compute_volume(&map);
    println!("Volume: {}", volume);
}

#[cfg(feature = "part2")]
fn part2(steps: &[Step]) {
    // The direction and distance is actually stored in the color value. The first five hexadecimal digits encode the distance in
    // meters as a five-digit hexadecimal number. The last hexadecimal digit encodes the direction to dig: 0 means R, 1 means D, 2
    // means L, and 3 means U. Build a new Step vector with the corrected values.

    let mut p: (i64, i64) = (0, 0);
    let vertices = steps[0..steps.len()]
        .into_iter()
        .map(|step| {
            let distance = ((step.color >> 4) & 0xFFFFF) as i64;
            let direction_code = (step.color & 0xF) as u8;
            let old_p = p;
            p = match direction_code {
                0 => (p.0 + distance, p.1), // Right
                1 => (p.0, p.1 - distance), // Down
                2 => (p.0 - distance, p.1), // Left
                3 => (p.0, p.1 + distance), // Up
                _ => panic!("Invalid direction code: {}", direction_code),
            };
            old_p
        })
        .collect::<Vec<(i64, i64)>>();

    let inner_area = area(&vertices);
    let perimeter_area = perimeter(&vertices) / 2;
    let total_area = inner_area + perimeter_area + 1;
    println!("Area: {}", total_area);
}

#[cfg(feature = "part2")]
fn area(vertices: &[(i64, i64)]) -> i64 {
    let n = vertices.len();
    let a: i64 = vertices.iter()
        .zip(vertices.iter().cycle().skip(1))
        .take(n) 
        .map(|(v0, v1)| (v0.0 * v1.1) - (v1.0 * v0.1))
        .sum();

    a.abs() / 2
}

#[cfg(feature = "part2")]
fn perimeter(vertices: &[(i64, i64)]) -> i64 {
    let n = vertices.len();
    let p: i64 = vertices.iter()
        .zip(vertices.iter().cycle().skip(1))
        .take(n) 
        .map(|(v0, v1)| (v1.0 - v0.0).abs() + (v1.1 - v0.1).abs())
        .sum();

    p
}

/// Parses the input into a vector of Steps
/// The format of the input is:
///     <dir> <length> '(#' <color> ')'
///     where <dir> is one of U, D, L, R
///     <length> is an integer
///     <color> is a hex color code
///
/// Example input:
/// U 2 (#ff0000)
/// R 4 (#0000ff)
/// D 3 (#00ff00)
/// ...
///
fn parse_steps(lines: &[String]) -> Vec<Step> {
    let steps_re = Regex::new(r"^([UDLR])\s+(\d+)\s+\(#([0-9a-fA-F]+)\)$").expect("Failed to compile regex");
    let mut steps: Vec<Step> = Vec::new();

    for line in lines {
        if let Some(captures) = steps_re.captures(line) {
            let direction_m = captures.get(1).expect("Failed to parse direction");
            let distance_m = captures.get(2).expect("Failed to parse distance");
            let color_m = captures.get(3).expect("Failed to parse color");
            let step = Step {
                direction: direction_m.as_str().chars().next().expect("Missing direction"),
                distance: distance_m.as_str().parse().expect("Invalid distance"),
                color: u32::from_str_radix(&color_m.as_str(), 16).expect("Invalid color"),
            };
            steps.push(step);
        } else {
            panic!("Failed to parse: '{}'", line);
        }
    }

    steps
}

#[cfg(not(feature = "part2"))]
fn compute_extents(steps: &[Step]) -> ((i32, i32), (i32, i32)) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    let mut x = 0;
    let mut y = 0;
    for step in steps {
        match step.direction {
            'U' => y -= step.distance,
            'D' => y += step.distance,
            'L' => x -= step.distance,
            'R' => x += step.distance,
            _ => panic!("Unknown direction: {}", step.direction),
        }
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    ((min_x, max_x), (min_y, max_y))
}

#[cfg(not(feature = "part2"))]
fn create_map(width: usize, height: usize, start: (usize, usize), steps: &[Step]) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = vec![vec!['.'; width]; height];

    let mut x = start.0;
    let mut y = start.1;
    map[y][x] = '#';
    for step in steps {
        for _ in 0..step.distance {
            match step.direction {
                'U' => y -= 1,
                'D' => y += 1,
                'L' => x -= 1,
                'R' => x += 1,
                _ => panic!("Unknown direction: {}", step.direction),
            }
            map[y][x] = '#';
        }
    }

    map
}

// fn print_map(map: &Vec<Vec<char>>) {
//     for row in map {
//         for cell in row {
//             print!("{}", cell);
//         }
//         println!();
//     }
//     println!();
// }

#[cfg(not(feature = "part2"))]
fn find_interior_point(map: &[Vec<char>]) -> (usize, usize) {
    // We are guaranteed to find an interior point on the second row because a horizontal boundary must exist on the
    // first row. We start at the left edge of the second row and move right until we find a wall. The next empty
    // space is an interior point if the space up and left is a wall. Otherwise, we are still outside.
    let y: usize = 1;
    let mut x: usize = 0;

    while x < map[y].len() {
        // Find the next wall
        while x < map[y].len() && map[y][x] != '#' {
            x += 1;
        }
        if x == map[y].len() {
            break;
        }
        // Find the next empty space
        while x < map[y].len() && map[y][x] == '#' {
            x += 1;
        }
        if x == map[y].len() {
            break;
        }
        // Check if the space up and left is a wall
        debug_assert!(x > 0);
        debug_assert!(y > 0);
        if map[y - 1][x - 1] == '#' {
            return (x, y);
        }
    }
    panic!("No interior point found");
}

#[cfg(not(feature = "part2"))]
fn flood_fill(map: &mut [Vec<char>], start: (usize, usize)) {
    let right_edge = map[0].len() - 1;
    let bottom_edge = map.len() - 1;

    let mut stack: Vec<(usize, usize)> = Vec::new();
    stack.push(start);

    while let Some((x, y)) = stack.pop() {
        if map[y][x] != '#' {
            map[y][x] = '#';
            if x > 0 {
                stack.push((x - 1, y));
            }
            if x < right_edge {
                stack.push((x + 1, y));
            }
            if y > 0 {
                stack.push((x, y - 1));
            }
            if y < bottom_edge {
                stack.push((x, y + 1));
            }
        }
    }
}

#[cfg(not(feature = "part2"))]
fn compute_volume(map: &[Vec<char>]) -> i32 {
    map.iter().flat_map(|row| row.iter()).filter(|&&cell| cell == '#').count() as i32
}
