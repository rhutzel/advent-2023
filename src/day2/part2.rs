use std::cmp;
use std::{io};
use std::fs::File;
use std::io::{BufRead, Error};
use std::path::Path;
use std::str::FromStr;
use super::parser;
use super::parser::Round;

pub fn run() -> Result<i32, Error> {
    let mut sum: i32 = 0;

    if let Ok(lines) = read_lines("src/day2/input.txt") {
        for line in lines {
            let game = parser::parse(line?.as_str());
            sum += squared_maxs(&game.rounds);
        }
        println!("Sum = {sum}");
    } else {
        println!("File could not be read.");
    }

    Ok(sum)
}

fn squared_maxs(rounds: &Vec<Round>) -> i32 {
    let mut blues: i32 = 0;
    let mut greens: i32 = 0;
    let mut reds: i32 = 0;

    for round in rounds {
        blues = cmp::max(blues, round.blues);
        greens = cmp::max(greens, round.greens);
        reds = cmp::max(reds, round.reds);
    }

    blues * greens * reds
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

        let mut game = parser::parse("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
        assert_eq!(630, squared_maxs(&game.rounds));
    }
}
