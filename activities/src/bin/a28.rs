// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing
#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

struct Shoes(Color);
impl Shoes {
    fn new(color: Color) -> Self {
        Self(color)
    }
}
struct Shirt(Color);
impl Shirt {
    fn new(color: Color) -> Self {
        Self(color)
    }
}
struct Pants(Color);
impl Pants {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

fn display_shoes(shoes: &Shoes) {
    match &shoes.0 {
        Color::Custom(c) => println!("Shoes color: {}", c),
        other => println!("Shoes color: {:?}", other),
    }
}
fn display_shirt(shirt: &Shirt) {
    match &shirt.0 {
        Color::Custom(c) => println!("Shirt color: {}", c),
        other => println!("Shirt color: {:?}", other),
    }
}
fn display_pants(pants: &Pants) {
    match &pants.0 {
        Color::Custom(c) => println!("Pants color: {}", c),
        other => println!("Pants color: {:?}", other),
    }
}

fn main() {
    let shoes = Shoes::new(Color::Red);
    let shirt = Shirt::new(Color::Custom("Cyan".to_string()));
    let pants = Pants::new(Color::Blue);

    display_shoes(&shoes);
    display_shirt(&shirt);
    display_pants(&pants);
}
