use common::load;

fn main() {
    println!("=== Day 6, part {} ===", if cfg!(feature = "part2") { "2" } else { "1" });
    let lines = load::lines().unwrap();

    if cfg!(feature = "part2")
    {
        let time = parse_line_part2(&lines[0]);
        let distance = parse_line_part2(&lines[1]);
        let ways = count_ways(time, distance);
        println!("Result: {}", ways);
    } else {
        let times = parse_line_part1(&lines[0]);
        let distances = parse_line_part1(&lines[1]);
        let result: i64 = times
            .iter()
            .zip(distances.iter())
            .map(|(&time, &distance)| count_ways(time as f64, distance as f64))
            .product();
        println!("Result: {}", result);
    }
}

fn count_ways(time: f64, distance: f64) -> i64 {
    let mut lower = ((time - (time * time - 4.0 * distance).sqrt()) / 2.0).ceil();
    if (time - lower) * lower <= distance {
        lower += 1.0;
    }
    let mut upper = ((time + (time * time - 4.0 * distance).sqrt()) / 2.0).ceil();
    if (time - upper) * upper <= distance {
        upper -= 1.0;
    }
    (upper - lower + 1.0) as i64
}

fn parse_line_part1(line: &str) -> Vec<i32> {
    line.split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn parse_line_part2(line: &str) -> f64 {
    line.split(':')
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse::<f64>()
        .unwrap()
}
