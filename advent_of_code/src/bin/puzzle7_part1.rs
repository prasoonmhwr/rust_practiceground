use std::collections::HashSet;
use std::fs;
use std::io::{self, Read};

fn solve(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    if grid.is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let start_col = grid[0].iter().position(|&c| c == 'S')
        .expect("Manifold must contain 'S' in the first row");

    let mut active_cols: HashSet<usize> = HashSet::new();
    if rows > 1 {
        active_cols.insert(start_col);
    }
    
    let mut split_count = 0;

    for r in 1..rows {
        let mut next_active_cols: HashSet<usize> = HashSet::new();
        
        
        for &c in active_cols.iter() {
            match grid[r][c] {
                
                '.' => {
                    if r + 1 < rows {
                        next_active_cols.insert(c);
                    }
                }
                
                
                '^' => {
                    split_count += 1;

                    
                    if c > 0 {
                        if r + 1 < rows {
                            next_active_cols.insert(c - 1);
                        }
                    }

                    
                    if c + 1 < cols {
                        if r + 1 < rows {
                            next_active_cols.insert(c + 1);
                        }
                    }
                }
                
                _ => {} 
            }
        }
        
        
        active_cols = next_active_cols;
        
        
        if active_cols.is_empty() {
            break;
        }
    }

    split_count
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
    
    
    println!("--- Tachyon Manifold Analysis ---");
    println!("Total number of times the beam was split: {}", result);
    
    Ok(())
}