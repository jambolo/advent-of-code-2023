use std::collections::HashMap;

use common::load;

struct Context {
    cache: HashMap<(usize, usize, usize), i64>,
    size: usize,
    template: u128,
    mask: u128,
}

fn main() {
    println!("=== Day 12, part {} ===", if cfg!(feature = "part2") { "2" } else { "1" });
    let lines = load::lines().unwrap();

    let mut sum: i64 = 0;
    for line in lines {
        let (size, template, mask, groups) = parse_line(&line);
        let space = size - (groups.iter().sum::<i32>() as usize + groups.len() - 1);
        let mut context = Context {
            cache: HashMap::new(),
            size,
            template,
            mask,
        };
        let count = number_of_permutations(&mut context, 0, 0, &groups, space);
        sum += count;
    }

    println!("Result: {}", sum);
}

fn insert_ones(x: u128, n: usize) -> u128 {
    (x << n) | ((1 << n) - 1)
}

fn insert_zeros(x: u128, n: usize) -> u128 {
    x << n
}

fn matches_template(x: u128, template: u128, mask: u128) -> bool {
    x & mask == template & mask
}

fn place_group(x: u128, leading_zeros: usize, group_size: usize) -> u128 {
    insert_ones(insert_zeros(x, leading_zeros), group_size)
}

/// Counts valid arrangements matching the template.
///
/// Recursively builds arrangements left-to-right by placing each group with
/// variable leading zeros (0..=space). For each placement:
/// - Base case (last group): completes the pattern with trailing zeros and
///   checks against template
/// - Recursive case: places group + 1 separator zero, prunes if partial wip
///   doesn't match template, then recurses with remaining groups and reduced space
///
/// Parameters:
/// - context: context containing cache, size, template, and mask
/// - size: total length of the spring record
/// - template: bitmask with 1s at '#' positions
/// - mask: bitmask with 0s at '?' positions
/// - wip: work-in-progress bitmask built left-to-right
/// - wip_size: number of bits placed in wip so far
/// - groups: remaining group sizes to place
/// - space: remaining space to distribute
fn number_of_permutations(
    context: &mut Context,
    wip: u128,
    wip_size: usize,
    remaining_groups: &[i32],
    remaining_space: usize,
) -> i64 {
    // Check cache
    if let Some(&cached_count) = context.cache.get(&(wip_size, remaining_groups.len(), remaining_space)) {
        return cached_count;
    }

    let g = remaining_groups[0] as usize;
    let mut count = 0;

    for i in 0..=remaining_space {
        let mut new_wip = place_group(wip, i, g);
        if remaining_groups.len() > 1 {
            // If this is not the last group, force a separator zero.
            new_wip = insert_zeros(new_wip, 1);
            let new_wip_size = wip_size + i + g + 1;
            let shift = context.size - new_wip_size;
            if matches_template(new_wip, context.template >> shift, context.mask >> shift) {
                count += number_of_permutations(
                    context,
                    new_wip,
                    new_wip_size,
                    &remaining_groups[1..],
                    remaining_space - i,
                );
            }
        } else {
            // Last group: complete with trailing zeros and check full match.
            new_wip = insert_zeros(new_wip, remaining_space - i);
            if matches_template(new_wip, context.template, context.mask) {
                count += 1;
            }
        }
    }

    // Cache result
    context.cache.insert((wip_size, remaining_groups.len(), remaining_space), count);

    count
}

fn parse_line(line: &str) -> (usize, u128, u128, Vec<i32>) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let (template, mask) = parse_record(parts[0]);
    let numbers: Vec<i32> = parts[1]
        .split(',')
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();

    if cfg!(feature = "part2") {
        unfolded(&parts, template, mask, &numbers)
    } else {
        (parts[0].len(), template, mask, numbers)
    }
}

fn unfolded(parts: &[&str], template: u128, mask: u128, numbers: &[i32]) -> (usize, u128, u128, Vec<i32>) {
    let fold_length = parts[0].len();
    assert!(fold_length * 5 <= 128);
    let unfolded_size = fold_length * 5 + 4;
    let mut unfolded_template = 0;
    let mut unfolded_mask = !0;
    let mut unfolded_numbers: Vec<i32> = vec![];
    for _ in 0..5 {
        unfolded_template = (unfolded_template << (fold_length + 1)) | template;
        unfolded_mask = (unfolded_mask << (fold_length + 1)) | (mask & ((1 << fold_length) - 1));
        unfolded_numbers.extend(numbers);
    }
    (unfolded_size, unfolded_template, unfolded_mask, unfolded_numbers)
}

/// Returns (template, mask)
/// where template has 1's for positions that are '#' and mask has 0s for positions that are '?'
fn parse_record(record: &str) -> (u128, u128) {
    let (template, mask) = record.chars().fold((0, 0), |(mut template, mut mask), c| {
        template = (template << 1) | if c == '#' { 1 } else { 0 };
        mask = (mask << 1) | if c == '?' { 1 } else { 0 };
        (template, mask)
    });
    (template, !mask)
}
