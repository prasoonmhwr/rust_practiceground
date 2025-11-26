// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock
use std::collections::HashMap;

struct Content{
    name: String,
    stock: i32,
}
fn main() {
    let mut inventory = HashMap::new();
    let mut total_count = 0;
    inventory.insert(1,Content{name: "Chairs".to_owned(),stock:5});
    inventory.insert(2,Content{name: "Beds".to_owned(),stock:3});
    inventory.insert(3,Content{name: "Tables".to_owned(),stock:2});
    inventory.insert(4,Content{name: "Couches".to_owned(),stock:0});

    for (_key,content) in inventory.iter(){
        total_count += content.stock;
        println!("Item: {}, Stock: {}",content.name,content.stock);
    }
    println!("Total Stock: {}", total_count);
}
