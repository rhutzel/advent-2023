use std::{io};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

pub fn run() {
    let mut sum: i32 = 0;

    if let Ok(lines) = read_lines("src/day1_part2/input.txt") {
        for line in lines {
            sum += first_and_last_digits_combined(line.unwrap().as_str());
        }
        println!("Sum = {sum}");
    } else {
        println!("File could not be read.");
    }
}

fn read_lines<A>(filename: A) -> io::Result<io::Lines<io::BufReader<File>>> where A: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn first_and_last_digits_combined(line: &str) -> i32 {
    let targets = [
        ("1", 1), ("one", 1), ("2", 2), ("two", 2), ("3", 3), ("three", 3),
        ("4", 4), ("four", 4), ("5", 5), ("five", 5), ("6", 6), ("six", 6),
        ("7", 7), ("seven", 7), ("8", 8), ("eight", 8), ("9", 9), ("nine", 9),
        ("10", 10), ("ten", 10)
    ];

    let mut first_digit: Option<i32> = None;
    let mut last_digit: Option<i32> = None;

    let mut min_idx: Option<usize> = None;
    let mut max_idx: Option<usize> = None;

    for target in targets {
        let idx = line.find(target.0);
        let ridx = line.rfind(target.0);

        if idx.is_some() && (min_idx.is_none() || min_idx.unwrap() > idx.unwrap()) {
            min_idx = idx;
            first_digit = Some(target.1);
        }
        if ridx.is_some() && (max_idx.is_none() || max_idx.unwrap() < ridx.unwrap()) {
            max_idx = ridx;
            last_digit = Some(target.1);
        }
    }

    if first_digit.is_some() && last_digit.is_some() {
        (first_digit.unwrap() * 10) + last_digit.unwrap()
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_eval() {
        let mut line = "eightwothree";
        assert_eq!(first_and_last_digits_combined(line), 83);
        line = "abcone2threexyz";
        assert_eq!(first_and_last_digits_combined(line), 13);
        line = "xtwone3four";
        assert_eq!(first_and_last_digits_combined(line), 24);
        line = "4nineeightseven2";
        assert_eq!(first_and_last_digits_combined(line), 42);
        line = "zoneight234";
        assert_eq!(first_and_last_digits_combined(line), 14);
        line = "7pqrstsixteen";
        assert_eq!(first_and_last_digits_combined(line), 76);
        line = "zlmlk1";
        assert_eq!(first_and_last_digits_combined(line), 0);
        line = "8jkfncbeight7seven8";
        assert_eq!(first_and_last_digits_combined(line), 88);
    }

}
