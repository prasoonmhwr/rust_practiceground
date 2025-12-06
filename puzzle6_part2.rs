use std::fs;

fn main() {
    
    let input = fs::read_to_string("input_puzzle6.txt")
        .expect("Failed to read input.txt");
    
    let lines: Vec<&str> = input.lines().collect();
    
    
    let mut operator_line_idx = lines.len() - 1;
    while operator_line_idx > 0 && lines[operator_line_idx].trim().is_empty() {
        operator_line_idx -= 1;
    }
    
    let operator_line = lines[operator_line_idx];
    let number_lines = &lines[0..operator_line_idx];
    
    // Determine the maximum width
    let width = number_lines.iter()
        .chain(std::iter::once(&operator_line))
        .map(|l| l.len())
        .max()
        .unwrap_or(0);
    
    
    let mut problems: Vec<(Vec<i64>, char)> = Vec::new();
    let mut current_numbers: Vec<i64> = Vec::new();
    let mut current_operator: Option<char> = None;
    
    for col in (0..width).rev() {
        
        let is_separator_in_numbers = number_lines.iter()
            .all(|line| col >= line.len() || line.chars().nth(col).unwrap_or(' ') == ' ');
        
        let operator_char = if col < operator_line.len() {
            operator_line.chars().nth(col).unwrap_or(' ')
        } else {
            ' '
        };
        
        let is_separator = is_separator_in_numbers && operator_char == ' ';
        
        if is_separator {
           
            if !current_numbers.is_empty() && current_operator.is_some() {
                problems.push((current_numbers.clone(), current_operator.unwrap()));
                current_numbers.clear();
                current_operator = None;
            }
            continue;
        }
        
        
        let mut digit_string = String::new();
        for line in number_lines {
            let ch = if col < line.len() {
                line.chars().nth(col).unwrap_or(' ')
            } else {
                ' '
            };
            
            if ch != ' ' && ch.is_ascii_digit() {
                digit_string.push(ch);
            }
        }
        
        
        if !digit_string.is_empty() {
            if let Ok(num) = digit_string.parse::<i64>() {
                current_numbers.push(num);
            }
        }
        
        
        if operator_char != ' ' && (operator_char == '+' || operator_char == '*') {
            current_operator = Some(operator_char);
        }
    }
    
    
    if !current_numbers.is_empty() && current_operator.is_some() {
        problems.push((current_numbers, current_operator.unwrap()));
    }
    
    
    let mut grand_total: i64 = 0;
    
    println!("Solving {} problems:", problems.len());
    for (i, (numbers, operator)) in problems.iter().enumerate() {
        let result = match operator {
            '+' => numbers.iter().sum::<i64>(),
            '*' => numbers.iter().product::<i64>(),
            _ => 0,
        };
        
        println!("Problem {}: {:?} {} = {}", i + 1, numbers, operator, result);
        grand_total += result;
    }
    
    println!("\nGrand Total: {}", grand_total);
}