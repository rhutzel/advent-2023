use std::{io};
use std::fs::File;
use std::io::{BufRead, Error};
use std::path::Path;
use super::parser;
use super::parser::Card;

pub fn run() -> Result<i32, Error> {
    let mut sum: i32 = 0;

    if let Ok(lines) = read_lines("src/day4/input.txt") {
        for line in lines {
            let card = parser::parse(line?.as_str());
            sum += calculate_card_value(&card);
        }
        println!("Sum = {sum}");
    } else {
        println!("File could not be read.");
    }

    Ok(sum)
}

fn calculate_card_value(card: &Card) -> i32 {
    let count = card.played_numbers.iter()
        .filter(|num| card.winning_numbers.contains(num)).count();
    if count > 0 {
        i32::pow(2, card.played_numbers.iter()
            .filter(|num| card.winning_numbers.contains(num))
            .count()
            .saturating_sub(1) as u32)
    } else {
        0
    }
}

fn read_lines<A>(filename: A) -> io::Result<io::Lines<io::BufReader<File>>> where A: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_eval() {
        let test1 = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let test2 = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let test3 = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let test4 = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        let test5 = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
        let test6 = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let mut card = parser::parse(test1);
        assert_eq!(8, calculate_card_value(&card));

        card = parser::parse(test2);
        assert_eq!(2, calculate_card_value(&card));

        card = parser::parse(test3);
        assert_eq!(2, calculate_card_value(&card));

        card = parser::parse(test4);
        assert_eq!(1, calculate_card_value(&card));

        card = parser::parse(test5);
        assert_eq!(0, calculate_card_value(&card));

        card = parser::parse(test6);
        assert_eq!(0, calculate_card_value(&card));
    }
}
