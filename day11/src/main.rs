use common::load;

const EXPANSION: i64 = if cfg!(feature = "part2") { 1000000 - 1 } else { 1 };

fn main() {
    println!("=== Day 11, part {} ===", if cfg!(feature = "part2") { "2" } else { "1" });
    let galaxy = load::lines().unwrap();

    let (xr, xc) = expand(&galaxy);
    let stars = find_stars(&galaxy);
    let distances = find_distances(&stars, &xr, &xc);

    println!("Result: {}", distances.iter().sum::<i64>());
}

fn find_distances(stars: &[(usize, usize)], xr: &[usize], xc: &[usize]) -> Vec<i64> {
    (0..stars.len() - 1)
        .flat_map(|i| (i + 1..stars.len())
            .map(move |j| distance(&stars[i], &stars[j], xr, xc)))
        .collect()
}

fn distance(star1: &(usize, usize), star2: &(usize, usize), xr: &[usize], xc: &[usize]) -> i64 {
    let min_r = star1.0.min(star2.0);
    let max_r = star1.0.max(star2.0);
    let min_c = star1.1.min(star2.1);
    let max_c = star1.1.max(star2.1);
    let row_expansion = number_of_expansions_between(xr, min_r, max_r) * EXPANSION;
    let dr = max_r as i64 - min_r as i64 + row_expansion;
    let column_expansion = number_of_expansions_between(xc, min_c, max_c) * EXPANSION;
    let dc = (star1.1 as i64 - star2.1 as i64).abs() + column_expansion;

    dr + dc
}

fn number_of_expansions_between(vec: &[usize], a: usize, b: usize) -> i64 {
    let start = match vec.binary_search(&a) {
        Ok(pos) | Err(pos) => pos,
    };
    let end = match vec.binary_search(&b) {
        Ok(pos) | Err(pos) => pos,
    };
    (end - start) as i64
}

fn find_stars(galaxy: &[String]) -> Vec<(usize, usize)> {
    (0..galaxy.len())
        .flat_map(|i| (0..galaxy[i].len())
            .filter(move |j| galaxy[i].chars().nth(*j).unwrap() == '#')
            .map(move |j| (i, j)))
        .collect()
}

// Expands the galaxy
fn expand(galaxy: &[String]) -> (Vec<usize>, Vec<usize>) {
    let xr = expand_vertically(galaxy);
    let xc = expand_horizontally(galaxy);
    (xr, xc)
}

// Expands the galaxy horizontally
fn expand_horizontally(galaxy: &[String]) -> Vec<usize> {
    (0..galaxy[0].len()).filter(|i| column_is_empty(galaxy, *i)).collect()
}

// Returns true if the column is empty
fn column_is_empty(galaxy: &[String], column: usize) -> bool {
    galaxy.iter().all(|line| line.chars().nth(column).unwrap() == '.')
}

fn expand_vertically(galaxy: &[String]) -> Vec<usize> {
    (0..galaxy.len()).filter(|i| row_is_empty(&galaxy[*i])).collect()
}

// Returns true if the row is empty
fn row_is_empty(row: &str) -> bool {
    row.chars().all(|c| c == '.')
}
