use std::fs;
use std::io::{self, Read};


fn solve(input: &str) -> u64 {
    
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .filter(|row: &Vec<char>| !row.is_empty())
        .collect();

    if grid.is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();

    
    let start_col = grid[0].iter().position(|&c| c == 'S')
        .expect("Manifold must contain 'S' in the first row");

    
    let mut timelines: Vec<u64> = vec![0; cols];
    
    
    timelines[start_col] = 1;

    
    for r in 1..rows {
        
        let mut next_timelines: Vec<u64> = vec![0; cols];
        
        
        for c in 0..cols {
           
            let paths_from_above = timelines[c];

            if paths_from_above > 0 {
                match grid[r][c] {
                    
                    '.' => {
                        next_timelines[c] += paths_from_above;
                    }
                    
                    
                    '^' => {
                        
                        if c > 0 {
                            next_timelines[c - 1] += paths_from_above;
                        }

                       
                        if c + 1 < cols {
                            next_timelines[c + 1] += paths_from_above;
                        }
                    }
                    
                    _ => {} 
                }
            }
        }
        
        
        timelines = next_timelines;
        
        
        if timelines.iter().all(|&count| count == 0) {
            break;
        }
    }

    
    timelines.iter().sum()
}



fn main() -> io::Result<()> {
    const INPUT_FILENAME: &str = "input_puzzle7.txt";
    
    
    let file_content = match fs::read_to_string(INPUT_FILENAME) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", INPUT_FILENAME, e);
            eprintln!("Please ensure the file exists and is in the same directory as the executable.");
            return Err(e);
        }
    };
    
    
    let result = solve(&file_content);
    
    
    println!("--- Quantum Tachyon Manifold Analysis ---");
    println!("Total number of different timelines: {}", result);
    
    Ok(())
}