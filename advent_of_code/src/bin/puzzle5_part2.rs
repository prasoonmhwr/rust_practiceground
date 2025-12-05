use std::fs;
use std::io;


type Range = (i64, i64);

fn read_file_to_string(path: &str) -> io::Result<String> {
     match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => Err(e),
    }
}

fn parse_ranges(range_strs: &Vec<String>) -> Vec<Range> {
    let mut ranges: Vec<Range> = Vec::new();
    for range_str in range_strs {
        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() == 2 {
            if let (Ok(start), Ok(end)) = (parts[0].parse::<i64>(), parts[1].parse::<i64>()) {
                ranges.push((start, end));
            }
        }
    }
    ranges
}

fn count_unique_fresh_ids_optimized(range_strs: &Vec<String>) -> i64 {
    let mut ranges = parse_ranges(range_strs);

    
    ranges.sort_unstable_by_key(|r| r.0); 

    if ranges.is_empty() {
        return 0;
    }

    
    let mut merged: Vec<Range> = Vec::new();
    merged.push(ranges[0]);

    
    for i in 1..ranges.len() {
        let current_range = ranges[i];
        
        let last_merged = merged.last_mut().unwrap(); 

        
        if current_range.0 <= last_merged.1 + 1 {
          
            last_merged.1 = last_merged.1.max(current_range.1);
        } else {
           
            merged.push(current_range);
        }
    }

   
    let total_fresh_ids: i64 = merged.iter().map(|&(start, end)| {
        end - start + 1
    }).sum();

    total_fresh_ids
}

fn main(){
    let path = "input_puzzle5.txt";
    let mut ranges: Vec<String> = vec![];

    match read_file_to_string(path) {
        Ok(content) => {
           let parts: Vec<&str> = content.split("\n\n").collect();
           if !parts.is_empty() {
               ranges = parts[0]
                    .lines() 
                    .map(|s| s.to_string()) 
                    .collect();
           } else {
               eprintln!("File content is empty or malformed.");
               return;
           }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    }
    
    let total_fresh_ids = count_unique_fresh_ids_optimized(&ranges);
    println!("Total unique fresh ingredient IDs (Optimized): {}", total_fresh_ids);
}