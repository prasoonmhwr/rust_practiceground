use std::fs;
use std::io;

fn read_file_to_string(path: &str) -> io::Result<String> {
     match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => Err(e),
    }
}

fn check_validity(ranges: &Vec<String>, food: &Vec<String>) {
    let mut total_valid = 0;

    for item in food {
        println!("Checking item: {}", item);
        let value: i64 = item.parse().unwrap();
        let mut valid = false;

        for range in ranges {
            let parts: Vec<&str> = range.split('-').collect();
            let start: i64 = parts[0].parse().unwrap();
            let end: i64 = parts[1].parse().unwrap();

            if value >= start && value <= end {
                valid = true;
                break;
            }
        }

        if valid {
            total_valid += 1;
        }
    }

    println!("{}",total_valid)
}
fn main(){
    let path = "input_puzzle5.txt";
    let mut ranges: Vec<_> = vec![];
    let mut healthyFood: Vec<_> = vec![];

    match read_file_to_string(path) {
        Ok(content) => {
           let parts: Vec<&str> = content.split("\n\n").collect();
           ranges = parts[0]
        .lines() 
        .map(|s| s.to_string()) 
        .collect();
        healthyFood = parts[1]
        .lines() 
        .map(|s| s.to_string()) 
        .collect();
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
    check_validity(&ranges, &healthyFood);

}