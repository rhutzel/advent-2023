use pest::Parser;
use pest_derive::Parser;

pub struct Card {
    pub played_numbers: Vec<i16>,
    pub winning_numbers: Vec<i16>
}

#[derive(Parser)]
#[grammar = "src/day4/cards.pest"]
pub struct CardsParser;

pub fn parse(line: &str) -> Card {
    let result = CardsParser::parse(Rule::card, line)
        .expect("unsuccessful parse")
        .next().unwrap();
    let mut played_numbers = Vec::new();
    let mut winning_numbers = Vec::new();

    for card_inner in result.into_inner() {
        if card_inner.as_rule() == Rule::played_number {
            played_numbers.push(card_inner.as_str().trim().parse::<i16>().unwrap());
        } else if card_inner.as_rule() == Rule::winning_number {
            winning_numbers.push(card_inner.as_str().trim().parse::<i16>().unwrap());
        }
    }

    Card { played_numbers, winning_numbers }
}
