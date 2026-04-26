use common::load;

fn main() {
    println!("=== Day 14, part {} ===", if cfg!(feature = "part2") { "2" } else { "1" });
    let lines = load::lines().unwrap();
    let mut map: Vec<Vec<char>> = vec![];
    for line in lines {
        map.push(line.chars().collect());
    }
    //    print_map(&map);

    if cfg!(feature = "part2") {
        // For part 2, the load repeats every 360 cycles
        const CYCLES: i64 = 1_000_000_000 % 360;

        for _ in 0..CYCLES {
            tip_north(&mut map);
            tip_west(&mut map);
            tip_south(&mut map);
            tip_east(&mut map);
        }
    } else {
        tip_north(&mut map);
    }

    println!("Result: {}", compute_load(&map));
}

fn tip_north(map: &mut [Vec<char>]) {
    let rows = map.len();
    let cols = map[0].len();

    for (i, j) in (1..rows).flat_map(|i| (0..cols).map(move |j| (i, j))) {
        if map[i][j] == 'O' {
            roll_north(map, i, j);
        }
    }
}

fn roll_north(map: &mut [Vec<char>], i: usize, j: usize) {
    let mut k = i;
    while k > 0 && map[k - 1][j] == '.' {
        k -= 1;
    }
    if k != i {
        map[k][j] = 'O';
        map[i][j] = '.';
    }
}

fn tip_west(map: &mut [Vec<char>]) {
    let rows = map.len();
    let cols = map[0].len();

    for (i, j) in (1..cols).flat_map(|j| (0..rows).map(move |i| (i, j))) {
        if map[i][j] == 'O' {
            roll_west(map, i, j);
        }
    }
}

fn roll_west(map: &mut [Vec<char>], i: usize, j: usize) {
    let mut k = j;
    while k > 0 && map[i][k - 1] == '.' {
        k -= 1;
    }
    if k != j {
        map[i][k] = 'O';
        map[i][j] = '.';
    }
}

fn tip_south(map: &mut [Vec<char>]) {
    let rows = map.len();
    let cols = map[0].len();

    for (i, j) in (0..rows - 1).rev().flat_map(|i| (0..cols).map(move |j| (i, j))) {
        if map[i][j] == 'O' {
            roll_south(map, i, j);
        }
    }
}

fn roll_south(map: &mut [Vec<char>], i: usize, j: usize) {
    let mut k = i;
    while k < map.len() - 1 && map[k + 1][j] == '.' {
        k += 1;
    }
    if k != i {
        map[k][j] = 'O';
        map[i][j] = '.';
    }
}

fn tip_east(map: &mut [Vec<char>]) {
    let rows = map.len();
    let cols = map[0].len();

    for (i, j) in (0..cols - 1).rev().flat_map(|j| (0..rows).map(move |i| (i, j))) {
        if map[i][j] == 'O' {
            roll_east(map, i, j);
        }
    }
}

fn roll_east(map: &mut [Vec<char>], i: usize, j: usize) {
    let mut k = j;
    while k < map[0].len() - 1 && map[i][k + 1] == '.' {
        k += 1;
    }
    if k != j {
        map[i][k] = 'O';
        map[i][j] = '.';
    }
}

fn compute_load(map: &[Vec<char>]) -> i64 {
    let mut sum = 0;
    for i in 0..map.len() {
        sum += row_load(map, i);
    }
    sum
}
fn row_load(map: &[Vec<char>], row: usize) -> i64 {
    let mut sum: i64 = 0;
    for c in map[row].iter() {
        if *c == 'O' {
            sum += 1;
        }
    }
    sum * (map.len() - row) as i64
}
