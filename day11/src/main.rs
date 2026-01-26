use common::load;

const EXPANSION: i64 = 1000000 - 1;

fn main() {
    println!("Day 11, part {}", if cfg!(feature = "part2") { "2" } else { "1" });
    let galaxy = load::lines().unwrap();

    let (xr, xc) = expand(&galaxy);
    let stars = find_stars(&galaxy);
    let distances = find_distances(&stars, &xr, &xc);

    println!("Sum of distances: {}", distances.iter().sum::<i64>());
}

fn find_distances(stars: &Vec<(usize, usize)>, xr: &Vec<usize>, xc: &Vec<usize>) -> Vec<i64> {
    let mut distances: Vec<i64> = Vec::new();
    for i in 0..stars.len() - 1 {
        for j in i + 1..stars.len() {
            distances.push(distance(&stars[i], &stars[j], xr, xc));
        }
    }
    distances
}

fn distance(star1: &(usize, usize), star2: &(usize, usize), xr: &Vec<usize>, xc: &Vec<usize>) -> i64 {
    let min_r = star1.0.min(star2.0);
    let max_r = star1.0.max(star2.0);
    let min_c = star1.1.min(star2.1);
    let max_c = star1.1.max(star2.1);
    let row_expansion = number_of_expansions_between(xr, min_r, max_r) * EXPANSION;
    let dr = max_r as i64 - min_r as i64 + row_expansion;
    let column_expansion = number_of_expansions_between(xc, min_c, max_c) * EXPANSION;
    let dc = (star1.1 as i64 - star2.1 as i64).abs() + column_expansion;
    (dr + dc) as i64
}

fn number_of_expansions_between(vec: &Vec<usize>, a: usize, b: usize) -> i64 {
    let start = match vec.binary_search(&a) {
        Ok(pos) | Err(pos) => pos,
    };
    let end = match vec.binary_search(&b) {
        Ok(pos) | Err(pos) => pos,
    };
    (end - start) as i64
}

fn find_stars(galaxy: &Vec<String>) -> Vec<(usize, usize)> {
    let mut stars: Vec<(usize, usize)> = Vec::new();
    for i in 0..galaxy.len() {
        for j in 0..galaxy[i].len() {
            if galaxy[i].chars().nth(j).unwrap() == '#' {
                stars.push((i, j));
            }
        }
    }
    stars
}

// Expands the galaxy
fn expand(galaxy: &Vec<String>) -> (Vec<usize>, Vec<usize>) {
    let xr = expand_vertically(galaxy);
    let xc = expand_horizontally(galaxy);
    (xr, xc)
}

// Expands the galaxy horizontally
fn expand_horizontally(galaxy: &Vec<String>) -> Vec<usize> {
    let mut xc: Vec<usize> = Vec::new();
    for i in 0..galaxy[0].len() {
        if column_is_empty(galaxy, i) {
            xc.push(i);
        }
    }
    xc
}

// Returns true if the column is empty
fn column_is_empty(galaxy: &Vec<String>, column: usize) -> bool {
    for line in galaxy {
        if line.chars().nth(column).unwrap() != '.' {
            return false;
        }
    }
    true
}

fn expand_vertically(galaxy: &Vec<String>) -> Vec<usize> {
    let mut xr: Vec<usize> = Vec::new();
    for i in 0..galaxy.len() {
        if row_is_empty(&galaxy[i]) {
            xr.push(i);
        }
    }
    xr
}

// Returns true if the row is empty
fn row_is_empty(row: &String) -> bool {
    for c in row.chars() {
        if c != '.' {
            return false;
        }
    }
    true
}
