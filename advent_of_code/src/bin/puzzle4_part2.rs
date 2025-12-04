use std::fs;

fn neighbors_count(grid: &mut Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut accessible = 0;
    
   
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' {
                
                let mut neighbors = 0;
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 { continue; } 
                        let nr = r as i32 + dr;
                        let nc = c as i32 + dc;
                        if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                            if grid[nr as usize][nc as usize] == '@' {
                                neighbors += 1;
                            }
                        }
                    }
                }
                
                if neighbors < 4 {
                    accessible += 1;
                    grid[r][c] = '.';
                }
            }
        }
    }
    accessible
}
fn main() {
    let content = fs::read_to_string("input_puzzle4.txt").unwrap();
    let mut grid: Vec<Vec<char>> = content.lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut total_accessible = 0;
    let mut accessible = neighbors_count(&mut grid);
    while accessible > 0 {
        total_accessible += accessible;
        accessible = neighbors_count(&mut grid);
    }
    
    println!("Accessible rolls: {}", total_accessible);
}