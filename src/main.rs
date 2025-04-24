
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

    // OWNERSHIP
    //     Ownership Rules
    // First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

    // Each value in Rust has an owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.

    let s = "hello";
    {                      // s is not valid here, it’s not yet declared
        let s = "hello world";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

    println!("{}", s);

    
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // This will print `hello, world!`



    let x = 5;
    let y = x;
    println!("{x}");
    println!("{y}");

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s1}");
    println!("{s2}");


}
