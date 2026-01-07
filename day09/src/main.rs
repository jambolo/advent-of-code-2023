use common::load;

fn main() {
    println!("Day 9, part {}", if cfg!(feature="part2") { "2" } else { "1" });
    let lines = load::lines().unwrap();

    let mut nsum: i64 = 0;
    let mut psum: i64 = 0;
    for line in lines {
        let numbers: Vec<i64> = line.split_ascii_whitespace()
                                    .map(|s| s.parse().unwrap())
                                    .collect();

        let mut ends: Vec<i64> = Vec::new();
        let mut starts: Vec<i64> = Vec::new();
        let mut sequence = numbers.clone();
        while !all_zeros(&sequence) {
            println!("{:?}", sequence);
            starts.push(sequence[0]);
            ends.push(*sequence.last().unwrap());
            sequence = next_sequence(&sequence);
        }
        let n: i64 = ends.iter().sum();
        println!("n: {}", n);
        nsum += n;

        let mut p: i64 = 0;
        for s in starts.iter().rev() {
            p = s - p;
        }
        println!("p: {} {:?}", p, starts);
        psum += p;
    }

    println!("nsum: {}", nsum);
    println!("psum: {}", psum);
}

fn all_zeros(sequence: &Vec<i64>) -> bool {
    for i in sequence {
        if *i != 0 {
            return false;
        }
    }
    true
}

fn next_sequence(sequence: &Vec<i64>) -> Vec<i64> {
    let mut new_sequence = Vec::new();
    for i in 1..sequence.len() {
        new_sequence.push(sequence[i] - sequence[i-1]);
    }
    new_sequence
}