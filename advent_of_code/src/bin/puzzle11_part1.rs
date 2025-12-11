use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io;


type DeviceGraph = HashMap<String, Vec<String>>;


fn parse_input(input: &str) -> DeviceGraph {
    let mut graph = DeviceGraph::new();

    for line in input.lines() {
       
        if line.trim().is_empty() {
            continue;
        }
        
        
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() != 2 {
          
            continue; 
        }

        let device = parts[0].trim().to_string();
       
        let outputs: Vec<String> = parts[1]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        graph.insert(device, outputs);
    }
    graph
}


fn count_paths_recursive(
    graph: &DeviceGraph,
    current_device: &str,
    target_device: &str,
    memo: &mut HashMap<String, u64>,
) -> u64 {
   
    if let Some(&count) = memo.get(current_device) {
        return count;
    }

   
    if current_device == target_device {
        return 1;
    }

    
    let mut total_paths = 0;
    
    
    if let Some(outputs) = graph.get(current_device) {
        for next_device in outputs {// Recursively call for each next device and add the results.
            total_paths += count_paths_recursive(
                graph,
                next_device,
                target_device,
                memo,
            );
        }
    }
    
   
    memo.insert(current_device.to_string(), total_paths);

    total_paths
}



fn solve_path_count(input: &str, start_device: &str, target_device: &str) -> u64 {
    let graph = parse_input(input);
    let mut memo = HashMap::new();

    
    count_paths_recursive(&graph, start_device, target_device, &mut memo)
}


fn main() -> Result<(), io::Error> {
   
    const FILENAME: &str = "input_puzzle11.txt";

   

    let puzzle_input = fs::read_to_string(FILENAME)?;

  
    const START_DEVICE: &str = "you";
    const TARGET_DEVICE: &str = "out";

    let final_answer = solve_path_count(&puzzle_input, START_DEVICE, TARGET_DEVICE);

   
    println!(
        "The total number of paths from '{}' to '{}' is: {}", 
        START_DEVICE, 
        TARGET_DEVICE, 
        final_answer
    );
    
    Ok(())
}