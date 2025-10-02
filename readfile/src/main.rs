use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("./hello.txt").expect("Failed to read file");
    println!("{}", contents);
}
