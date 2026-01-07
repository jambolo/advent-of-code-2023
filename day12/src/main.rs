use common::load;

fn main() {
    println!("Day 12, part {}", if cfg!(feature="part2") { "2" } else { "1" });
    let lines = load::lines().unwrap();

    let mut sum: i64 = 0;
    for line in lines {
        let (size, template, mask, groups) = parse_line(&line);
        let space = size - (groups.iter().sum::<i32>() as usize + groups.len() - 1);
        let count = number_of_permutations(template, mask, 0, &groups, space);
        sum += count;
    }

    println!("Sum: {}", sum);
}

fn insert_ones(x: u64, n: usize) -> u64 {
    x << n | (1 << n) - 1
}

fn insert_zeros(x: u64, n: usize) -> u64 {
    x << n
}

fn matches_template(x: u64, template: u64, mask: u64) -> bool {
    x & mask == template & mask
}
fn number_of_permutations(
    template: u64,
    mask: u64,
    wip: u64,
    groups: &Vec<i32>,
    space: usize,
) -> i64 {
    let g = groups[0] as usize;
    let mut count = 0;

    for i in 0..=space {

        if groups.len() == 1 {
            let new_wip = insert_zeros(insert_ones(insert_zeros(wip, i), g), space - i);
            if matches_template(new_wip, template, mask) {
                count += 1;
            }
        } else {
            let new_wip = insert_ones(insert_zeros(wip, i), g) << 1;
            count += number_of_permutations(
                template,
                mask,
                new_wip,
                &groups[1..].to_vec(),
                space - i,
            );
        }
    }
    count
}

fn parse_line(line: &String) -> (usize, u64, u64, Vec<i32>) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let (template, mask) = parse_record(parts[0]);
    let numbers: Vec<i32> = parts[1].split(',').map(|s| s.parse().unwrap()).collect();

    (parts[0].len(), template, mask, numbers)
}

fn parse_record(record: &str) -> (u64, u64) {
    let (template, mask) = record.chars().fold((0, 0), |(mut template, mut mask), c| {
        template = (template << 1) | if c == '#' { 1 } else { 0 };
        mask = (mask << 1) | if c == '?' { 1 } else { 0 };
        (template, mask)
    });
    (template, !mask) // Note: mask is inverted so that we can use bitwise AND
}
