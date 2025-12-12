use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Shape {
    cells: Vec<(i32, i32)>,
    width: i32,
    height: i32,
}

impl Shape {
    fn from_lines(lines: &[&str]) -> Self {
        let mut cells = Vec::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    cells.push((x as i32, y as i32));
                }
            }
        }
        let s = Self { cells, width: 0, height: 0 };
        s.normalize()
    }

    fn rotate(&self) -> Self {
        let cells = self.cells.iter().map(|&(x, y)| (-y, x)).collect();
        Self { cells, width: 0, height: 0 }.normalize()
    }

    fn flip(&self) -> Self {
        let cells = self.cells.iter().map(|&(x, y)| (-x, y)).collect();
        Self { cells, width: 0, height: 0 }.normalize()
    }

    fn normalize(&self) -> Self {
        if self.cells.is_empty() {
            return self.clone();
        }
        let min_x = self.cells.iter().map(|&(x, _)| x).min().unwrap();
        let min_y = self.cells.iter().map(|&(_, y)| y).min().unwrap();
        let max_x = self.cells.iter().map(|&(x, _)| x).max().unwrap();
        let max_y = self.cells.iter().map(|&(_, y)| y).max().unwrap();
        
        let cells: Vec<_> = self.cells.iter()
            .map(|&(x, y)| (x - min_x, y - min_y))
            .collect();
        
        Self {
            cells,
            width: max_x - min_x + 1,
            height: max_y - min_y + 1,
        }
    }

    fn all_orientations(&self) -> Vec<Shape> {
        let mut seen = HashSet::new();
        let mut orientations = Vec::new();
        let mut current = self.clone();
        
        for _ in 0..2 {
            for _ in 0..4 {
                let key = format!("{:?}", current.cells);
                if seen.insert(key) {
                    orientations.push(current.clone());
                }
                current = current.rotate();
            }
            current = current.flip();
        }
        
        orientations
    }
}

struct Grid {
    width: usize,
    height: usize,
    occupied: Vec<Vec<bool>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            occupied: vec![vec![false; width]; height],
        }
    }

    fn can_place(&self, shape: &Shape, x: i32, y: i32) -> bool {
        for &(dx, dy) in &shape.cells {
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || ny < 0 || nx >= self.width as i32 || ny >= self.height as i32 {
                return false;
            }
            if self.occupied[ny as usize][nx as usize] {
                return false;
            }
        }
        true
    }

    fn place(&mut self, shape: &Shape, x: i32, y: i32) {
        for &(dx, dy) in &shape.cells {
            self.occupied[(y + dy) as usize][(x + dx) as usize] = true;
        }
    }

    fn remove(&mut self, shape: &Shape, x: i32, y: i32) {
        for &(dx, dy) in &shape.cells {
            self.occupied[(y + dy) as usize][(x + dx) as usize] = false;
        }
    }
}

fn solve(grid: &mut Grid, presents: &[Vec<Shape>], idx: usize) -> bool {
    if idx >= presents.len() {
        return true;
    }

    let orientations = &presents[idx];
    
   
    for y in 0..grid.height as i32 {
        for x in 0..grid.width as i32 {
           
            if grid.occupied[y as usize][x as usize] {
                continue;
            }
            
            for orientation in orientations {
                if grid.can_place(orientation, x, y) {
                    grid.place(orientation, x, y);
                    if solve(grid, presents, idx + 1) {
                        return true;
                    }
                    grid.remove(orientation, x, y);
                }
            }
        }
    }
    
    false
}

fn can_fit_presents(width: usize, height: usize, shapes: &[Shape], counts: &[usize]) -> bool {
   
    let total_cells_needed: usize = counts.iter().enumerate()
        .map(|(idx, &count)| count * shapes[idx].cells.len())
        .sum();
    
    if total_cells_needed > width * height {
        return false;
    }
    
    let mut presents = Vec::new();
    for (shape_idx, &count) in counts.iter().enumerate() {
        for _ in 0..count {
            presents.push(shapes[shape_idx].all_orientations());
        }
    }
    
  
    presents.sort_by_key(|orients| orients.len());
    
    let mut grid = Grid::new(width, height);
    solve(&mut grid, &presents, 0)
}

fn main() {
    let input = include_str!("input_puzzle12.txt");
    let lines: Vec<&str> = input.lines().collect();
    
    let mut shapes = Vec::new();
    let mut i = 0;
    
    // Parse shapes
    while i < lines.len() {
        let line = lines[i].trim();
        
        if line.contains(':') && !line.contains('x') {
            i += 1;
            let mut shape_lines = Vec::new();
            
            while i < lines.len() {
                let l = lines[i];
                if l.trim().is_empty() || l.contains('x') {
                    break;
                }
                shape_lines.push(l);
                i += 1;
            }
            
            if !shape_lines.is_empty() {
                shapes.push(Shape::from_lines(&shape_lines));
            }
        } else if line.contains('x') && line.contains(':') {
            break;
        } else {
            i += 1;
        }
    }
    
    println!("Parsed {} shapes", shapes.len());
    
    
    let mut count = 0;
    let mut total = 0;
    
    while i < lines.len() {
        let line = lines[i].trim();
        
        if line.is_empty() {
            i += 1;
            continue;
        }
        
        if line.contains('x') && line.contains(':') {
            let parts: Vec<&str> = line.split(':').collect();
            let dims: Vec<&str> = parts[0].trim().split('x').collect();
            
            let width: usize = dims[0].trim().parse().unwrap();
            let height: usize = dims[1].trim().parse().unwrap();
            
            let counts: Vec<usize> = parts[1]
                .trim()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            
            total += 1;
            
            if can_fit_presents(width, height, &shapes, &counts) {
                count += 1;
            }
            
            if total % 10 == 0 {
                println!("Checked {} regions, {} fit so far", total, count);
            }
        }
        
        i += 1;
    }
    
    println!("\nFinal answer: {} regions can fit all presents (out of {} total)", count, total);
}