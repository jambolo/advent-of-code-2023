use common::load;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}
#[derive(Debug, Clone, Copy)]
struct Branch {
    direction: Direction,
    x: usize,
    y: usize,
}

fn main() {
    println!("Day 16, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    let map = load::map().unwrap();

    #[cfg(not(feature = "part2"))]
    {
        let start = Branch {
            direction: Direction::Right,
            x: 0,
            y: 0,
        };
        let energize = energize(map, start);
        println!("Energized cells: {}", energize);
    }
    #[cfg(feature = "part2")]
    {
        let mut max: i32 = 0;
        for y in 0..map.len() {
            let start = Branch {
                direction: Direction::Right,
                x: 0,
                y,
            };
            let energize = energize(map.clone(), start);
            if energize > max {
                max = energize;
            }
        }

        for x in 0..map[0].len() {
            let start = Branch {
                direction: Direction::Up,
                x,
                y: map.len() - 1,
            };
            let energize = energize(map.clone(), start);
            if energize > max {
                max = energize;
            }
        }

        for y in 0..map.len() {
            let start = Branch {
                direction: Direction::Left,
                x: map[0].len() - 1,
                y,
            };
            let energize = energize(map.clone(), start);
            if energize > max {
                max = energize;
            }
        }

        for x in 0..map[0].len() {
            let start = Branch {
                direction: Direction::Down,
                x,
                y: 0,
            };
            let energize = energize(map.clone(), start);
            if energize > max {
                max = energize;
            }
        }
        println!("Max energized cells: {}", max);
    }
}

fn energize(mut map: Vec<Vec<char>>, start: Branch) -> i32 {
    let mut energized: Vec<Vec<i32>> = vec![vec![0; map[0].len()]; map.len()];
    let mut branches: Vec<Branch> = Vec::new();
    branches.push(start);

    // Move according to each branch in the stack until we have none left. Movement in a direction is done until we
    // hit something or we reach the edge of the map. If we hit something, we stop moving in that direction and push
    // 0, 1, or 2 new branches onto the stack depending on why we stopped.
    while let Some(branch) = branches.pop() {
        let x = branch.x;
        let y = branch.y;
        let direction = branch.direction;

        match direction {
            Direction::Right => move_right(&mut map, x, y, &mut branches, &mut energized),
            Direction::Up => move_up(&mut map, x, y, &mut branches, &mut energized),
            Direction::Left => move_left(&mut map, x, y, &mut branches, &mut energized),
            Direction::Down => move_down(&mut map, x, y, &mut branches, &mut energized),
        }
    }
    // Return the number of energized cells
    energized.iter().flatten().sum()
}

fn move_right(map: &mut Vec<Vec<char>>, mut x: usize, y: usize, branches: &mut Vec<Branch>, energized: &mut Vec<Vec<i32>>) {
    let right_edge = map[0].len() - 1;
    let bottom_edge = map.len() - 1;

    // Move right until we hit something or the edge
    loop {
        energized[y][x] = 1;
        if map[y][x] != '.' && map[y][x] != '-' || x >= right_edge {
            break;
        }
        x += 1;
    }

    // If we hit something, we need to branch unless we are at the edge
    match map[y][x] {
        '/' => branch_up(x, y, branches),
        '\\' => branch_down(x, y, bottom_edge, branches),
        '|' => {
            map[y][x] = 'X'; // Prevent cycles
            branch_up(x, y, branches);
            branch_down(x, y, bottom_edge, branches);
        }
        'X' => {} // Cycle detected, so we don't continue in any direction
        '.' => {} // Going off the edge, so we don't continue in any direction
        '-' => {} // Going off the edge, so we don't continue in any direction

        _ => panic!("Unexpected character: {}", map[y][x]),
    }
}

fn move_up(map: &mut Vec<Vec<char>>, x: usize, mut y: usize, branches: &mut Vec<Branch>, energized: &mut Vec<Vec<i32>>) {
    let right_edge = map[0].len() - 1;
    let _bottom_edge = map.len() - 1;

    // Move up until we hit something or the edge
    loop {
        energized[y][x] = 1;
        if map[y][x] != '.' && map[y][x] != '|' || y == 0 {
            break;
        }
        y -= 1;
    }

    // If we hit something, we need to branch
    match map[y][x] {
        '/' => branch_right(x, y, right_edge, branches),
        '\\' => branch_left(x, y, branches),
        '-' => {
            map[y][x] = 'X'; // Prevent cycles
            branch_right(x, y, right_edge, branches);
            branch_left(x, y, branches);
        }
        'X' => {} // Cycle detected, so we don't continue in any direction
        '.' => {} // Going off the edge, so we don't continue in any direction
        '|' => {} // Going off the edge, so we don't continue in any direction

        _ => panic!("Unexpected character: {}", map[y][x]),
    }
}

fn move_left(map: &mut Vec<Vec<char>>, mut x: usize, y: usize, branches: &mut Vec<Branch>, energized: &mut Vec<Vec<i32>>) {
    let _right_edge = map[0].len() - 1;
    let bottom_edge = map.len() - 1;

    // Move left until we hit something or the edge
    loop {
        energized[y][x] = 1;
        if map[y][x] != '.' && map[y][x] != '-' || x == 0 {
            break;
        }
        x -= 1;
    }

    // If we hit something, we need to branch
    match map[y][x] {
        '/' => branch_down(x, y, bottom_edge, branches),
        '\\' => branch_up(x, y, branches),
        '|' => {
            map[y][x] = 'X'; // Prevent cycles
            branch_down(x, y, bottom_edge, branches);
            branch_up(x, y, branches);
        }
        'X' => {} // Cycle detected, so we don't continue in any direction
        '.' => {} // Going off the edge, so we don't continue in any direction
        '-' => {} // Going off the edge, so we don't continue in any direction

        _ => panic!("Unexpected character: {}", map[y][x]),
    }
}

fn move_down(map: &mut Vec<Vec<char>>, x: usize, mut y: usize, branches: &mut Vec<Branch>, energized: &mut Vec<Vec<i32>>) {
    let right_edge = map[0].len() - 1;
    let bottom_edge = map.len() - 1;

    // Move down until we hit something or the edge
    loop {
        energized[y][x] = 1;
        if map[y][x] != '.' && map[y][x] != '|' || y >= bottom_edge {
            break;
        }
        y += 1;
    }

    // If we hit something, we need to branch
    match map[y][x] {
        '/' => branch_left(x, y, branches),
        '\\' => branch_right(x, y, right_edge, branches),
        '-' => {
            map[y][x] = 'X'; // Prevent cycles
            branch_left(x, y, branches);
            branch_right(x, y, right_edge, branches);
        }
        'X' => {} // Cycle detected, so we don't continue in any direction
        '.' => {} // Going off the edge, so we don't continue in any direction
        '|' => {} // Going off the edge, so we don't continue in any direction

        _ => panic!("Unexpected character: {}", map[y][x]),
    }
}

fn branch_up(x: usize, y: usize, branches: &mut Vec<Branch>) {
    if y > 0 {
        branches.push(Branch {
            direction: Direction::Up,
            x,
            y: y - 1,
        })
    }
}

fn branch_down(x: usize, y: usize, edge: usize, branches: &mut Vec<Branch>) {
    if y < edge {
        branches.push(Branch {
            direction: Direction::Down,
            x,
            y: y + 1,
        })
    }
}

fn branch_left(x: usize, y: usize, branches: &mut Vec<Branch>) {
    if x > 0 {
        branches.push(Branch {
            direction: Direction::Left,
            x: x - 1,
            y,
        })
    }
}

fn branch_right(x: usize, y: usize, edge: usize, branches: &mut Vec<Branch>) {
    if x < edge {
        branches.push(Branch {
            direction: Direction::Right,
            x: x + 1,
            y,
        })
    }
}

fn _print_map(map: &Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}
