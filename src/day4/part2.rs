use std::{io};
use std::cmp;
use std::fs::File;
use std::io::{BufRead};
use std::path::Path;
use super::parser;
use super::parser::Card;

pub fn run() -> i32 {
    let mut winners_per_card: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("src/day4/input.txt") {
        for line in lines {
            let card = parser::parse(line.unwrap().as_str());
            winners_per_card.push(count_card_winners(&card));
        }
        let total_cards = calculate_total_cards(&winners_per_card);
        println!("Sum = {total_cards}");
        total_cards
    } else {
        panic!("File could not be read.");
    }
}

fn calculate_total_cards(winners_per_card: &Vec<i32>) -> i32 {
    let mut total_cards: i32 = 0;
    for card_idx in 0..winners_per_card.len() {
        total_cards += process_card(&winners_per_card, card_idx);
    }
    total_cards
}

fn process_card(winners_per_card: &Vec<i32>, card_idx: usize) -> i32 {
    let winner_count = winners_per_card[card_idx];
    let mut total_cards = 1;

    let max_idx: i32 = winners_per_card.len() as i32 - 1;
    if winner_count > 0 {
        let bonus_idx_start: i32 = card_idx as i32 + 1;
        let bonus_idx_end: i32 = cmp::min(card_idx as i32 + winner_count, max_idx);

        if bonus_idx_start <= max_idx && bonus_idx_start <= bonus_idx_end {
            for bonus_card in (bonus_idx_start)..=bonus_idx_end {
                total_cards += process_card(&winners_per_card, bonus_card as usize);
            }
        }
    }
    total_cards
}

fn count_card_winners(card: &Card) -> i32 {
    card.played_numbers.iter()
        .filter(|num| card.winning_numbers.contains(num)).count() as i32
}

fn read_lines<A>(filename: A) -> io::Result<io::Lines<io::BufReader<File>>> where A: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_card() {
        let test1 = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let test2 = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let test3 = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let test4 = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        let test5 = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
        let test6 = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let winners_per_card: Vec<i32> = vec![
            count_card_winners(&parser::parse(test1)),
            count_card_winners(&parser::parse(test2)),
            count_card_winners(&parser::parse(test3)),
            count_card_winners(&parser::parse(test4)),
            count_card_winners(&parser::parse(test5)),
            count_card_winners(&parser::parse(test6))
        ];

        assert_eq!(30, calculate_total_cards(&winners_per_card));
    }
}
