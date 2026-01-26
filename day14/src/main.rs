use common::load;

const CYCLES: i64 = 1000000;

fn main() {
    println!("Day 14, part {}", if cfg!(feature = "part2") { "2" } else { "1" });
    let lines = load::lines().unwrap();
    let mut map: Vec<Vec<char>> = vec![];
    for line in lines {
        map.push(line.chars().collect());
    }
    //    print_map(&map);

    if cfg!(feature = "part2") {
        let mut previous = map.clone();
        for i in 0..CYCLES {
            tip_north(&mut map);
            tip_west(&mut map);
            tip_south(&mut map);
            tip_east(&mut map);
            if i % (CYCLES / 10) == 0 {
                println!("Cycles: {:.0}%", (i as f64 / CYCLES as f64 * 100.0).round());
            }
            if previous == map {
                println!("Stable after {} cycles", i);
                break;
            }
            previous = map.clone();
        }
    } else {
        tip_north(&mut map);
    }

    println!("Load: {}", map_load(&map));
}

fn tip_north(map: &mut Vec<Vec<char>>) {
    let rows = map.len();
    let cols = map[0].len();

    for i in 1..rows {
        for j in 0..cols {
            if map[i][j] == 'O' {
                let mut k = i;
                while k > 0 && map[k - 1][j] == '.' {
                    k -= 1;
                }
                if map[k][j] == '.' {
                    map[k][j] = 'O';
                    map[i][j] = '.';
                }
            }
        }
    }
}

fn tip_west(map: &mut Vec<Vec<char>>) {
    let rows = map.len();
    let cols = map[0].len();

    for j in 1..cols {
        for i in 0..rows {
            if map[i][j] == 'O' {
                let mut k = j;
                while k > 0 && map[i][k - 1] == '.' {
                    k -= 1;
                }
                if map[i][k] == '.' {
                    map[i][k] = 'O';
                    map[i][j] = '.';
                }
            }
        }
    }
}

fn tip_south(map: &mut Vec<Vec<char>>) {
    let rows = map.len();
    let cols = map[0].len();

    for i in (0..rows - 1).rev() {
        for j in 0..cols {
            if map[i][j] == 'O' {
                let mut k = i;
                while k < rows - 1 && map[k + 1][j] == '.' {
                    k += 1;
                }
                if map[k][j] == '.' {
                    map[k][j] = 'O';
                    map[i][j] = '.';
                }
            }
        }
    }
}

fn tip_east(map: &mut Vec<Vec<char>>) {
    let rows = map.len();
    let cols = map[0].len();

    for j in (0..cols - 1).rev() {
        for i in 0..rows {
            if map[i][j] == 'O' {
                let mut k = j;
                while k < cols - 1 && map[i][k + 1] == '.' {
                    k += 1;
                }
                if map[i][k] == '.' {
                    map[i][k] = 'O';
                    map[i][j] = '.';
                }
            }
        }
    }
}

fn map_load(map: &Vec<Vec<char>>) -> i64 {
    let mut sum = 0;
    for i in 0..map.len() {
        sum += row_load(map, i);
    }
    sum
}
fn row_load(map: &Vec<Vec<char>>, row: usize) -> i64 {
    let mut sum: i64 = 0;
    for c in map[row].iter() {
        if *c == 'O' {
            sum += 1;
        }
    }
    sum * (map.len() - row) as i64
}
