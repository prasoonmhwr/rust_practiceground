// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
enum Colors{
    Red,
    Blue,
    Green,
    Yellow
}
fn print_color(colors:Colors){
    match colors{
        Colors::Red => println!("red"),
        Colors::Blue => println!("blue"),
        Colors::Green => println!("green"),
        Colors::Yellow => println!("yellow")

    }
}
fn main() {
    print_color(Colors::Red)
}
