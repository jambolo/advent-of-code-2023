use common::load;
#[cfg(feature = "part2")]
use regex::Regex;

fn main() {
    println!("Day 15, part {}", if cfg!(feature="part2") { "2" } else { "1" });
    #[cfg(not(feature = "part2"))]
    {
        let steps = load::comma_separated_values().unwrap();
        let sum: u64 = steps.iter().map(|s| hash(s)).sum();
        println!("Sum: {}", sum);
    }

    #[cfg(feature = "part2")]
    {
        let steps = load::comma_separated_values().unwrap();

        let re_assign = Regex::new(r"^([a-z]+)=(\d+)$").unwrap();
        let re_remove = Regex::new(r"^([a-z]+)-$").unwrap();

        #[derive(Debug)]
        struct Lens {
            symbol: String,
            f: i64,
        }

        let mut boxes: Vec<Vec<Lens>> = Vec::new();
        for _ in 0..256 {
            boxes.push(Vec::new());
        }
        for s in steps {
            if let Some(captured) = re_assign.captures(&s) {
                let symbol = captured.get(1).unwrap().as_str();
                let f: i64 = captured.get(2).unwrap().as_str().parse().unwrap();
                let i = hash(&symbol) as usize;
                let mut found = false;
                for lens in &mut boxes[i] {
                    if lens.symbol == symbol {
                        lens.f = f;
                        found = true;
                        break;
                    }
                }
                if !found {
                    boxes[i].push(Lens { symbol: symbol.to_string(), f });
                }
            } else if let Some(captured) = re_remove.captures(&s) {
                let symbol = captured.get(1).unwrap().as_str();
                let i = hash(&symbol) as usize;
                boxes[i].retain(|lens| lens.symbol != symbol);
            } else {
                println!("No match for: {}", s);
            }
        }
        // Sum the focusing power of lenses in boxes that are not empty
        let sum: i64 = boxes.iter().enumerate().fold(0, |a0, (b, lenses)| {
            a0 + lenses.iter().enumerate().fold(0, |a1, (i, lens)| {
                a1 + focusing_power(b, i, lens.f)
            })
        });
        println!("Sum: {}", sum);
    }
}

fn hash(s: &str) -> u64 {
    s.chars().fold(0, |sum, c| ((sum + c as u64) * 17) & 0xff)
}

#[cfg(feature = "part2")]
fn focusing_power(box_number: usize, i: usize, f: i64) -> i64 {
    (box_number as i64 + 1) * (i as i64 + 1) * f
}