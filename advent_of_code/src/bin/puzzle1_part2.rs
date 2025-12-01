

use std::fs;
use std::io;

fn read_file_to_string(path: &str) -> io::Result<String> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => Err(e),
    }
}
fn main(){
    let path = "input.txt";
    let mut codes = vec![];
    let mut dial = 50;
    let mut total = 0;
    match read_file_to_string(path) {
        Ok(content) => {
            for line in content.lines() {
                codes.push(line.to_string());
            }
            println!("Read {} lines from {}", codes.len(), path);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
    

    for code in codes {
        let split_codes: Vec<&str> = code.split("").collect();
        let number_slice = &split_codes[2..split_codes.len() - 1]; 
    
    
    let joined_string: String = number_slice.join(""); 

    
    let final_number: i32 = joined_string
        .parse()
        .expect("Failed to parse joined string into i32");

    let distance_to_first_zero = match split_codes[1] {
            "R" => {
                100 - dial 
            },
            "L" => {
                if dial == 0 {
            100
        } else {
            dial
        }
            },
            _ => panic!("Invalid direction"),
        };
    
    if final_number >= distance_to_first_zero && distance_to_first_zero > 0 {
           
            total += 1; 

           
            let remaining_distance = final_number - distance_to_first_zero;
            
            
            let cycle_count = remaining_distance / 100;
            total += cycle_count;
        }
    let position_change = match split_codes[1] {
            "L" => -final_number, 
            "R" => final_number,  
            _ => panic!("Invalid direction: expected L or R"),
        };

       
    let mut next_position = dial + position_change;
        
        
        next_position = (next_position % 100 + 100) % 100;

        
        dial = next_position;

        
       
    }
    println!("Total 0: {}", total);
}