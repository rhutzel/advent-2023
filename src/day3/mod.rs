mod part2;

pub fn run() {
    match part2::run() {
        Ok(sum) => println!("Sum = {sum}"),
        Err(_) => println!("Error!")
    }
}
