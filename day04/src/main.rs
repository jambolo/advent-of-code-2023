use common::load;

fn main() {
    println!("=== Day 4, part {} ===", if cfg!(feature = "part2") { "2" } else { "1" });
    let lines = load::lines().unwrap();

    // Parse the cards
    let cards = lines.iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect();
            let _id: i32 = parts[0].split_whitespace().last().unwrap().parse().unwrap();

            let sets: Vec<&str> = parts[1].split("|").collect();
            let mut winning = sets[0].split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<i32>>();
            let mut yours = sets[1].split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<i32>>();
            winning.sort();
            yours.sort();
            (1, winning, yours)})
        .collect();

    let result = if cfg!(feature = "part2") { part2(cards) } else { part1(cards) };
    println!("Result: {}", result);
}

fn part1(cards: Vec<(i32, Vec<i32>, Vec<i32>)>) -> i32
{
    let mut total_points = 0;
    for card in &cards {
        let winners = intersection(&card.1, &card.2);
        if !winners.is_empty() {
            let matches = winners.len() as u32;
            let points = 2_i32.pow(matches - 1);
            total_points += points;
        }
    }
    total_points
}

fn part2(mut cards: Vec<(i32, Vec<i32>, Vec<i32>)>) -> i32
{
    let mut count = 0;
    for i in 0..cards.len() {
        count += cards[i].0;
        let winners = intersection(&cards[i].1, &cards[i].2);
        if !winners.is_empty() {
            for j in i + 1..=i + winners.len() {
                cards[j].0 += cards[i].0;
            }
        }
    }
    count
}

// Returns the intersection of two sorted vectors
fn intersection(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            i += 1;
        } else if a[i] > b[j] {
            j += 1;
        } else {
            result.push(a[i]);
            i += 1;
            j += 1;
        }
    }

    result
}
