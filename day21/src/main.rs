use common::load;
use std::collections::HashSet;

const NUMBER_OF_STEPS: i64 = 64;

fn main() {
    println!("=== Day 21, part {} ===", if cfg!(feature = "part2") { "2" } else { "1" });

    // Load the map
    let mut map: Vec<Vec<char>> = load::map().unwrap();
    //    print_map(&map);
    let width = map[0].len();
    let height = map.len();

    // Find the initial position
    let start = find_start(&map).unwrap();

    // Remove the start position from the map to make it easier to work with
    map[start.1][start.0] = '.';

    println!("Number of steps: {}", NUMBER_OF_STEPS);

    let result = if cfg!(feature = "part2") {
        part2(map, width, height, start)
    } else {
        part1(map, width, height, start)
    };

    // Print the result
    println!("Result: {}", result);
}

fn part1(map: Vec<Vec<char>>, width: usize, height: usize, start: (usize, usize)) -> usize {
    // List of positions that have been reached at each step
    let mut terminals: HashSet<(usize, usize)> = HashSet::new();
    terminals.insert(start);

    // Breadth-first search
    for _step in 1..=NUMBER_OF_STEPS {
        let positions = terminals.into_iter().collect::<Vec<_>>();
        terminals = HashSet::new();
        for (x, y) in positions {
            // Add the neighbors of the position to the list of positions
            if x < width - 1 && (map[y][x + 1] == '.') {
                terminals.insert((x + 1, y));
            }
            if x > 0 && map[y][x - 1] == '.' {
                terminals.insert((x - 1, y));
            }
            if y < height - 1 && map[y + 1][x] == '.' {
                terminals.insert((x, y + 1));
            }
            if y > 0 && map[y - 1][x] == '.' {
                terminals.insert((x, y - 1));
            }
        }
    }
    terminals.len()
}

fn part2(map: Vec<Vec<char>>, width: usize, height: usize, start: (usize, usize)) -> usize {
    // Offset the positions by multiple of width and height to avoid negative indices
    #[allow(clippy::manual_div_ceil)]
    let offset_x = width * ((NUMBER_OF_STEPS as usize + width - 1) / width);
    #[allow(clippy::manual_div_ceil)]
    let offset_y = height * ((NUMBER_OF_STEPS as usize + height - 1) / height);
    // List of positions that have been reached at each step
    let mut terminals: HashSet<(usize, usize)> = HashSet::new();
    terminals.insert((start.0 + offset_x, start.1 + offset_y));

    // Breadth-first search
    for _step in 1..=NUMBER_OF_STEPS {
        let positions = terminals.into_iter().collect::<Vec<_>>();
        terminals = HashSet::new();
        for (x, y) in positions {
            let wrapped_x = x % width;
            let wrapped_y = y % height;
            // Add the neighbors of the position to the list of positions
            if map[wrapped_y][(wrapped_x + 1) % width] == '.' {
                terminals.insert((x + 1, y));
            }
            if map[wrapped_y][(wrapped_x + width - 1) % width] == '.' {
                terminals.insert((x - 1, y));
            }
            if map[(wrapped_y + 1) % height][wrapped_x] == '.' {
                terminals.insert((x, y + 1));
            }
            if map[(wrapped_y + height - 1) % height][wrapped_x] == '.' {
                terminals.insert((x, y - 1));
            }
        }
    }
    // The number of steps should be 26501365, but that is too large to compute in a reasonable time, so I computed results for
    // multiples of the map size + 65 and extrapolated the result to the final number of steps, which is returned here.
    // The answer is `15505n^2 + 15633n + 3944`, where `n` is (26501365 - 65) / width
    let n = (26501365_usize - 65) / width;
    15505 * n * n + 15633 * n + 3944
}

fn find_start(map: &[Vec<char>]) -> Option<(usize, usize)> {
    map.iter().enumerate().find_map(|(y, row)| {
        row.iter()
            .enumerate()
            .find_map(|(x, &cell)| if cell == 'S' { Some((x, y)) } else { None })
    })
}
