use common::load;

fn main() {
    println!("Day 12, part {}", if cfg!(feature="part2") { "2" } else { "1" });
    let lines = load::lines().unwrap();

    let mut sum: i64 = 0;
    for line in lines {
        let (size, template, mask, groups) = parse_line(&line);
        let space = size - (groups.iter().sum::<i32>() as usize + groups.len() - 1);
        let count = number_of_permutations(size, template, mask, 0, 0, &groups, space);
        println!("{}: {} {:b} {:b} {:?} {}", line, size, template, mask, groups, count);
        sum += count;
    }

    println!("Sum: {}", sum);
}

fn insert_ones(x: u128, n: usize) -> u128 {
    x << n | (1 << n) - 1
}

fn insert_zeros(x: u128, n: usize) -> u128 {
    x << n
}

fn matches_template(x: u128, template: u128, mask: u128) -> bool {
    x & mask == template & mask
}
fn number_of_permutations(
    size: usize,
    template: u128,
    mask: u128,
    wip: u128,
    wip_size: usize,
    groups: &[i32],
    space: usize,
) -> i64 {
    let g = groups[0] as usize;
    let mut count = 0;

    for i in 0..=space {
        if groups.len() == 1 {
            let new_wip = insert_zeros(wip, i);
            let new_wip = insert_ones(new_wip, g);
            let new_wip = insert_zeros(new_wip, space - i);
            if matches_template(new_wip, template, mask) {
                count += 1;
            }
        } else {
            let new_wip = insert_zeros(wip, i);
            let new_wip = insert_ones(new_wip, g);
            let new_wip = insert_zeros(new_wip, 1);
            let new_wip_size = wip_size + i + g + 1;
            let shift = size - new_wip_size;
            if matches_template(new_wip << shift, template, (mask >> shift) << shift) {
                count += number_of_permutations(
                    size,
                    template,
                    mask,
                    new_wip,
                    new_wip_size,
                    &groups[1..],
                    space - i,
                );
            }
        }
    }
    count
}

fn parse_line(line: &str) -> (usize, u128, u128, Vec<i32>) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let (template, mask) = parse_record(parts[0]);
    let numbers: Vec<i32> = parts[1].split(',').map(|s| s.parse().expect("Failed to parse number")).collect();

    if cfg!(feature="part2") {
        let fold_length = parts[0].len();
        assert!(fold_length * 5 <= 128);
        let unfolded_size = fold_length * 5 + 4;
        let mut unfolded_template = 0;
        let mut unfolded_mask = !0;
        let mut unfolded_numbers: Vec<i32> = vec![];
        for _ in 0..5 {
            unfolded_template = (unfolded_template << (fold_length + 1)) | template;
            unfolded_mask = (unfolded_mask << (fold_length + 1)) | (mask & ((1 << fold_length) - 1));
            unfolded_numbers.extend(&numbers);
        }
        return (unfolded_size, unfolded_template, unfolded_mask, unfolded_numbers);
    } else {
        return (parts[0].len(), template, mask, numbers);
    }
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
