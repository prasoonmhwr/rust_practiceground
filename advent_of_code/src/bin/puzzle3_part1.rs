use std::fs;
use std::io;

fn read_file_to_string(path: &str) -> io::Result<String> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => Err(e),
    }
}

fn what_to_turn_on(battery: &str) -> i32 {
    let mut to_turn_on = vec![];
    let mut max_so_far = 1;
    let mut first_index = 0;
    for (index, ch) in battery.chars().enumerate() {
        if ch.to_digit(10).unwrap() as i32 > max_so_far {
            if index != battery.len() - 1 {
                max_so_far = ch.to_digit(10).unwrap() as i32;
                first_index = index + 1;
            }
        }
    }
    to_turn_on.push(max_so_far.to_string());
    max_so_far = 1;
    for i in first_index..battery.len() {
        let ch = battery.chars().nth(i).unwrap();
        if ch.to_digit(10).unwrap() as i32 >= max_so_far {
            max_so_far = ch.to_digit(10).unwrap() as i32;
        }
    }
    to_turn_on.push(max_so_far.to_string());
    let f = ("".to_owned() + &to_turn_on[0] + &to_turn_on[1]).parse::<i32>().unwrap();
    println!("For battery {}, turn on values: {:?}", battery, f);
    f
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
        total += what_to_turn_on(&battery);
    }
    println!("Total to turn on: {}", total);
}