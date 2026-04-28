use common::load;
use itertools::Itertools;

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
    let c = hand[0]; // The hand is sorted so the first card can never be a joker
    hand.iter().all(|&card| card == c || card == 'J')
}

#[cfg(not(feature = "part2"))]
fn is_four_of_a_kind(hand: &[char]) -> bool {
    let counts = hand.iter().counts();
    counts.values().any(|&n| n >= 4)
}

#[cfg(feature = "part2")]
fn is_four_of_a_kind(hand: &[char]) -> bool {
    let n_jokers = hand.iter().filter(|&&c| c == 'J').count();
    hand.iter()
        .filter(|&&c| c != 'J')
        .counts()
        .values()
        .any(|&n| n + n_jokers >= 4)
}

#[cfg(not(feature = "part2"))]
fn is_full_house(hand: &[char]) -> bool {
    let counts = hand.iter().counts();
    let values = counts.values().cloned().collect::<Vec<usize>>();
    values.contains(&3) && values.contains(&2)
}

#[cfg(feature = "part2")]
fn is_full_house(hand: &[char]) -> bool {
    let n_jokers = hand.iter().filter(|&&c| c == 'J').count();
    let counts = hand.iter().filter(|&&c| c != 'J').counts();
    let values = counts.values().cloned().collect::<Vec<usize>>();

    (values.contains(&3) && values.contains(&2)) ||                         // xxx yy
    (values.contains(&3) && n_jokers >= 1) ||                               // xxx yJ OR xxx JJ
    (values.iter().filter(|&&v| v == 2).count() == 2 && n_jokers >= 1) ||    // xxJ yy
    (values.contains(&2) && n_jokers >= 2) ||                               // xxJ yJ OR xxJ JJ
    (n_jokers >= 3)                                                         // xxJ JJ OR xJJ yJ OR xJJ JJ OR JJJ JJ
}

#[cfg(not(feature = "part2"))]
fn is_three_of_a_kind(hand: &[char]) -> bool {
    let counts = hand.iter().counts();
    counts.values().any(|&n| n >= 3)
}

#[cfg(feature = "part2")]
fn is_three_of_a_kind(hand: &[char]) -> bool {
    let n_jokers = hand.iter().filter(|&&c| c == 'J').count();
    hand.iter()
        .filter(|&&c| c != 'J')
        .counts()
        .values()
        .any(|&n| n + n_jokers >= 3)
}

#[cfg(not(feature = "part2"))]
fn is_two_pair(hand: &[char]) -> bool {
    let counts = hand.iter().counts();
    counts.values().filter(|&&n| n >= 2).count() == 2
}

#[cfg(feature = "part2")]
fn is_two_pair(hand: &[char]) -> bool {
    let n_jokers = hand.iter().filter(|&&c| c == 'J').count();
    let counts = hand.iter().filter(|&&c| c != 'J').counts();
    let values = counts.values().cloned().collect::<Vec<usize>>();

    (values.iter().filter(|&&v| v >= 2).count() >= 2) ||    // xx yy -
    (values.iter().any(|&v| v >= 2) && n_jokers >= 1) ||    // xx yJ - OR xx xJ -
    (n_jokers >= 2)                                         // xJ yJ - OR xJ xJ - OR xJ JJ -
}

#[cfg(not(feature = "part2"))]
fn is_pair(hand: &[char]) -> bool {
    let counts = hand.iter().counts();
    counts.values().any(|&n| n >= 2)
}

#[cfg(feature = "part2")]
fn is_pair(hand: &[char]) -> bool {
    let n_jokers = hand.iter().filter(|&&c| c == 'J').count();
    let counts = hand.iter().counts();

    counts.values().any(|&n| n >= 2) || n_jokers >= 1
}
