use std::{
    env,
    fs::read_to_string,
    io,
    path::Path,
};

/// Loads lines of data from the file specified in the command into a vector of strings
pub fn lines() -> Result<Vec<String>, String> {
    let path = get_path()?;
    read_lines(&path).map_err(|e| format!("Could not read file \"{}\": {}", path, e))
}

/// Reads a file line by line into a vector of strings
fn read_lines(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let input = read_to_string(filename)?;
    Ok(input.lines().map(|line| line.to_string()).collect())
}

/// Loads a file into a vector of strings separated by ','.
pub fn comma_separated_values() -> Result<Vec<String>, String> {
    let path = get_path()?;
    read_comma_separated_values(&path).map_err(|e| format!("Could not read file \"{}\": {}", path, e))
}

/// Reads an entire file into a string and splits it by ',' into a vector of strings
fn read_comma_separated_values(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let input = read_to_string(filename)?;
    Ok(input.split(',').map(|s| s.trim().to_string()).collect())
}

/// Loads a file into a 2D array of characters
pub fn map() -> Result<Vec<Vec<char>>, String> {
    let path = get_path()?;
    read_map(&path).map_err(|e| format!("Could not read file \"{}\": {}", path, e))
}

/// Reads an entire file into a 2D array of characters
fn read_map(filename: impl AsRef<Path>) -> io::Result<Vec<Vec<char>>> {
    let input = read_to_string(filename)?;
    Ok(input.lines().map(|line| line.chars().collect()).collect())
}

/// Loads a file into a 2D array of numbers
pub fn numbers_map() -> Result<Vec<Vec<i32>>, String> {
    let path = get_path()?;
    read_numbers_map(&path).map_err(|e| format!("Could not read file \"{}\": {}", path, e))
}

/// Reads an entire file into a 2D array of numbers
fn read_numbers_map(filename: impl AsRef<Path>) -> io::Result<Vec<Vec<i32>>> {
    let input = read_to_string(filename)?;
    Ok(input.lines()
        .map(|line| line
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as i32))
            .collect()
        )
        .collect()
    )
}

fn get_path() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    args.get(1)
        .cloned()
        .ok_or_else(|| "Missing input file argument".to_string())
}
