fn main() {
    const UNCHANGING:u32 = 123;
    println!("Const value is {}", UNCHANGING);

    let int = 34;
    println!("{}", int);

    let int:u32 = 34;
    println!("Or, you could add the type {}", int);

    let mut change = 34;
    change += 1;
    println!("This value is mutable, therefore changed {}", change);

    let string = "Hello world!";
    println!("{}", string);

    for ch in string.chars() {
        println!("{}", ch);
    }

    for ch in string.bytes() {
        println!("{}", ch);
    }

    let mut string = String::from("Hello two!");
    string.push_str(" Hello three!");
    println!("{}, length {}", string, string.len());

}
