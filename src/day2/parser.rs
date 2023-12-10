use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

pub struct Game {
    pub game_number: i16,
    pub rounds: Vec<Round>
}

#[derive(Debug)]
pub struct Round {
    pub reds: i8,
    pub greens: i8,
    pub blues: i8
}

#[derive(Parser)]
#[grammar = "src/day2/games.pest"]
pub struct GamesParser;

impl Default for Round {
    fn default() -> Self {
        Round {
            reds: 0,
            greens: 0,
            blues: 0
        }
    }
}

pub fn parse(line: &str) -> Game {
    let result = GamesParser::parse(Rule::game, line)
        .expect("unsuccessful parse")
        .next().unwrap();
    let mut game_number: Option<i16> = None;
    let mut rounds = Vec::new();

    for game_inner in result.into_inner() {
        if game_inner.as_rule() == Rule::game_title {
            game_number = Some(game_inner.into_inner().next().unwrap().as_str().parse::<i16>().unwrap());
        } else if game_inner.as_rule() == Rule::round {
            let round = process_round(game_inner);
            // println!("{:?}", round);
            rounds.push(round);
        }
    }

    Game {
        game_number: game_number.unwrap(),
        rounds: rounds
    }
}

fn process_round(pair: Pair<Rule>) -> Round {
    let mut round = Round::default();

    for color_count in pair.into_inner() {
        let color = color_count.as_str();
        let count = color_count.into_inner().next().unwrap()
            .as_str().trim().parse::<i8>().unwrap();
        if color.contains("blue") {
            round.blues += count;
        } else if color.contains("green") {
            round.greens += count;
        } else if color.contains("red") {
            round.reds += count;
        }
    }

    round
}
