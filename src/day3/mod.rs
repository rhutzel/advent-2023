mod part1;

pub fn run() {
    match part1::run() {
        Ok(sum) => println!("Sum = {sum}"),
        Err(_) => println!("Error!")
    }
}
