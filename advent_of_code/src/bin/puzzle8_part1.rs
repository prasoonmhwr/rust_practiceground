use std::collections::HashMap;
use std::fs;
use std::path::Path;


struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Dsu {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    
    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        self.parent[i] = self.find(self.parent[i]);
        self.parent[i]
    }

    
    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
            return true;
        }
        false
    }
}

// --- Point and Distance Structures ---
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct Connection {
    p1_index: usize,
    p2_index: usize,
    sq_dist: u64,
}

impl Ord for Connection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sq_dist.cmp(&other.sq_dist)
    }
}

impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.sq_dist == other.sq_dist
    }
}

impl Eq for Connection {}

fn squared_distance(p1: &Point, p2: &Point) -> u64 {
    let dx = (p1.x - p2.x) as i64;
    let dy = (p1.y - p2.y) as i64;
    let dz = (p1.z - p2.z) as i64;

    (dx * dx + dy * dy + dz * dz) as u64
}



pub fn solve(input: &str) -> u64 {
   
    let points: Vec<Point> = input
        .trim()
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 3 {
                let x = parts[0].parse::<i64>().ok()?;
                let y = parts[1].parse::<i64>().ok()?;
                let z = parts[2].parse::<i64>().ok()?;
                Some(Point { x, y, z })
            } else {
                None
            }
        })
        .collect();

    let num_points = points.len();
    if num_points < 3 {
        return 0;
    }

    
    let mut connections: Vec<Connection> = Vec::new();

    for i in 0..num_points {
        for j in (i + 1)..num_points {
            let sq_dist = squared_distance(&points[i], &points[j]);
            connections.push(Connection {
                p1_index: i,
                p2_index: j,
                sq_dist,
            });
        }
    }

    connections.sort_unstable();

    
    let num_connections_to_make = 1000;
    let mut dsu = Dsu::new(num_points);
    let connections_to_process = connections.iter().take(num_connections_to_make);

    for conn in connections_to_process {
        dsu.union(conn.p1_index, conn.p2_index);
    }

    
    let mut circuit_sizes: HashMap<usize, u64> = HashMap::new();

    for i in 0..num_points {
        let root = dsu.find(i);
        
        circuit_sizes.insert(root, dsu.size[root] as u64);
    }
    
    let mut sizes: Vec<u64> = circuit_sizes.values().cloned().collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    
   
    let s1 = *sizes.get(0).unwrap_or(&0);
    let s2 = *sizes.get(1).unwrap_or(&0);
    let s3 = *sizes.get(2).unwrap_or(&0);

    s1 * s2 * s3
}



fn main() -> Result<(), std::io::Error> {
    let input_path = Path::new("input_puzzle8.txt");
    
    println!("🔌 Reading input from: {}", input_path.display());

    
    let input = fs::read_to_string(input_path)?;

    
    let result = solve(&input);

    println!("---");
    println!("✅ Product of the three largest circuit sizes after 1000 connections: {}", result);
    Ok(())
   
}