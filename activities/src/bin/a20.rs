// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)
use std::io;
enum State{
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}
impl State{
    fn new(state: &str) -> Option<State>{
        let state = state.trim().to_lowercase();
        match state.as_str(){
            "off" => Some(State::Off),
            "sleep" => Some(State::Sleep),
            "reboot" => Some(State::Reboot),
            "shutdown" => Some(State::Shutdown),
            "hibernate" => Some(State::Hibernate),
            _ => None
        }
    }
}
fn take_action(state: State){
    use State::*;
    match state{
        Off => println!("turning off"),
        Sleep => println!("sleeping"),
        Reboot => println!("rebooting"),
        Shutdown => println!("shutting down"),
        Hibernate => println!("hibernating"),
    }
}
fn main() {
    let mut buffer = String::new();
    let user_input_status = io::stdin().read_line(&mut buffer);
    if user_input_status.is_ok(){
        match State::new(&buffer){
            Some(state) => take_action(state),
            None => println!("invalid Power state"),
        }
    } else {
        println!("error reading input");
    }
}
