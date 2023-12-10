mod parser;

use std::{io};
use std::fs::File;
use std::io::{BufRead, Error};
use std::path::Path;
use std::str::FromStr;
use crate::day2::parser::Round;

pub fn run() -> Result<i16, Error> {
    let mut sum: i16 = 0;
    let limits: Round = Round { blues: 14, greens: 13, reds: 12 };

    if let Ok(lines) = read_lines("src/day2/input.txt") {
        for line in lines {
            let game = parser::parse(line?.as_str());
            if within_limits(&limits, &game.rounds) {
                sum += game.game_number;
            }
        }
        println!("Sum = {sum}");
    } else {
        println!("File could not be read.");
    }

    Ok(sum)
}

/*
fn parse_line_into_game(line: String) {
    let mut raw_game = line.split(":");
    let mut game = Game {
        game_number: FromStr::from_str(raw_game.next().unwrap().replace("Game ", "").as_str()).unwrap(),
        rounds: Vec::new()
    };

    let gameNumber = game.game_number;
    println!("{gameNumber}");

    let mut raw_rounds = raw_game.next().unwrap().split(";");
    for raw_round in raw_rounds {
        let mut round = Round::default();

        let mut color_expressions = raw_round.split(",");
        for color_exp in color_expressions {
            //let mut colorQuantity = colorExp.split(" ").next().unwrap();
            // println!("{colorQuantity}");
        }

        game.rounds.push(round);
    }
}
*/

fn within_limits(limits: &Round, test_rounds: &Vec<Round>) -> bool {
    for round in test_rounds {
        if limits.reds < round.reds || limits.greens < round.greens || limits.blues < round.blues {
            return false;
        }
    }
    true
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
        let limits: Round = Round { blues: 14, greens: 13, reds: 12 };

        let test1_possible = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let mut test = parser::parse(test1_possible);
        assert_eq!(true, within_limits(&limits, &test.rounds));

        let test2_possible = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        test = parser::parse(test2_possible);
        assert_eq!(true, within_limits(&limits, &test.rounds));

        let test3_impossible = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        test = parser::parse(test3_impossible);
        assert_eq!(false, within_limits(&limits, &test.rounds));

        let test4_impossible = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        test = parser::parse(test4_impossible);
        assert_eq!(false, within_limits(&limits, &test.rounds));

        let test5_possible = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        test = parser::parse(test5_possible);
        assert_eq!(true, within_limits(&limits, &test.rounds));
    }
}
