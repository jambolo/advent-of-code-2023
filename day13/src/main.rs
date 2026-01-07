use common::load;

fn main() {
    println!("Day 13, part {}", if cfg!(feature="part2") { "2" } else { "1" });
    let lines = load::lines().unwrap();
    let maps = load_maps(&lines);
    let mut sum: i32 = 0;
    for m in maps {
        if let Some(col) = find_vertical_mirror(&m) {
            sum += col;
        } else if let Some(row) = find_horizontal_mirror(&m) {
            sum += 100 * row;
        } else {
            println!("Error: no split found");
            draw_map(&m);
        }
    }
    println!("Sum: {}", sum);
}

fn load_maps(lines: &[String]) -> Vec<Vec<Vec<char>>> {
    let mut maps: Vec<Vec<Vec<char>>> = vec![];
    let mut map:Vec<Vec<char>> = vec![];
    for line in lines {
        if line.is_empty() {
            if !map.is_empty() {
                maps.push(map.clone());
                map = vec![];
            }
        } else {
            map.push(line.chars().collect());
        }
    }
    if !map.is_empty() {
        maps.push(map);
    }
    maps
}

fn draw_map(m: &Vec<Vec<char>>) {
    for row in m {
        println!("{}", row.iter().collect::<String>());
    }
}
fn find_vertical_mirror(m: &Vec<Vec<char>>) -> Option<i32> {
    let num_columns = m[0].len();
    for i in 1..num_columns {
        let span = std::cmp::min(i, num_columns - i);
        let mut mirrored = true;
        if cfg!(feature="part2") {
            let mut smudged = false;
            for j in 1..=span {
                let same;
                (same, smudged) = smudged_columns_eq(m, i - j, i + j - 1, smudged);
                if !same {
                    mirrored = false;
                    break;
                }
            }
            if !smudged {
                mirrored = false;
            }
        } else {
            for j in 1..=span {
                if !columns_eq(m, i - j, i + j - 1) {
                    mirrored = false;
                    break;
                }
            }
        }
        if mirrored {
            return Some(i as i32);
        }
    }
    None
}

fn columns_eq(m: &Vec<Vec<char>>, j0: usize, j1: usize) -> bool {
    m.iter().all(|row| row[j0] == row[j1])
}

fn smudged_columns_eq(m: &Vec<Vec<char>>, j0: usize, j1: usize, smudged: bool) -> (bool, bool) {
    let same = columns_eq(m, j0, j1);
    if !same && !smudged && columns_differ_by_one(m, j0, j1) {
        (true, true)
    } else {
        (same, smudged)
    }
}

fn columns_differ_by_one(m: &Vec<Vec<char>>, j0: usize, j1: usize) -> bool {
    let mut differ_by_one = false;

    for i in 0..m.len() {
        if m[i][j0] != m[i][j1] {
            if differ_by_one {
                return false; // If we already found one difference, return false
            }
            differ_by_one = true;
        }
    }

    differ_by_one
}

fn find_horizontal_mirror(m: &Vec<Vec<char>>) -> Option<i32> {
    for i in 1..m.len() {
        let span = std::cmp::min(i, m.len() - i);
        let mut mirrored = true;
        if cfg!(feature="part2") {
            let mut smudged = false;
            for j in 1..=span {
                let same;
                (same, smudged) = smudged_rows_eq(m, i - j, i + j - 1, smudged);
                if !same {
                    mirrored = false;
                    break;
                }
            }
            if !smudged {
                mirrored = false;
            }
        } else {
            for j in 1..=span {
                if !rows_eq(m, i - j, i + j - 1) {
                    mirrored = false;
                    break;
                }
            }
        }
        if mirrored {
            return Some(i as i32);
        }
    }
    None
}

fn rows_eq(m: &Vec<Vec<char>>, i0: usize, i1: usize) -> bool {
    m[i0] == m[i1]
}

fn smudged_rows_eq(m: &Vec<Vec<char>>, i0: usize, i1: usize, smudged: bool) -> (bool, bool) {
    let same = rows_eq(m, i0, i1);
    if !same && !smudged && rows_differ_by_one(m, i0, i1) {
        (true, true)
    } else {
        (same, smudged)
    }
}

fn rows_differ_by_one(m: &Vec<Vec<char>>, i0: usize, i1: usize) -> bool {
    let mut differ_by_one = false;

    for j in 0..m[0].len() {
        if m[i0][j] != m[i1][j] {
            if differ_by_one {
                return false;   // If we already found one difference, return false
            }
            differ_by_one = true;
        }
    }

    differ_by_one
}