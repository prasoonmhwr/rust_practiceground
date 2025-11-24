// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery{
    quantity: i32,
    id: i32,
}

fn display_quantity(groc: &Grocery){
    println!("Quantity: {:?}",groc.quantity)
}
fn display_id(groc: &Grocery){
    println!("Id: {:?}",groc.id)
}
fn main() {
    let grocery = Grocery {
        quantity: 10,
        id: 1,
    };
    display_quantity(&grocery);
    display_id(&grocery);

}
