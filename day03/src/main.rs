use common::load;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn main() {
    println!("Day 3, part {}", if cfg!(feature="part2") { "2" } else { "1" });
    let lines = load::lines().unwrap();

    // Create a 2D array of characters
    let mut grid: Vec<Vec<char>> = Vec::new();

    for s in lines {
        grid.push(s.chars().collect());
    }

    let mut sum = 0;
    let mut gears: BTreeMap<(usize, usize), Vec<u32>> = BTreeMap::new();

    // Scan the grid for part numbers
    for y in 0 .. grid.len() {
        let mut x = 0;
        while x < grid[y].len() {
            if grid[y][x].is_ascii_digit() {
                let (new_x, value, is_part_number, adjacent_gears) = scan_number(&grid, x, y);
                if is_part_number {
                    sum += value;
                    for g in adjacent_gears {
                        gears.entry(g).or_insert(Vec::new()).push(value);
                    }
                }
                x = new_x;
            }
            x = x + 1;
        }
    }

    #[cfg(not(feature="part2"))]
    println!("Sum: {}", sum);

    #[cfg(feature="part2")]
    {
        let mut gear_ratio_sum: u64 = 0;
        for g in gears {
            if g.1.len() == 2 {
                gear_ratio_sum += g.1[0] as u64 * g.1[1] as u64;
            }
        }

        println!("Gear ratio sum: {}", gear_ratio_sum);
    }
}

// Returns true if the character is a symbol
fn is_symbol(ch: char) -> bool {
    ch != '.' && !ch.is_ascii_digit()
}

// Returns true if the character is a gear
fn is_gear(ch: char) -> bool {
    ch == '*'
}

// Returns true if the character adjacent to a symbol
fn is_adjacent_to_symbol(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x > 0 {
        if is_symbol(grid[y][x-1]) {
            return true;
        }
        if y > 0 && is_symbol(grid[y-1][x-1]) {
            return true;
        }
        if y < grid.len() - 1 && is_symbol(grid[y+1][x-1]) {
            return true;
        }
    }

    if y > 0 && is_symbol(grid[y-1][x]) {
        return true;
    }
    if y < grid.len() - 1 && is_symbol(grid[y+1][x]) {
        return true;
    }

    if x < grid[y].len() - 1 {
        if is_symbol(grid[y][x+1]) {
            return true;
        }
        if y > 0 && is_symbol(grid[y-1][x+1]) {
            return true;
        }
        if y < grid.len() - 1 && is_symbol(grid[y+1][x+1]) {
            return true;
        }
    }

    false
}

// Returns true if the character adjacent to a symbol
fn find_adjacent_gears(grid: &Vec<Vec<char>>, x: usize, y: usize, gears: &mut BTreeSet<(usize, usize)>) {
    if x > 0 {
        if is_gear(grid[y][x-1]) {
            gears.insert((x-1, y));
        }
        if y > 0 && is_gear(grid[y-1][x-1]) {
            gears.insert((x-1, y-1));
        }
        if y < grid.len() - 1 && is_gear(grid[y+1][x-1]) {
            gears.insert((x-1, y+1));
        }
    }

    if y > 0 && is_gear(grid[y-1][x]) {
        gears.insert((x, y-1));
    }
    if y < grid.len() - 1 && is_gear(grid[y+1][x]) {
        gears.insert((x, y+1));
    }

    if x < grid[y].len() - 1 {
        if is_gear(grid[y][x+1]) {
            gears.insert((x+1, y));
        }
        if y > 0 && is_gear(grid[y-1][x+1]) {
            gears.insert((x+1, y-1));
        }
        if y < grid.len() - 1 && is_gear(grid[y+1][x+1]) {
            gears.insert((x+1, y+1));
        }
    }
}

fn scan_number(grid: &Vec<Vec<char>>, x0: usize, y0: usize) -> (usize, u32, bool, BTreeSet<(usize, usize)>) {
    let mut xn = x0;
    let mut value = 0;
    let mut is_part_number = false;
    let mut adjacent_gears: BTreeSet<(usize, usize)> = BTreeSet::new();
    while xn < grid[y0].len() && grid[y0][xn].is_ascii_digit() {
        value = value * 10 + grid[y0][xn].to_digit(10).unwrap();
        if is_adjacent_to_symbol(grid, xn, y0) {
            is_part_number = true;
            find_adjacent_gears(grid, xn, y0, &mut adjacent_gears);
        }
        xn = xn + 1;
    }
    (xn, value, is_part_number, adjacent_gears)
}
