use std::io::{self, BufRead};

fn is_add(line: &str) -> bool {
    line.starts_with('+') && !line.starts_with("+++")
}

fn is_del(line: &str) -> bool {
    line.starts_with('-') && !line.starts_with("---")
}

fn word_count(line: &str) -> usize {
    line.split_whitespace().count()
}

fn main() {
    let diff_lines: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();

    let added_lines = diff_lines.iter().filter(|line| is_add(line));
    let removed_lines = diff_lines.iter().filter(|line| is_del(line));

    let added_words = added_lines
        .clone()
        .fold(0, |sum, line| word_count(line) + sum);
    let removed_words = removed_lines
        .clone()
        .fold(0, |sum, line| word_count(line) + sum);

    println!("{} lines of diff", diff_lines.len());
    println!(
        "{} lines (+{}, -{})",
        (added_lines.clone().count() as i32 - removed_lines.clone().count() as i32).abs(),
        added_lines.count(),
        removed_lines.count()
    );
    println!(
        "{} words (+{}, -{})",
        (added_words as i32 - removed_words as i32).abs(),
        added_words,
        removed_words
    );
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts_words_in_line() {
        assert_eq!(word_count("this is a line"), 4);
    }

    #[test]
    fn test_valid_del_line() {
        assert!(!is_add("-use std::io::{self, BufRead};"));
        assert!(is_del("-use std::io::{self, BufRead};"));
    }

    #[test]
    fn test_valid_add_line() {
        assert!(is_add("+use std::io::{self, BufRead};"));
        assert!(!is_del("+use std::io::{self, BufRead};"));
    }

    #[test]
    fn test_file_with_changes() {
        assert!(!is_add("+++ src/main.rs"));
        assert!(!is_del("+++ src/main.rs"));
    }
}
