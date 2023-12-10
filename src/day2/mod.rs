mod parser;
mod part1;
mod part2;

use std::io::{BufRead, Error};
use std::str::FromStr;

pub fn run() -> Result<i32, Error> {
    // Ok(i32::from(part1::run().unwrap()))
    part2::run()
}
