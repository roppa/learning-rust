struct User {
    name: String,
    email: String,
    age: i32
}

fn main() {
    let bob = User{
        name: "Bob".to_string(),
        email: "bob@bobbity.com".to_string(),
        age: 32
    };

    println!("Bob is: {}, {}, {}", bob.name, bob.email, bob.age);
}
