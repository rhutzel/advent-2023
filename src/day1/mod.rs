use std::{io};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

pub fn run() {
    let mut sum: u32 = 0;

    if let Ok(lines) = read_lines("src/day1/input.txt") {
        for line in lines {
            sum += first_and_last_digits_combined(line.unwrap().as_str());
        }
    }

    println!("Sum = {sum}");
}

fn read_lines<A>(filename: A) -> io::Result<io::Lines<io::BufReader<File>>> where A: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn first_and_last_digits_combined(line: &str) -> u32 {
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;

    for c in line.chars().into_iter() {
        match c.to_digit(10) {
            Some(d) => {
                first_digit = Some(d);
                break;
            },
            None => {}
        }
    }

    for c in line.chars().into_iter().rev() {
        match c.to_digit(10) {
            Some(d) => {
                last_digit = Some(d);
                break;
            },
            None => {}
        }
    }

    if first_digit.is_some() && last_digit.is_some() {
        (first_digit.unwrap() * 10) + last_digit.unwrap()
    } else {
        0
    }
}
