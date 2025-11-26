// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student{
    name: String,
    locker: Option<i32>,
}
fn main() {
    let student = Student {
        name: "Kobe".to_owned(),
        locker: None,
    };

    println!("student: {:?}", student.name);
    match student.locker{
        Some(num) => println!("locker number: {:?}", num),
        None => println!("no locker assigned"),
    }
}
