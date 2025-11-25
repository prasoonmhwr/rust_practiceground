// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person{
    age: i32,
    name: String,
    color: String,
}
fn print_details(peeps: &Person){
    println!("Name {:?}",peeps.name);
    println!("Color {:?}", peeps.color);
}
fn main() {
    let people = vec![
        Person {
            age: 30,
            name: "Prasoon".to_owned(),
            color: String::from("Blue"),
        },
        Person {
            age: 9,
            name: "George".to_owned(),
            color: String::from("Red"),
        },
        Person {
            age: 20,
            name: "Pootnika".to_owned(),
            color: String::from("Pink"),
        },
    ];
    for peeps in &people{
        if peeps.age <= 10 {
            print_details(peeps)
        }
    }
}
