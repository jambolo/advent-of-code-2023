use common::load;

type Point = (usize, usize);
type Direction = (isize, isize);

fn main() {
    println!("=== Day 10, part {} ===", if cfg!(feature = "part2") { "2" } else { "1" });
    let lines = load::lines().unwrap();

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    let starting_point = find_start(&grid);
    let points = find_exits(starting_point, &grid);
    grid[starting_point.1][starting_point.0] = type_from_exits(&points);

    let result = if cfg!(feature = "part2") {
        part2(&mut grid, starting_point, &points)
    } else {
        part1(&grid, starting_point, &points)
    };

    println!("Result: {}", result);
}

fn part1(grid: &[Vec<char>], starting_point: Point, points: &[(Point, Direction)]) -> i32 {
    let mut steps = 0;
    let mut p = points[0];
    while p.0 != starting_point {
        steps += 1;
        p = next_point(p, grid);
    }
    (steps + 1) / 2
}

fn part2(grid: &mut [Vec<char>], starting_point: Point, points: &[(Point, Direction)]) -> i32 {
    let mut occupied: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

    occupied[starting_point.1][starting_point.0] = true;
    let mut p = points[0];
    while p.0 != starting_point {
        occupied[p.0 .1][p.0 .0] = true;
        p = next_point(p, grid);
    }

    clear_unoccupied_points(grid, &occupied);

    let number_of_inside_points = occupied
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row
            .iter()
            .enumerate()
            .map(move |(x, &occupied_here)| (x, y, occupied_here)))
        .filter(|(x, y, occupied_here)| !occupied_here && count_crossings(*x, *y, grid) % 2 == 1)
        .count();

    number_of_inside_points as i32
}

fn clear_unoccupied_points(grid: &mut [Vec<char>], occupied: &[Vec<bool>]) {
    // Set all unoccupied points to '.'
    (0..occupied.len()).flat_map(|y| (0..occupied[y].len()).map(move |x| (x, y)))
        .filter(|(x, y)| !occupied[*y][*x])
        .for_each(|(x, y)| grid[y][x] = '.');
}

fn type_from_exits(exits: &[(Point, Direction)]) -> char {
    let d0 = exits[0].1;
    let d1 = exits[1].1;
    if d0.1 == -1 && d1.0 == 1 || d1.1 == -1 && d0.0 == 1 {
        return 'L';
    }
    if d0.0 == 0 && d1.0 == 0 {
        return '|';
    }
    if d0.1 == -1 && d1.0 == -1 || d1.1 == -1 && d0.0 == -1 {
        return 'J';
    }
    if d0.1 == 1 && d1.0 == 1 || d1.1 == 1 && d0.0 == 1 {
        return 'F';
    }
    if d0.1 == 0 && d1.1 == 0 {
        return '-';
    }
    if d0.0 == -1 && d1.1 == 1 || d1.0 == -1 && d0.1 == 1 {
        return '7';
    }
    panic!("Unknown type");
}

fn count_crossings(x0: usize, y0: usize, grid: &[Vec<char>]) -> i32 {
    (1..=x0.min(y0))
        .filter(|i| matches!(grid[y0 - i][x0 - i], '|' | '-' | 'F' | 'J'))
        .count() as i32
}

fn next_point(p: (Point, Direction), grid: &[Vec<char>]) -> (Point, Direction) {
    let d = direction(p.1, grid[p.0 .1][p.0 .0]);
    let n = advance(p.0, d);
    (n, d)
}

fn find_exits(point: Point, grid: &[Vec<char>]) -> Vec<(Point, Direction)> {
    let mut exits: Vec<(Point, Direction)> = Vec::new();
    if point.1 > 0 {
        let d: Direction = (0, -1);
        let n: Point = advance(point, d);
        let g = grid[n.1][n.0];
        if g == '|' || g == 'F' || g == '7' {
            exits.push((n, d));
        }
    }
    if point.0 < grid[point.1].len() - 1 {
        let d: Direction = (1, 0);
        let n: Point = advance(point, d);
        let g = grid[n.1][n.0];
        if g == '-' || g == 'J' || g == '7' {
            exits.push((n, d));
        }
    }
    if point.1 < grid.len() - 1 {
        let d: Direction = (0, 1);
        let n: Point = advance(point, d);
        let g = grid[n.1][n.0];
        if g == '|' || g == 'J' || g == 'L' {
            exits.push((n, d));
        }
    }
    if point.0 > 0 {
        let d: Direction = (-1, 0);
        let n: Point = advance(point, d);
        let g = grid[n.1][n.0];
        if g == '-' || g == 'L' || g == 'F' {
            exits.push((n, d));
        }
    }
    exits
}

fn advance(p: Point, d: Direction) -> Point {
    let n: Point = ((p.0 as isize + d.0) as usize, (p.1 as isize + d.1) as usize);
    n
}

// Returns the direction specified by the character.
fn direction(d: Direction, c: char) -> Direction {
    match c {
        '-' => {
            if d.0 == 1 {
                (1, 0)
            } else {
                (-1, 0)
            }
        }
        '|' => {
            if d.1 == 1 {
                (0, 1)
            } else {
                (0, -1)
            }
        }
        'F' => {
            if d.1 == -1 {
                (1, 0)
            } else {
                (0, 1)
            }
        }
        '7' => {
            if d.0 == 1 {
                (0, 1)
            } else {
                (-1, 0)
            }
        }
        'J' => {
            if d.0 == 1 {
                (0, -1)
            } else {
                (-1, 0)
            }
        }
        'L' => {
            if d.1 == 1 {
                (1, 0)
            } else {
                (0, -1)
            }
        }
        _ => (0, 0),
    }
}

// Returns the starting point of the grid
fn find_start(grid: &[Vec<char>]) -> Point {
    grid.iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|&c| c == 'S')
                .map(|x| (x, y))
        })
        .expect("No starting point found")
}
