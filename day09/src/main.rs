use common::load;
use itertools::Itertools;

fn main() {
    println!("=== Day 9, part {} ===", if cfg!(feature = "part2") { "2" } else { "1" });
    let lines = load::lines().unwrap();

    #[cfg(not(feature = "part2"))]
    let mut nsum: i64 = 0;
    #[cfg(feature = "part2")]
    let mut psum: i64 = 0;

    for line in lines {
        let numbers: Vec<i64> = line.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();

        let mut ends: Vec<i64> = Vec::new();
        let mut starts: Vec<i64> = Vec::new();
        let mut sequence = numbers.clone();
        while !sequence.iter().all(|&x| x == 0) {
            starts.push(sequence[0]);
            ends.push(*sequence.last().unwrap());
            sequence = next_sequence(&sequence);
        }

        #[cfg(not(feature = "part2"))]
        {
            let n: i64 = ends.iter().sum();
            nsum += n;
        }

        #[cfg(feature = "part2")]
        {
            let mut p: i64 = 0;
            for s in starts.iter().rev() {
                p = s - p;
            }
            psum += p;
        }
    }

    #[cfg(not(feature = "part2"))]
    println!("Result: {}", nsum);

    #[cfg(feature = "part2")]
    println!("Result: {}", psum);
}

fn next_sequence(sequence: &[i64]) -> Vec<i64> {
    sequence.iter().tuple_windows().map(|(a, b)| b - a).collect()
}
