use std::fs;
use std::io;

fn read_file_to_string(path: &str) -> io::Result<String> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => Err(e),
    }
}

fn what_to_turn_on(battery: &str, num_digits_to_select: usize) -> u128 {
   let digits: Vec<char> = battery.chars().collect();
    let total_len = digits.len();

    let mut result = String::new();
    let mut current_start_index = 0;
    let mut digits_selected = 0;

    
    while digits_selected < num_digits_to_select {
       
        let digits_to_select_remaining = num_digits_to_select - digits_selected;
        
        
        let available_digits_remaining = total_len - current_start_index;

       
        let search_end_index = current_start_index + (available_digits_remaining - digits_to_select_remaining);

       
        let mut best_digit = '0';
        let mut best_index = current_start_index;

        for i in current_start_index..=search_end_index {
            if digits[i] > best_digit {
                best_digit = digits[i];
                best_index = i;
            }
        }

        
        result.push(best_digit);
        digits_selected += 1;

        
        current_start_index = best_index + 1;
    }

    
    result.parse::<u128>().unwrap()
}

fn main(){
    let path = "input_puzzle3.txt";
    let mut batteries = vec![];
    let mut total = 0;
    match read_file_to_string(path) {
        Ok(content) => {
            for line in content.lines() {
                batteries.push(line.to_string());
            }
            println!("Read {} lines from {}", batteries.len(), path);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
    for battery in batteries {
        total += what_to_turn_on(&battery,12);
    }
    println!("Total to turn on: {}", total);
}