use common::load;
use std::collections::HashSet;

const NUMBER_OF_STEPS: i64 = 64;

fn main() {
    println!("Day 21, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    // Load the map
    let map = load::map().unwrap();
    //    print_map(&map);
    let width = map[0].len();
    let height = map.len();

    // Find the initial position
    let start = find_start(&map).unwrap();

    // List of terminals
    let mut terminals: HashSet<(usize, usize)> = HashSet::new();
    terminals.insert(start); // Start position is automatically a terminal

    // List of non-terminals
    let mut neighbors = Vec::new();
    neighbors.push(start);

    // Breadth-first search
    for step in 1..=NUMBER_OF_STEPS {
        // Get the next list of neighbors to check and clear the neighbors list
        let mut next = Vec::new();
        (next, neighbors) = (neighbors, next);

        // Check each cell and add the neighbors
        for (x, y) in next.iter() {
            if step % 2 != 0 {
                // Odd steps: nothing special, just add the neighbours
                if *x < width - 1 && map[*y][*x + 1] == '.' {
                    neighbors.push((*x + 1, *y));
                }
                if *x > 0 && map[*y][*x - 1] == '.' {
                    neighbors.push((*x - 1, *y));
                }
                if *y < height - 1 && map[*y + 1][*x] == '.' {
                    neighbors.push((*x, *y + 1));
                }
                if *y > 0 && map[*y - 1][*x] == '.' {
                    neighbors.push((*x, *y - 1));
                }
            } else {
                // Even steps: add the non-terminal neighbours
                if *x < width - 1 && map[*y][*x + 1] == '.' && !terminals.contains(&(*x + 1, *y)) {
                    neighbors.push((*x + 1, *y));
                    terminals.insert((*x + 1, *y));
                }
                if *x > 0 && map[*y][*x - 1] == '.' && !terminals.contains(&(*x - 1, *y)) {
                    neighbors.push((*x - 1, *y));
                    terminals.insert((*x - 1, *y));
                }
                if *y < height - 1 && map[*y + 1][*x] == '.' && !terminals.contains(&(*x, *y + 1)) {
                    neighbors.push((*x, *y + 1));
                    terminals.insert((*x, *y + 1));
                }
                if *y > 0 && map[*y - 1][*x] == '.' && !terminals.contains(&(*x, *y - 1)) {
                    neighbors.push((*x, *y - 1));
                    terminals.insert((*x, *y - 1));
                }
            }
        }
        //        print_map_with_terminals(&map, &terminals);
    }

    // Print the result
    println!("Number of terminals: {}", terminals.len());
}

fn find_start(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    map.iter().enumerate().find_map(|(y, row)| {
        row.iter()
            .enumerate()
            .find_map(|(x, &cell)| if cell == 'S' { Some((x, y)) } else { None })
    })
}
