struct User {
    name: String,
    email: String,
    age: i32
}

struct Instructor {
    name: String,
    email: String,
    courses: Vec<String>,
    age: i32
}

pub trait InstructorCourses {
    fn add_course(&mut self, course: String);
    fn remove_course(&mut self, course: String);
}

impl InstructorCourses for Instructor {
    fn add_course(&mut self, course: String) {
        self.courses.push(course);
    }

    fn remove_course(&mut self, course: String) {
        self.courses.retain(|c| c != &course);
    }
}

fn main() {
    let bob = User{
        name: "Bob".to_string(),
        email: "bob@bobbity.com".to_string(),
        age: 32
    };

    println!("Bob is: {}, {}, {}", bob.name, bob.email, bob.age);
}
