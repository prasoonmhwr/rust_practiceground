use std::io;
use std::fs;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

fn read_to_string(path: &str) -> io::Result<String> {
    match fs::read_to_string(path){
        Ok(content) => Ok(content),
        Err(e) => Err(e),
    }
}

fn main(){
    let path = "input_puzzle9.txt";
    let mut coords = vec![];
    let mut largest_area = 0;
    match read_to_string(path) {
        Ok(content) => {
            for line in content.lines() {
                coords.push(line.to_string());
            }
            println!("Read {} lines from {}", coords.len(), path);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    let points: Vec<Point> = coords
        .iter()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 2 {
                let x = parts[0].trim().parse::<i64>().ok()?;
                let y = parts[1].trim().parse::<i64>().ok()?;
                Some(Point { x, y })
            } else {
                None
            }
        })
        .collect();
    
   for p1 in &points {
        for p2 in &points {
            let width = (p1.x - p2.x).abs() + 1;
            let height = (p1.y - p2.y).abs() + 1;
            
            let area = width * height; 

            if area > largest_area {
                largest_area = area;
            }
           
        }
    }

    
    println!("Largest Area: {}", largest_area);
}