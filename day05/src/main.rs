use common::load;

fn main() {
    println!("Day 5, part {}", if cfg!(feature = "part2") { "2" } else { "1" });
    let lines = load::lines().unwrap();
    let mut iter = lines.iter();

    // Load the seeds
    let seeds = parse_seeds(&mut iter);

    // Load each map
    let mut maps: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    while let Some(_line) = iter.next() {
        // Ignore name of map
        maps.push(parse_map(&mut iter));
    }

    let mut map = create_map_from_seeds(&seeds);
    for m in &maps {
        map = combine(&map, m);
    }

    println!("Min location: {}", map[0].0);
}

fn parse_seeds<'a, I>(iter: &mut I) -> Vec<(i64, i64)>
where
    I: Iterator<Item = &'a String>,
{
    let input = iter.next().unwrap();
    let parts: Vec<&str> = input.split(":").collect();
    let numbers: Vec<i64> = parts[1].trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let seeds = numbers.chunks(2).map(|c| (c[0], c[1])).collect();

    iter.next(); // Skip blank line

    seeds
}

fn parse_map<'a, I>(iter: &mut I) -> Vec<(i64, i64, i64)>
where
    I: Iterator<Item = &'a String>,
{
    let mut map = Vec::new();

    while let Some(line) = iter.next() {
        if line.trim().is_empty() {
            break;
        }

        let parts: Vec<i64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();

        map.push((parts[0], parts[1], parts[2]));
    }

    map.sort_unstable_by(|a, b| a.1.cmp(&b.1));
    map
}

fn create_map_from_seeds(seeds: &Vec<(i64, i64)>) -> Vec<(i64, i64, i64)> {
    let mut map = Vec::new();

    for s in seeds {
        map.push((s.0, s.0, s.1));
    }

    map.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    map
}

// Combine two maps, ignoring source ranges in the second map that are outside of the destination ranges in the first map
fn combine(map1: &Vec<(i64, i64, i64)>, map2: &Vec<(i64, i64, i64)>) -> Vec<(i64, i64, i64)> {
    let mut new_map: Vec<(i64, i64, i64)> = Vec::new();

    for e1 in map1 {
        let mut e = e1.clone();
        for e2 in map2 {
            // The map 1 entry range is split into three parts depending on how it overlaps with the map 2 entry

            // Create an entry for the map 1 entry range that is before the map 2 entry range
            if dst_start(&e) < src_start(e2) {
                let part1_size = src_start(e2) - dst_start(&e);
                new_map.push((dst_start(&e), src_start(&e), part1_size));
                e = (dst_start(&e) + part1_size, src_start(&e) + part1_size, size(&e) - part1_size);
            }

            // If the map 1 entry range has been accounted for then move on to the next map 1 entry
            if size(&e) <= 0 {
                break;
            }

            // Create an entry combining overlapping ranges
            if dst_start(&e) < src_end(e2) {
                let part2_size = std::cmp::min(src_end(e2) - dst_start(&e), size(&e));
                new_map.push((dst_start(e2) + dst_start(&e) - src_start(e2), src_start(&e), part2_size));
                e = (dst_start(&e) + part2_size, src_start(&e) + part2_size, size(&e) - part2_size);
            }

            // If the map 1 entry range has been accounted for then move on to the next map 1 entry
            if size(&e) <= 0 {
                break;
            }

            // Otherwise, continue with any remainder to the next map 2 entry
        }

        if size(&e) > 0 {
            new_map.push(e);
        }
    }
    new_map.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    reduce(&new_map)
}

// Returns the start of the source range of the map entry
fn src_start(e: &(i64, i64, i64)) -> i64 {
    e.1
}

// Returns the end of the source range of the map entry
fn src_end(e: &(i64, i64, i64)) -> i64 {
    e.1 + e.2
}

// Returns the start of the destination range of the map entry
fn dst_start(e: &(i64, i64, i64)) -> i64 {
    e.0
}

// Returns the end of the destination range of the map entry
fn dst_end(e: &(i64, i64, i64)) -> i64 {
    e.0 + e.2
}

// Returns the size of the range of the map entry
fn size(e: &(i64, i64, i64)) -> i64 {
    e.2
}

// Combine sorted map entries with adjacent source and destination ranges
fn reduce(map: &Vec<(i64, i64, i64)>) -> Vec<(i64, i64, i64)> {
    let mut new_map = Vec::new();

    let mut i = map.iter();
    let mut e0 = i.next();
    while e0.is_some() {
        let mut new_e = e0.unwrap().clone();
        let mut e1 = i.next();
        while e1.is_some() {
            if adjacent(&new_e, e1.unwrap()) {
                new_e = join(&new_e, e1.unwrap());
            } else if adjacent(e1.unwrap(), &new_e) {
                new_e = join(e1.unwrap(), &new_e);
            } else {
                // If it is not adjacent, then there are no more entries to combine with this one
                break;
            }
            e1 = i.next();
        }
        new_map.push(new_e);
        e0 = e1;
    }
    new_map
}

// Returns true if both the source and destination ranges of the second map entry immediately follow the first map entry
fn adjacent(e1: &(i64, i64, i64), e2: &(i64, i64, i64)) -> bool {
    src_end(e1) == src_start(e2) && dst_end(e1) == dst_start(e2)
}

// Join two adjacent map entries
fn join(e1: &(i64, i64, i64), e2: &(i64, i64, i64)) -> (i64, i64, i64) {
    (dst_start(e1), src_start(e1), size(e1) + size(e2))
}
