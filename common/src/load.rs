use std::{env, fs::read_to_string, io, path::Path};

/// Loads the entire contents of the file specified as the first command-line argument into a string.
///
/// # Errors
///
/// Returns an error if the input file argument is missing or the file cannot be read.
pub fn string() -> Result<String, String> {
    let path = get_path()?;
    read_to_string(&path).map_err(|e| format!("Could not read file \"{}\": {}", path, e))
}

/// Loads lines of data from the file specified in the command into a vector of strings.
///
/// # Errors
///
/// Returns an error if the input file argument is missing or the file cannot be read.
pub fn lines() -> Result<Vec<String>, String> {
    let path = get_path()?;
    read_lines(&path).map_err(|e| format!("Could not read file \"{}\": {}", path, e))
}

/// Reads a file line by line into a vector of strings
fn read_lines(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let input = read_to_string(filename)?;
    Ok(input.lines().map(|line| line.to_string()).collect())
}

/// Loads a file into a vector of strings separated by `','`.
///
/// Leading and trailing whitespace is trimmed from each value.
///
/// # Errors
///
/// Returns an error if the input file argument is missing or the file cannot be read.
pub fn comma_separated_values() -> Result<Vec<String>, String> {
    let path = get_path()?;
    read_comma_separated_values(&path).map_err(|e| format!("Could not read file \"{}\": {}", path, e))
}

/// Reads an entire file into a string and splits it by ',' into a vector of strings
fn read_comma_separated_values(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let input = read_to_string(filename)?;
    Ok(input.split(',').map(|s| s.trim().to_string()).collect())
}

/// Loads a file into a 2D array of characters, one row per line.
///
/// # Errors
///
/// Returns an error if the input file argument is missing or the file cannot be read.
pub fn map() -> Result<Vec<Vec<char>>, String> {
    let path = get_path()?;
    read_map(&path).map_err(|e| format!("Could not read file \"{}\": {}", path, e))
}

/// Reads an entire file into a 2D array of characters
fn read_map(filename: impl AsRef<Path>) -> io::Result<Vec<Vec<char>>> {
    let input = read_to_string(filename)?;
    Ok(input.lines().map(|line| line.chars().collect()).collect())
}

/// Loads a file into a 2D array of single-digit numbers (`i32`), one row per line.
///
/// Non-digit characters are silently ignored.
///
/// # Errors
///
/// Returns an error if the input file argument is missing or the file cannot be read.
pub fn numbers_map() -> Result<Vec<Vec<i32>>, String> {
    let path = get_path()?;
    read_numbers_map(&path).map_err(|e| format!("Could not read file \"{}\": {}", path, e))
}

/// Reads an entire file into a 2D array of numbers
fn read_numbers_map(filename: impl AsRef<Path>) -> io::Result<Vec<Vec<i32>>> {
    let input = read_to_string(filename)?;
    Ok(input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10).map(|d| d as i32)).collect())
        .collect())
}

fn get_path() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    args.get(1).cloned().ok_or_else(|| "Missing input file argument".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn temp_file(content: &str) -> std::path::PathBuf {
        let n = FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("aoc_load_test_{}.txt", n));
        std::fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn lines_basic() {
        assert_eq!(read_lines(temp_file("line1\nline2\nline3")).unwrap(), vec!["line1", "line2", "line3"]);
    }

    #[test]
    fn lines_empty_file() {
        assert_eq!(read_lines(temp_file("")).unwrap(), Vec::<String>::new());
    }

    #[test]
    fn lines_single_line() {
        assert_eq!(read_lines(temp_file("only one line")).unwrap(), vec!["only one line"]);
    }

    #[test]
    fn lines_trailing_newline_stripped() {
        assert_eq!(read_lines(temp_file("a\nb\n")).unwrap(), vec!["a", "b"]);
    }

    #[test]
    fn lines_blank_lines_preserved() {
        assert_eq!(read_lines(temp_file("a\n\nb")).unwrap(), vec!["a", "", "b"]);
    }

    #[test]
    fn lines_nonexistent_file() {
        assert!(read_lines("/nonexistent/path/file.txt").is_err());
    }

    #[test]
    fn csv_basic() {
        assert_eq!(read_comma_separated_values(temp_file("a,b,c")).unwrap(), vec!["a", "b", "c"]);
    }

    #[test]
    fn csv_trims_whitespace() {
        assert_eq!(read_comma_separated_values(temp_file("a, b , c")).unwrap(), vec!["a", "b", "c"]);
    }

    #[test]
    fn csv_trims_newlines() {
        assert_eq!(read_comma_separated_values(temp_file("a,b,\nc,d")).unwrap(), vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn csv_single_value() {
        assert_eq!(read_comma_separated_values(temp_file("hello")).unwrap(), vec!["hello"]);
    }

    #[test]
    fn csv_empty_values() {
        assert_eq!(read_comma_separated_values(temp_file("a,,b")).unwrap(), vec!["a", "", "b"]);
    }

    #[test]
    fn map_basic() {
        assert_eq!(read_map(temp_file("abc\ndef")).unwrap(), vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]);
    }

    #[test]
    fn map_empty_file() {
        assert_eq!(read_map(temp_file("")).unwrap(), Vec::<Vec<char>>::new());
    }

    #[test]
    fn map_single_char() {
        assert_eq!(read_map(temp_file("A")).unwrap(), vec![vec!['A']]);
    }

    #[test]
    fn map_preserves_spaces() {
        assert_eq!(read_map(temp_file("a b\nc d")).unwrap(), vec![vec!['a', ' ', 'b'], vec!['c', ' ', 'd']]);
    }

    #[test]
    fn numbers_map_basic() {
        assert_eq!(read_numbers_map(temp_file("123\n456")).unwrap(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    }

    #[test]
    fn numbers_map_ignores_non_digits() {
        assert_eq!(read_numbers_map(temp_file("1a2b3\n4.5.6")).unwrap(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    }

    #[test]
    fn numbers_map_empty_file() {
        assert_eq!(read_numbers_map(temp_file("")).unwrap(), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn numbers_map_all_non_digits() {
        assert_eq!(read_numbers_map(temp_file("abc\ndef")).unwrap(), vec![vec![], vec![]]);
    }
}
