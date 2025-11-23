// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
enum DrinkFlavor{
    Orange,
    Strawberry,
    Grape
}
struct Drink{
    flavor: DrinkFlavor,
    ounce: f64,
}
fn print_drink(drink: Drink){
    match drink.flavor {
        DrinkFlavor::Orange => println!("orange"),
        DrinkFlavor::Strawberry => println!("strawberry"),
        DrinkFlavor::Grape => println!("grape"),
    }

    println!("oz: {:?}", drink.ounce);
}
fn main() {
 
    let orange_drink = Drink {
        flavor: DrinkFlavor::Orange,
        ounce: 12.3
    };

    print_drink(orange_drink);
}
