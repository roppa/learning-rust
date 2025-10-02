fn main() {
    //exclusive range
    for i in 0..10 { // 0 to 9
        println!("i is {}", i);
    }
    // inclusive range
    for i in 1..=10 { // 1 to 10
        println!("i is {}", i);
    }
}
