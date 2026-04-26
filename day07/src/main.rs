use common::load;

#[cfg(not(feature = "part2"))]
const SORT_ORDER: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

#[cfg(feature = "part2")]
const SORT_ORDER: [char; 13] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

fn main() {
    println!("=== Day 7, part {} ===", if cfg!(feature = "part2") { "2" } else { "1" });
    let lines = load::lines().unwrap();

    let mut game = lines.into_iter()
        .map(|line| parse_line(&line))
        .collect::<Vec<(Vec<char>, i64)>>();
    game.sort_unstable_by(|a, b| hand_sorter(&a.0, &b.0));

    let mut sum: i64 = 0;
    for i in 0..game.len() {
        let bid = game[i].1;
        let rank = (game.len() - i) as i64;
        sum += bid * rank;
    }

    println!("Result: {}", sum);
}

fn card_sorter(a: &char, b: &char) -> std::cmp::Ordering {
    let ia = SORT_ORDER.iter().position(|&x| x == *a).unwrap();
    let ib = SORT_ORDER.iter().position(|&x| x == *b).unwrap();

    ia.cmp(&ib)
}

fn hand_sorter(a: &[char], b: &[char]) -> std::cmp::Ordering {
    let a_type = classify(a);
    let b_type = classify(b);
    if a_type != b_type {
        return a_type.cmp(&b_type);
    }

    // Same type, sort by value
    for i in 0..a.len() {
        if a[i] != b[i] {
            return card_sorter(&a[i], &b[i]);
        }
    }

    std::cmp::Ordering::Equal
}

fn parse_line(line: &str) -> (Vec<char>, i64) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let hand = parts[0].chars().collect();
    let bid = parts[1].parse().unwrap();
    (hand, bid)
}

// Returns the type of hand
fn classify(hand: &[char]) -> i64 {
    let mut sorted = hand.to_vec();
    sorted.sort_unstable_by(card_sorter);

    if is_five_of_a_kind(&sorted) {
        0
    } else if is_four_of_a_kind(&sorted) {
        1
    } else if is_full_house(&sorted) {
        2
    } else if is_three_of_a_kind(&sorted) {
        3
    } else if is_two_pair(&sorted) {
        4
    } else if is_pair(&sorted) {
        5
    } else {
        6
    }
}

#[cfg(not(feature = "part2"))]
fn is_five_of_a_kind(hand: &[char]) -> bool {
    let c = hand[0];
    hand.iter().all(|&card| card == c)
}

#[cfg(feature = "part2")]
fn is_five_of_a_kind(hand: &[char]) -> bool {
    let c = hand[0];
    for card in hand {
        if *card != c && *card != 'J' {
            return false;
        }
    }
    true
}

#[cfg(not(feature = "part2"))]
fn is_four_of_a_kind(hand: &[char]) -> bool {
    let mut counts = [0; 13];
    for &card in hand {
        if let Some(pos) = SORT_ORDER.iter().position(|&x| x == card) {
            counts[pos] += 1;
        }
    }
    counts.contains(&4)
}

#[cfg(feature = "part2")]
fn is_four_of_a_kind(hand: &[char]) -> bool {
    let mut count = 0;
    let mut last = ' ';
    let number_of_jokers = hand.iter().filter(|&x| *x == 'J').count() as i32;
    for card in hand {
        if *card == last {
            count += 1;
        } else if *card == 'J' {
            continue;
        } else {
            count = 1;
        }
        last = *card;
        if count + number_of_jokers == 4 {
            return true;
        }
    }
    false
}

#[cfg(not(feature = "part2"))]
fn is_full_house(hand: &[char]) -> bool {
    let mut counts = [0; 13];
    for &card in hand {
        if let Some(pos) = SORT_ORDER.iter().position(|&x| x == card) {
            counts[pos] += 1;
        }
    }
    counts.contains(&3) && counts.contains(&2)
}

#[cfg(feature = "part2")]
fn is_full_house(hand: &[char]) -> bool {
    let mut count1 = 0;
    let mut count2 = 0;
    let mut last1 = ' ';
    let mut last2 = ' ';
    for card in hand {
        if *card == last1 {
            count1 += 1;
        } else if *card == last2 {
            count2 += 1;
        } else if count1 == 0 {
            count1 = 1;
            last1 = *card;
        } else if count2 == 0 {
            count2 = 1;
            last2 = *card;
        } else if *card == 'J' {
            continue;
        } else {
            return false;
        }
    }
    true
}

#[cfg(not(feature = "part2"))]
fn is_three_of_a_kind(hand: &[char]) -> bool {
    let mut counts = [0; 13];
    for &card in hand {
        if let Some(pos) = SORT_ORDER.iter().position(|&x| x == card) {
            counts[pos] += 1;
        }
    }
    counts.contains(&3)
}

#[cfg(feature = "part2")]
fn is_three_of_a_kind(hand: &[char]) -> bool {
    let mut count = 0;
    let mut last = ' ';
    let number_of_jokers = hand.iter().filter(|&x| *x == 'J').count() as i32;
    for card in hand {
        if *card == last {
            count += 1;
        } else if *card == 'J' {
            continue;
        } else {
            count = 1;
        }
        last = *card;
        if count + number_of_jokers == 3 {
            return true;
        }
    }
    false
}

#[cfg(not(feature = "part2"))]
fn is_two_pair(hand: &[char]) -> bool {
    let mut counts = [0; 13];
    for &card in hand {
        if let Some(pos) = SORT_ORDER.iter().position(|&x| x == card) {
            counts[pos] += 1;
        }
    }
    counts.iter().filter(|&&c| c == 2).count() == 2
}

#[cfg(feature = "part2")]
fn is_two_pair(hand: &[char]) -> bool {
    let mut count1 = 0;
    let mut count2 = 0;
    let mut count3 = 0;
    let mut last1 = ' ';
    let mut last2 = ' ';
    let mut last3 = ' ';
    for card in hand {
        if *card == last1 {
            count1 += 1;
        } else if *card == last2 {
            count2 += 1;
        } else if *card == last3 {
            count3 += 1;
        } else if count1 == 0 {
            count1 = 1;
            last1 = *card;
        } else if count2 == 0 {
            count2 = 1;
            last2 = *card;
        } else if count3 == 0 {
            count3 = 1;
            last3 = *card;
        } else if *card == 'J' {
            continue;
        } else {
            return false;
        }
    }
    true
}

#[cfg(not(feature = "part2"))]
fn is_pair(hand: &[char]) -> bool {
    let mut counts = [0; 13];
    for &card in hand {
        if let Some(pos) = SORT_ORDER.iter().position(|&x| x == card) {
            counts[pos] += 1;
        }
    }
    counts.iter().filter(|&&c| c == 2).count() == 1
}

#[cfg(feature = "part2")]
fn is_pair(hand: &[char]) -> bool {
    let mut count = 0;
    let mut last = ' ';
    let number_of_jokers = hand.iter().filter(|&x| *x == 'J').count() as i32;
    for card in hand {
        if *card == last {
            count += 1;
        } else if *card == 'J' {
            continue;
        } else {
            count = 1;
        }
        last = *card;
        if count + number_of_jokers == 2 {
            return true;
        }
    }
    false
}
