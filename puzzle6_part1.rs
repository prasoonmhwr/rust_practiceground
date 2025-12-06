use std::fs::File;
use std::io::{self, BufReader, BufRead};


fn main() -> io::Result<()> {
    
    let file = File::open("input_puzzle6.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();

    
    let grand_total = solve(&lines);

    println!("🐙 The Grand Total of all problem answers is: **{}**", grand_total);
    
    Ok(())
}


fn solve(lines: &[String]) -> u64 {
    if lines.is_empty() {
        return 0;
    }

    
    let max_len = lines.iter().map(|s| s.len()).max().unwrap_or(0);
    if max_len == 0 {
        return 0;
    }

    
    let operator_line = lines.last().unwrap();
    let operator_indices: Vec<usize> = operator_line.char_indices()
        .filter_map(|(i, c)| match c {
            '+' | '*' => Some(i),
            _ => None,
        })
        .collect();

    
    let mut problem_boundaries = Vec::new();
    let mut current_start = 0;

    for &op_index in &operator_indices {
        
        let mut left_boundary = op_index;
        while left_boundary > current_start && !is_all_spaces(&lines, left_boundary - 1) {
            left_boundary -= 1;
        }
        
        
        let mut right_boundary = op_index + 1;
        while right_boundary < max_len && !is_all_spaces(&lines, right_boundary) {
            right_boundary += 1;
        }

        
        problem_boundaries.push((left_boundary, right_boundary, op_index));
        
        
        current_start = right_boundary;
    }

    let mut grand_total: u64 = 0;
    
    for (start, end, op_index) in problem_boundaries {
        let mut numbers = Vec::new();
        
        for line in lines.iter().take(lines.len() - 1) {
            let problem_slice = &line.as_str()[start..end].trim();
            
            if let Ok(num) = problem_slice.parse::<u64>() {
                numbers.push(num);
            }
        }
        
        let operator = operator_line.chars().nth(op_index).unwrap();

        if !numbers.is_empty() {
            let answer = calculate_problem_answer(&numbers, operator);
            grand_total += answer;
        }
    }

    grand_total
}

fn is_all_spaces(lines: &[String], col_index: usize) -> bool {
    for line in lines.iter().take(lines.len() - 1) {
        if col_index < line.len() {
            if line.chars().nth(col_index).unwrap() != ' ' {
                return false;
            }
        }
    }
    true
}

fn calculate_problem_answer(numbers: &[u64], operator: char) -> u64 {
    match operator {
        '+' => numbers.iter().sum(),
        '*' => numbers.iter().product(),
        _ => panic!("Unknown operator: {}", operator), // Should not happen with valid input
    }
}