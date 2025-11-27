// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.
use std::io;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Bill{
    name: String,
    amount: f64
}

struct Bills {
    inner: HashMap<String, Bill>
}

impl Bills {
    fn new() -> Self{
        Self {inner: HashMap::new()}
    }

    fn add(&mut self,bill: Bill){
        self.inner.insert(bill.name.clone(),bill);
    }

    fn get_all(&self) -> Vec<Bill> {
        let mut bills = vec![];
        for bill in self.inner.values(){
            bills.push(bill.clone());
        }
        bills
    }

    fn remove(&mut self, name: &str) -> bool{
        self.inner.remove(name).is_some()
    }

    fn update(&mut self, name: &str, amount: f64) -> bool {
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                true
            }
            None => false,
        }
    }
}
fn read_input() -> Option<String>{
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err(){
        println!("Please enter your data again")
    } 
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}
fn get_bill_amount() -> Option<f64>{
    println!("Amount:");
    loop {
        let input = match read_input(){
            Some(input) => input,
            None => return None,
        };
        if &input == "" {
            return None;
        }
        let parsed_input: Result<f64,_> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a number"),
        }
    }
}
fn add_bill_menu(bills: &mut Bills){
    let name = match read_input(){
        Some(input) => input,
        None => return,
    };
    let amount = match get_bill_amount(){
         Some(amount) => amount,
         None => return,
    };
    let bill = Bill {name, amount};
    bills.add(bill);
    println!("Bill added");
}
fn remove_bill_menu(bills: &mut Bills){
    for bill in bills.get_all(){
        println!("{:?}",bill);
    }
    let input = match read_input(){
        Some(input) => input,
        None => return,
    };
    if bills.remove(&input){
        println!("removed");
    }else {
        println!("bill not found");
    }
}
fn update_bill_menu(bills: &mut Bills){
    for bill in bills.get_all(){
        println!("{:?}",bill);
    }
    println!("Enter bill name to udpate:");
    let input = match read_input(){
        Some(input) => input,
        None => return,
    };
    let amount = match get_bill_amount(){
         Some(amount) => amount,
         None => return,
    };
    if bills.update(&input, amount){
        println!("updated");
    }else {
        println!("bill not found");
    }
}
fn view_bills_menu(bills: &Bills){
    for bill in bills.get_all(){
        println!("{:?}", bill);
    }
}
fn main_menu(){
    fn show() {
        println!("");
        println!("== Manage Bills ==");
        println!("1. Add bill");
        println!("2. View bills");
        println!("3. Remove bill");
        println!("4. Update bill");
        println!("");
        println!("Enter");
    }

    let mut bills = Bills::new();

    loop {
        show();
        let input = match read_input(){
        Some(input) => input,
        None => return,
    };
        match input.as_str(){
            "1" => add_bill_menu(&mut bills),
            "2" => view_bills_menu(&bills),
            "3" => remove_bill_menu(&mut bills),
            "4" => update_bill_menu(&mut bills),
            _ => break,
        }
    }
}
fn main() {
    main_menu();
}
