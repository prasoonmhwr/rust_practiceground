// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
enum Colors{
    Green,
    Blue,
    Red
}
impl Colors{
    fn print(&self){
        match self{
            Colors::Green => println!("green"),
            Colors::Blue => println!("blue"),
            Colors::Red => println!("red"),

        }
    }
}
struct Dimensions{
    width: f64,
    height: f64,
    depth: f64,
}
impl Dimensions{
    fn print(&self){
        println!("width {:?}",self.width);
        println!("height {:?}",self.height);
        println!("depth {:?}",self.depth);

    }
}
struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: Colors
}
impl Box{
    fn new(weight:f64,color:Colors,dimensions:Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions
        }
    }

    fn print(&self){
        self.color.print();
        self.dimensions.print();
        println!("weight {:?}", self.weight);
    }
}
fn main() {
    let dimension_1 = Dimensions {
        width: 1.0,
        height:2.0,
        depth: 4.0,
    };

    let boxx = Box::new(5.0,Colors::Red,dimension_1);
    boxx.print();
}
