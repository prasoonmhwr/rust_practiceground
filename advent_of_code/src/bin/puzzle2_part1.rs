use std::collections::HashSet;
use std::fs;
use std::io::{self, Read}; 


type ID = u64;

fn read_file_to_string(path: &str) -> io::Result<String> {
    
    fs::read_to_string(path)
}


fn parse_range_string(range_str: &str) -> Option<(ID, ID)> {
    let mut parts = range_str.split('-');
    let start = parts.next()?.parse::<ID>().ok();
    let end = parts.next()?.parse::<ID>().ok();
    match (start, end) {
        (Some(s), Some(e)) => Some((s, e)),
        _ => None,
    }
}


fn precompute_invalid_ids() -> HashSet<ID> {
    let mut invalid_ids = HashSet::new();
    
    for digits in 1..=5 {
        let max_n = 10u64.pow(digits) - 1;
        let multiplier = 10u64.pow(digits);
        for n in 10u64.pow(digits - 1)..=max_n {
            let id = n * multiplier + n;
            invalid_ids.insert(id);
        }
    }
    
    for n in 1..=9 {
        invalid_ids.insert(n * 10 + n);
    }
    invalid_ids
}

fn main() {
    let path = "input_puzzle2.txt";
    let mut total_sum: ID = 0;

    let content = match read_file_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

   
    let invalid_id_set = precompute_invalid_ids();

   
    for range_str in content.trim().split(',') {
       
        if let Some((start, end)) = parse_range_string(range_str) {
           
            for id in start..=end {
               
                if invalid_id_set.contains(&id) {
                    total_sum += id;
                }
            }
        }
    }

    println!("The total sum of invalid ID parts is: {}", total_sum);
}