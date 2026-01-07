use common::load;

fn main() {
    println!("Day 10, part {}", if cfg!(feature="part2") { "2" } else { "1" });
    let lines = load::lines().unwrap();

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

//    draw_grid(&grid);

    let mut occupied: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

    // Mark the starting point as occupied
    let starting_point = find_start(&grid);
    let points = find_exits(starting_point, &grid);
    grid[starting_point.1][starting_point.0] = type_from_exits(&points);

    // Follow the path until the start is reached again, marking each point as occupied
    occupied[starting_point.1][starting_point.0] = true;
    let mut p = points[0].clone();
    while p.0.0 != starting_point.0 || p.0.1 != starting_point.1 {
        occupied[p.0.1][p.0.0] = true;
        p = next_point(p, &grid);
    }

    // Clear all unoccupied points
    clear_unoccupied_points(&mut grid, &occupied);

//    draw_grid(&grid);

    // For each unoccupied point, find the number of times a ray in the direction (-1, -1) crosses a pipe wall
    let mut number_of_inside_points = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if !occupied[y][x] {
                if count_crossings(x, y, &grid) % 2 == 1 {
//                    println!("inside: ({}, {})", x, y);
                    number_of_inside_points += 1;
                }
            }
        }
    }

//    draw_annotated_grid(&grid, &occupied);

    println!("number of inside points: {}", number_of_inside_points);
}

/* 
fn draw_grid(grid: &Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{}", grid[y][x]);
        }
        println!("");
    }
    println!("");
}

fn draw_annotated_grid(grid: &Vec<Vec<char>>, occupied: &Vec<Vec<bool>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if !occupied[y][x] {
                print!("{}", if count_crossings(x, y, &grid) % 2 == 1 { 'I' } else { 'O' });
            } else {
                print!("{}", grid[y][x]);
            }
        }
        println!("");
    }
    println!("");
}
*/

fn clear_unoccupied_points(grid: &mut Vec<Vec<char>>, occupied: &Vec<Vec<bool>>) {
    // Set all unoccupied points to '.'
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if !occupied[y][x] {
                grid[y][x] = '.';
            }
        }
    }
}

fn type_from_exits(exits: &Vec<((usize, usize), (isize, isize))>) -> char {
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

fn count_crossings(x0: usize, y0: usize, grid: &Vec<Vec<char>>) -> i32 {
    let mut number_of_crossings = 0;
    let mut x = x0;
    let mut y = y0;
    while x > 0 && y > 0 {
        x -= 1;
        y -= 1;
        let g = grid[y][x];
        if g == '|' || g == '-' || g == 'F' || g == 'J' {
            number_of_crossings += 1;
        }
    }
    number_of_crossings
}

fn next_point(p: ((usize, usize), (isize, isize)), grid: &Vec<Vec<char>>) -> ((usize, usize), (isize, isize)) {
    let d = direction(p.1, grid[p.0.1][p.0.0]);
    let n = advance(p.0, d);
    (n, d)
}

fn find_exits(point: (usize, usize), grid: &Vec<Vec<char>>) -> Vec<((usize, usize), (isize, isize))> {
    let mut exits: Vec<((usize, usize), (isize, isize))> = Vec::new();
    if point.1 > 0 {
        let d: (isize, isize) = (0, -1);
        let n: (usize, usize) = advance(point, d);
        let g = grid[n.1][n.0];
        if g == '|' || g == 'F' || g == '7' {
            exits.push((n, d));
        }
    }
    if point.0 < grid[point.1].len() - 1 {
        let d: (isize, isize) = (1, 0);
        let n: (usize, usize) = advance(point, d);
        let g = grid[n.1][n.0];
        if g == '-' || g == 'J' || g == '7' {
            exits.push((n, d));
        }
    }
    if point.1 < grid.len() - 1 {
        let d: (isize, isize) = (0, 1);
        let n: (usize, usize) = advance(point, d);
        let g = grid[n.1][n.0];
        if g == '|' || g == 'J' || g == 'L' {
            exits.push((n, d));
        }
    }
    if point.0 > 0 {
        let d: (isize, isize) = (-1, 0);
        let n: (usize, usize) = advance(point, d);
        let g = grid[n.1][n.0];
        if g == '-' || g == 'L' || g == 'F' {
            exits.push((n, d));
        }
    }
    exits
}

fn advance(p: (usize, usize), d: (isize, isize)) -> (usize, usize) {
    let n:(usize, usize) = ((p.0 as isize + d.0) as usize, (p.1 as isize + d.1) as usize);
    n
}

// Returns the direction specified by the character.
fn direction(d: (isize, isize), c: char) -> (isize, isize) {
    match c {
        '-' => if d.0 == 1 { (1, 0) } else { (-1, 0) },
        '|' => if d.1 == 1 { (0, 1) } else { (0, -1) },
        'F' => if d.1 == -1 { (1, 0) } else { (0, 1) },
        '7' => if d.0 == 1 { (0, 1) } else { (-1, 0) },
        'J' => if d.0 == 1 { (0, -1) } else { (-1, 0) },
        'L' => if d.1 == 1 { (1, 0) } else { (0, -1) },
        _ => (0, 0)
    }
}

// Returns the starting point of the grid
fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'S' {
                return (x, y);
            }
        }
    }
    panic!("No starting point found");
}
