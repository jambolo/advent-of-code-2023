use regex::Regex;
use common::load;

#[derive(Debug)]
struct Step {
    direction: char,
    distance: i32,
}

fn main() {
    println!("Day 18, part {}", if cfg!(feature="part2") { "2" } else { "1" });

    // Color is ignored
    let steps_re = Regex::new(r"^([UDLR])\s+(\d+)\s+\(#[0-9a-fA-F]+\)$").unwrap();

    let lines = load::lines().unwrap();
    let mut steps:Vec<Step> = Vec::new();
    for line in lines {
        if let Some(captures) = steps_re.captures(line.as_str()) {
            let step = Step {
                direction: captures.get(1).unwrap().as_str().chars().next().unwrap(),
                distance: captures.get(2).unwrap().as_str().parse().unwrap(),
            };
            steps.push(step);
        } else {
            panic!("No match for: {}", line);
        }
    }

    let extents = compute_extents(&steps);
    let width = (extents.0).1 - (extents.0).0 + 1;
    let height = (extents.1).1 - (extents.1).0 + 1;
    let start: (usize, usize) = ((-(extents.0).0).try_into().unwrap(), (-(extents.1).0).try_into().unwrap());

    let mut map = create_map(width as usize, height as usize, start, &steps);

    let interior_point = find_interior_point(&map);
    flood_fill(&mut map, interior_point);

    let volume = compute_volume(&map);
    println!("Volume: {}", volume);
}

fn compute_extents(steps: &Vec<Step>) -> ((i32, i32), (i32, i32)) {
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

fn create_map(width: usize, height: usize, start: (usize, usize), steps: &Vec<Step>) -> Vec<Vec<char>> {
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

fn find_interior_point(map: &Vec<Vec<char>>) -> (usize, usize) {
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
        if map[y-1][x-1] == '#' {
            return (x, y);
        }
    }
    panic!("No interior point found");
}

fn flood_fill(map: &mut Vec<Vec<char>>, start: (usize, usize)) {
    let right_edge = map[0].len() - 1;
    let bottom_edge = map.len() - 1;

    let mut stack: Vec<(usize, usize)> = Vec::new();
    stack.push(start);

    while let Some((x, y)) = stack.pop() {
        if map[y][x] != '#' {
            map[y][x] = '#';
            if x > 0 {
                stack.push((x-1, y));
            }
            if x < right_edge {
                stack.push((x+1, y));
            }
            if y > 0 {
                stack.push((x, y-1));
            }
            if y < bottom_edge {
                stack.push((x, y+1));
            }
        }
    }
}

fn compute_volume(map: &Vec<Vec<char>>) -> i32 {
    map.iter()
        .flat_map(|row| row.iter())
        .filter(|&&cell| cell == '#')
        .count() as i32
}