

const MY_AGE: u8 = 22;

fn main() {

    // VARIABLES
    let name: &str = "John";
    println!("Your name is: {}", name);

    let distance1: f64 = 5.5;
    let total_distance: f64 = distance1 * 2.0;
    println!("{total_distance}");
    println!("{MY_AGE}");

    let personal_data: (i32, &str) = (22, "John");
    let (age: i32, name: &str) = personal_data;

    



}
