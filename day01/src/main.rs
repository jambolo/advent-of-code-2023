use common::load;

const DIGIT_NAMES: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

// Returns the first digit in a string
fn first_number_in(s: &str) -> Option<(usize, u32)> {
    s.char_indices()
        .find_map(|(i, c)| if c.is_ascii_digit() { Some((i, c.to_digit(10)?)) } else { None })
}

// Returns the last digit in a string
fn last_number_in(s: &str) -> Option<(usize, u32)> {
    s.char_indices()
        .rev()
        .find_map(|(i, c)| if c.is_ascii_digit() { Some((i, c.to_digit(10)?)) } else { None })
}

// Returns the first digit by name in a string
fn first_name_in(s: &str) -> Option<(usize, usize)> {
    let mut first_value: Option<usize> = None;
    let mut first_pos: Option<usize> = None;
    for (value, &digit_name) in DIGIT_NAMES.iter().enumerate() {
        if let Some(pos) = s.find(digit_name) {
            if first_pos.is_none() || pos < first_pos.unwrap() {
                first_pos = Some(pos);
                first_value = Some(value);
            }
        }
    }
    if first_pos.is_none() {
        return None;
    }
    return Some((first_pos.unwrap(), first_value.unwrap()));
}

// Returns the last digit by name in a string
fn last_name_in(s: &str) -> Option<(usize, usize)> {
    let mut last_value: Option<usize> = None;
    let mut last_pos: Option<usize> = None;
    for (value, &digit_name) in DIGIT_NAMES.iter().enumerate().rev() {
        if let Some(pos) = s.rfind(digit_name) {
            if last_pos.is_none() || pos > last_pos.unwrap() {
                last_pos = Some(pos);
                last_value = Some(value);
            }
        }
    }
    if last_pos.is_none() {
        return None;
    }
    return Some((last_pos.unwrap(), last_value.unwrap()));
}

// Returns the first digit
fn first_value((a1, a2): (usize, u32), (b1, b2): (usize, usize)) -> u32 {
    if a1 < b1 {
        a2
    } else {
        b2 as u32
    }
}
// Returns the last digit
fn last_value((a1, a2): (usize, u32), (b1, b2): (usize, usize)) -> u32 {
    if a1 > b1 {
        a2
    } else {
        b2 as u32
    }
}
fn main() {
    println!("Day 1, part {}", if cfg!(feature="part2") { "2" } else { "1" });
    let lines = load::lines().unwrap();

    let mut sum = 0;
    for line in lines {
        let first_number = first_number_in(&line).unwrap();
        let (_, mut first_digit) = first_number;
        let first_name = first_name_in(&line);
        if first_name.is_some() {
            first_digit = first_value(first_number, first_name.unwrap());
        } 
        let last_number = last_number_in(&line).unwrap();
        let (_, mut last_digit) = last_number;
        let last_name = last_name_in(&line);
        if last_name.is_some() {
            last_digit = last_value(last_number, last_name.unwrap());
        } 
//        println!("{} {} {} {}", first_digit, last_digit, first_digit * 10 + last_digit, line);
        sum += first_digit * 10 + last_digit;
    }

    println!("Sum: {}", sum);
}
