
const MY_AGE: u8 = 22;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,

}

fn main() {

    // VARIABLES
    let name: &str = "John";
    println!("Your name is: {}", name);

    let distance1: f64 = 5.5;
    let total_distance: f64 = distance1 * 2.0;
    println!("{total_distance}");
    println!("{MY_AGE}");

    let personal_data: (i32, &str) = (22, "John");
    // let (age: i32, name: &str) = personal_data;

    // STRUCTURES

    let person = Person {
        name: "John".to_string(),
        age: 30,
    };

    println!("{:?}", person);
    println!("My name is {} and age {}", person.name, person.age);

    // if Expressions
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }




}
