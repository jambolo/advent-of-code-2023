use common::load;

#[derive(Clone)]
struct MapEntry {
    dst: i64,
    src: i64,
    size: i64,
}

#[derive(Clone)]
struct SeedRange {
    start: i64,
    size: i64,
}

fn main() {
    println!("=== Day 5, part {} ===", if cfg!(feature = "part2") { "2" } else { "1" });
    let lines = load::lines().unwrap();
    let mut iter = lines.iter();

        let seeds = parse_seeds(&mut iter);
        iter.next(); // Skip blank line

        // Load each map
        let mut maps: Vec<Vec<MapEntry>> = Vec::new();
        while let Some(_line) = iter.next() {
            maps.push(parse_map(&mut iter));
        }

    let result = if cfg!(feature = "part2") { part2(&seeds, &maps) } else { part1(&seeds, &maps) };
    println!("Result: {}", result);
}

fn part2(seeds: &[i64], maps: &[Vec<MapEntry>]) -> i64
{
    // Part 2: seeds are ranges of values in pairs of start and size
    let seed_ranges: Vec<SeedRange> = seeds.chunks(2).map(|s| SeedRange { start: s[0], size: s[1] }).collect();
    let map = maps.iter()
        .fold(create_map_from_seeds(&seed_ranges),
            |combined, m| combine(&combined, m)
        );
    map[0].dst
}

fn part1(seeds: &[i64], maps: &[Vec<MapEntry>]) -> i64
{
    let mut min_location = i64::MAX;
    for &seed in seeds {
        let mut value = seed;
        for map in maps {
            value = lookup(value, map);
        }
        min_location = min_location.min(value);
    }

    min_location
}

fn parse_seeds<'a, I>(iter: &mut I) -> Vec<i64>
where
    I: Iterator<Item = &'a String>,
{
    let input = iter.next().unwrap();
    let parts: Vec<&str> = input.split(":").collect();
    let seeds: Vec<i64> = parts[1].split_whitespace().map(|s| s.parse().unwrap()).collect();

    seeds
}

fn parse_map<'a, I>(iter: &mut I) -> Vec<MapEntry>
where
    I: Iterator<Item = &'a String>,
{
    let mut map = Vec::new();

    for line in iter {
        if line.trim().is_empty() {
            break;
        }

        let parts: Vec<i64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();

        map.push(MapEntry { dst: parts[0], src: parts[1], size: parts[2] });
    }

    map.sort_unstable_by(|a, b| a.src.cmp(&b.src));
    map
}

fn lookup(value: i64, map: &[MapEntry]) -> i64 {
    for entry in map {
        if value >= entry.src && value < entry.src + entry.size {
            return entry.dst + (value - entry.src);
        }
    }
    value
}

fn create_map_from_seeds(seeds: &[SeedRange]) -> Vec<MapEntry> {
    let mut map = Vec::new();

    for s in seeds {
        map.push(MapEntry { dst: s.start, src: s.start, size: s.size });
    }

    map.sort_unstable_by(|a, b| a.dst.cmp(&b.dst));
    map
}

// Combine two maps, ignoring source ranges in the second map that are outside of the destination ranges in the first map
fn combine(map1: &[MapEntry], map2: &[MapEntry]) -> Vec<MapEntry> {
    let mut new_map: Vec<MapEntry> = Vec::new();

    for e1 in map1 {
        let mut e = e1.clone();
        for e2 in map2 {
            // The map 1 entry range is split into three parts depending on how it overlaps with the map 2 entry

            // Create an entry for the map 1 entry range that is before the map 2 entry range
            if e.dst < e2.src {
                let part1_size = e2.src - e.dst;
                new_map.push(MapEntry { dst: e.dst, src: e.src, size: part1_size });
                e = MapEntry { dst: e.dst + part1_size, src: e.src + part1_size, size: e.size - part1_size };
            }

            // If the map 1 entry range has been accounted for then move on to the next map 1 entry
            if e.size <= 0 {
                break;
            }

            // Create an entry combining overlapping ranges
            if e.dst < src_end(e2) {
                let part2_size = std::cmp::min(src_end(e2) - e.dst, e.size);
                new_map.push(MapEntry{ dst: e2.dst + e.dst - e2.src, src: e.src, size: part2_size });
                e = MapEntry { dst: e.dst + part2_size, src: e.src + part2_size, size: e.size - part2_size };
            }

            // If the map 1 entry range has been accounted for then move on to the next map 1 entry
            if e.size <= 0 {
                break;
            }

            // Otherwise, continue with any remainder to the next map 2 entry
        }

        if e.size > 0 {
            new_map.push(e);
        }
    }
    new_map.sort_unstable_by(|a, b| a.dst.cmp(&b.dst));
    reduce(&new_map)
}

// Returns the end of the source range of the map entry
fn src_end(e: &MapEntry) -> i64 {
    e.src + e.size
}

// Returns the end of the destination range of the map entry
fn dst_end(e: &MapEntry) -> i64 {
    e.dst + e.size
}

// Combine sorted map entries with adjacent source and destination ranges
fn reduce(map: &[MapEntry]) -> Vec<MapEntry> {
    let mut new_map = Vec::<MapEntry>::new();

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
fn adjacent(e1: &MapEntry, e2: &MapEntry) -> bool {
    src_end(e1) == e2.src && dst_end(e1) == e2.dst
}

// Join two adjacent map entries
fn join(e1: &MapEntry, e2: &MapEntry) -> MapEntry {
    MapEntry { dst: e1.dst, src: e1.src, size: e1.size + e2.size }
}
