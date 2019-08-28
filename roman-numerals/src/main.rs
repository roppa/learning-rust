use std::env::args;
use roman_numerals::numeralise;

fn main() {
    let args: Vec<String> = args().skip(1).collect();
    println!("{:?}", numeralise(args[0].parse::<i32>().unwrap()));
}
