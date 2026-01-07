use common::load;

fn main() {
    println!("Day 4, part {}", if cfg!(feature="part2") { "2" } else { "1" });
    let lines = load::lines().unwrap();

    // Parse the cards
    let mut cards: Vec<(i32, Vec<i32>, Vec<i32>)> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split(":").collect();
        let _id: i32 = parts[0].trim().split_whitespace().last().unwrap().parse().unwrap();

        let sets: Vec<&str> = parts[1].split("|").collect();
        let mut winning: Vec<i32> = sets[0].trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let mut yours: Vec<i32> = sets[1].trim().split_whitespace().map(|s| s.parse().unwrap()).collect();

        winning.sort();
        yours.sort();
        cards.push((1, winning, yours));
    }

    // Find the winning cards and accumulate more cards
    let mut count = 0;
    for i in 0..cards.len() {
        count += cards[i].0;
        let winners = intersection(&cards[i].1, &cards[i].2);
        if winners.len() > 0 {
            for j in i + 1 ..= i + winners.len() {
                cards[j].0 += cards[i].0;
            }
        }
    }

    println!("Count: {}", count);
}

 // Returns the intersection of two sorted vectors
fn intersection(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
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