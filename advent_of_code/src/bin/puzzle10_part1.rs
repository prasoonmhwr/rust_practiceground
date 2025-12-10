use std::collections::BTreeSet;
use std::fs;
use std::env;


fn gaussian_elimination_f2(matrix: &mut Vec<Vec<u8>>) -> usize {
    let rows = matrix.len();
    if rows == 0 { return 0; }
    let cols = matrix[0].len();

    let mut pivot_row = 0;
    let mut rank = 0;

    for j in 0..cols - 1 { 
        if pivot_row == rows { break; }

        let mut i = pivot_row;
        while i < rows && matrix[i][j] == 0 {
            i += 1;
        }

        if i < rows {
            matrix.swap(i, pivot_row);

            for k in 0..rows {
                if k != pivot_row && matrix[k][j] == 1 {
                  
                    for l in j..cols {
                        matrix[k][l] ^= matrix[pivot_row][l];
                    }
                }
            }

            pivot_row += 1;
            rank += 1;
        }
    }
    rank
}


fn solve_min_presses(mut matrix: Vec<Vec<u8>>, num_buttons: usize) -> Option<usize> {
    let num_lights = matrix.len();
    if num_lights == 0 { return Some(0); }
    let num_cols = matrix[0].len(); 

    let rank = gaussian_elimination_f2(&mut matrix);

    
    for i in 0..num_lights {
        let is_all_zero = matrix[i][0..num_buttons].iter().all(|&x| x == 0);
        if is_all_zero && matrix[i][num_buttons] == 1 {
            return None; 
        }
    }

    
    let mut pivot_cols: BTreeSet<usize> = BTreeSet::new();
    let mut free_cols: BTreeSet<usize> = (0..num_buttons).collect();

    let mut current_pivot_row = 0;
    for j in 0..num_buttons {
        if current_pivot_row < num_lights && matrix[current_pivot_row][j] == 1 {
            pivot_cols.insert(j);
            free_cols.remove(&j);
            current_pivot_row += 1;
        }
    }

    let num_free = free_cols.len();
    let free_cols_vec: Vec<usize> = free_cols.into_iter().collect();

   
    let mut min_presses = std::usize::MAX;
    
    
    for i in 0..1 << num_free {
        let mut x = vec![0u8; num_buttons];
        let mut current_presses = 0;

        
        for k in 0..num_free {
            let col = free_cols_vec[k];
            let value = ((i >> k) & 1) as u8;
            x[col] = value;
            if value == 1 {
                current_presses += 1;
            }
        }

        
        for r in (0..rank).rev() {
            let j = *pivot_cols.iter().nth(r).unwrap();

            let mut val = matrix[r][num_buttons]; // b'[r]

            for l in j + 1..num_buttons {
                val ^= matrix[r][l] * x[l]; 
            }

            x[j] = val;
            if val == 1 {
                current_presses += 1;
            }
        }

        min_presses = min_presses.min(current_presses);
    }

   
    if min_presses == std::usize::MAX {
       
        None 
    } else {
        Some(min_presses)
    }
}



fn parse_input(input: &str) -> Vec<(Vec<Vec<u8>>, usize)> {
    let mut machines = Vec::new();

    for line in input.trim().lines() {
        
        let parts: Vec<&str> = line.split('{').collect();
        let machine_data = parts[0].trim();

        
        let start_bracket = machine_data.find('[').unwrap() + 1;
        let end_bracket = machine_data.find(']').unwrap();
        let light_diagram = &machine_data[start_bracket..end_bracket];
        let num_lights = light_diagram.len();
        let b: Vec<u8> = light_diagram.chars().map(|c| if c == '#' { 1 } else { 0 }).collect();

        
        let button_schematics_str = machine_data[end_bracket + 1..].trim();
        
        let schematics: Vec<&str> = button_schematics_str.split(')')
            .filter_map(|s| {
                let s = s.trim();
                if s.starts_with('(') { Some(&s[1..]) } else { None }
            })
            .collect();
        
        let num_buttons = schematics.len();

        let mut a: Vec<Vec<u8>> = vec![vec![0; num_buttons]; num_lights];

        for (j, indices_str) in schematics.iter().enumerate() {
           
            let indices: Vec<usize> = indices_str.split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();

            for i in indices {
                if i < num_lights {
                    a[i][j] = 1;
                }
            }
        }

       
        let mut augmented_matrix = Vec::new();
        for i in 0..num_lights {
            let mut row = a[i].clone();
            row.push(b[i]);
            augmented_matrix.push(row);
        }

        machines.push((augmented_matrix, num_buttons));
    }

    machines
}


pub fn solve_puzzle(input: &str) -> Option<usize> {
    let machines_data = parse_input(input);
    let mut total_min_presses = 0;

    for (matrix, num_buttons) in machines_data {
        match solve_min_presses(matrix, num_buttons) {
            Some(min_presses) => {
                total_min_presses += min_presses;
            }
            None => {
                eprintln!("Error: Found an unsolvable machine!");
                return None;
            }
        }
    }

    Some(total_min_presses)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).unwrap_or(&String::from("input_puzzle10.txt")).clone();

    println!("Reading input from: {}", file_path);
    
   
    let contents = fs::read_to_string(file_path)?;

    match solve_puzzle(&contents) {
        Some(result) => {
            println!("The fewest total button presses required is: **{}**", result);
        }
        None => {
            eprintln!("Error");
        }
    }

    Ok(())
}