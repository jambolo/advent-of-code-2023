use common::load;
use regex::Regex;

// which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
const MAX: (i32, i32, i32) = (12, 13, 14);

fn main() {
    println!("=== Day 2, part {} ===", if cfg!(feature = "part2") { "2" } else { "1" });

    let games = load::lines().unwrap();

    let result = if cfg!(feature = "part2") { part2(&games) } else { part1(&games) };
    println!("Result: {}", result);
}

fn part1(games: &[String]) -> i32 {
    let mut id_sum = 0;

    let game_regex = Regex::new(r"Game (\d+):").unwrap();
    let tuple_regex = Regex::new(r"(\d+) (\w+)([,;]?)").unwrap();

    // Check games
    for game in games {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let mut failed = false;

        // Parse a game
        let game_id: i32 = game_regex.captures(game).unwrap()[1].parse().unwrap();
        for cap in tuple_regex.captures_iter(game) {
            let number: i32 = cap[1].parse().unwrap();
            let color: &str = &cap[2];
            let separator: &str = &cap[3];
            match color {
                "red" => red += number,
                "green" => green += number,
                "blue" => blue += number,
                _ => panic!("Unknown color {}", color),
            }
            if separator != "," {
                if red > MAX.0 || green > MAX.1 || blue > MAX.2 {
                    failed = true;
                }
                red = 0;
                green = 0;
                blue = 0;
            }
        }
        if !failed {
            id_sum += game_id;
        }
    }

    id_sum
}

fn part2(games: &[String]) -> i32 {
    let mut sum_of_powers = 0;
    let tuple_regex = Regex::new(r"(\d+) (\w+)([,;]?)").unwrap();

    // Check games
    for game in games {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let mut max_red: Option<i32> = None;
        let mut max_green: Option<i32> = None;
        let mut max_blue: Option<i32> = None;

        // Parse a game
        for cap in tuple_regex.captures_iter(game) {
            let number: i32 = cap[1].parse().unwrap();
            let color: &str = &cap[2];
            let separator: &str = &cap[3];
            match color {
                "red" => red += number,
                "green" => green += number,
                "blue" => blue += number,
                _ => panic!("Unknown color {}", color),
            }
            if separator != "," {
                if max_red.is_none() || red > max_red.unwrap() {
                    max_red = Some(red);
                }
                if max_green.is_none() || green > max_green.unwrap() {
                    max_green = Some(green);
                }
                if max_blue.is_none() || blue > max_blue.unwrap() {
                    max_blue = Some(blue);
                }

                red = 0;
                green = 0;
                blue = 0;
            }
        }

        {
            let power = max_red.unwrap() * max_green.unwrap() * max_blue.unwrap();
            sum_of_powers += power;
        }
    }

    sum_of_powers
}
