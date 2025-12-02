use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
fn read_file_to_string(path: &str) -> io::Result<String> {
    
    fs::read_to_string(path)
}

fn is_invalid_id(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    for n_len in 1..len {
        if len % n_len == 0 {
            
            let pattern = &s[..n_len];
            
            
            let repetitions = len / n_len;

            
            let reconstructed: String = pattern.repeat(repetitions);

            if s == reconstructed {
                
                return true;
            }
        }
    }

    false
}

// Function to solve the puzzle
fn solve_puzzle(input: &str) -> u64 {
    println!("Input: {}", input);
    let mut invalid_ids: HashSet<u64> = HashSet::new();

    
    for range_str in input.split(',') {
        
        let parts: Vec<&str> = range_str.trim().split('-').collect();

        
        // if parts.len() == 2 {
            
            if let (Ok(start), Ok(end)) = (parts[0].parse::<u64>(), parts[1].parse::<u64>()) {
                
                for n in start..=end {
                    
                    if is_invalid_id(n) {
                        invalid_ids.insert(n);
                    }
                }
            }
        // }
    }

    
    invalid_ids.iter().sum()
}


fn main() {
    
    let path = "input_puzzle2.txt";
    let content = match read_file_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };
    let final_answer = solve_puzzle(content.trim());
    println!("The final answer (sum of invalid IDs) is: {}", final_answer);
}